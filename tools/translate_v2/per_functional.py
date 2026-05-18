#!/usr/bin/env python3
"""Per-functional subcrate emission orchestration shared by the three
translators (Phase 11 D-04, D-10).

`emit_functional(adapter, func_name, c_file, is_vxc_only, split_threshold)` is
the family-agnostic driver: it loops (level, spin), runs each output's
single-file-vs-split decision, builds the q02 nested-by-output `<output>/mod.rs`
wrapper-of-parts, and CSE-subdivides a single output that is STILL over the
line cap after the per-output-component cut.

Each translator hands over a `FamilyAdapter` carrying its family-specific
callables/data — `translate_lda_v2` / `translate_gga` / `translate_mgga` all
expose compatible primitives (`gen_fn`, `split_by_output`, `merge_small`,
`parse`, `build_dep_graph`, `transitive_deps`, ...). The translator-side glue
is the thin `emit_per_functional` wrapper in each translator module, and
`maple_to_kernels.py translate` calls those wrappers directly (replacing the
stale `regen_phase09.py` in-place-replacement pipeline).

D-02 ABI RISK (documented for plan 11-03's STEP-3 CHECKPOINT):
The Wave-0 spike proved only `#[cube] fn f<F: Float>(x: F, y: F) -> (F, F)`.
Real translator output for an over-cap single output references *ambient*
identifiers — `rho0`/`rho1`/`sigma*`/`lapl*`/`tau*` (pol loads), `rho[ip]`-style
indexing (unpol), `param_*`, and the `f64` `dens_threshold`/`zeta_threshold`.
A pure scalar-`F` chunk cannot see those. `emit_cse_chunked_output` threads
every ambient identifier through as an explicit chunk `F` argument as a
best-effort design — but the mixed-`F`/`f64` and array-indexing cases are NOT
spike-validated. If 11-03's `audit_kernel_size.py --strict` still trips or a
CSE-chunked subcrate fails to compile, that is the documented 11-02 <-> 11-03
retune loop, not a silent failure.

Build env source of truth: .cargo/config.toml (do not duplicate values here).
"""

from dataclasses import dataclass, field
from pathlib import Path
import re
import sys

# Allow standalone `python3 tools/translate_v2/per_functional.py --selftest`:
# put tools/ on the path so the `translate_v2` package resolves. When imported
# by a translator, tools/ is already on sys.path (the translators insert it).
sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from translate_v2 import cse
from translate_v2 import emit
from translate_v2.cse import (PositionContext,
                              collect_named_const_uses,
                              HOIST_PRELUDE_TEMPLATE)
from translate_v2.helpers_allowlist import (NAMED_CONSTS,
                                            is_generic_helper_call)

# Pol-spin input loads per family (unpol uses `rho[ip]`-style indexing instead).
_POL_LOADS = {
    "lda": ["rho0", "rho1"],
    "gga": ["rho0", "rho1", "sigma0", "sigma1", "sigma2"],
    "mgga": ["rho0", "rho1", "sigma0", "sigma1", "sigma2",
             "lapl0", "lapl1", "tau0", "tau1"],
}
_INPUT_ARRAYS = {
    "lda": ["rho"],
    "gga": ["rho", "sigma"],
    "mgga": ["rho", "sigma", "lapl", "tau"],
}
_LOAD_STRIDE = {"rho": {"lda": 2, "gga": 2, "mgga": 2},
                "sigma": {"gga": 3, "mgga": 3},
                "lapl": {"mgga": 2}, "tau": {"mgga": 2}}
_IDENT_RE = re.compile(r'\b([A-Za-z_]\w*)\b')
_ASSIGN_RE = re.compile(r'^\s*(\w+)\s*=\s*(.*)$')

# CubeCL 0.10 fix #2 — chunk bodies emit `<F: Float>` generic code; raw f64
# literals do NOT auto-coerce to F under `#[cube]` (E0277 "Mul<F> for {float}").
# This regex matches every f64 literal token in a translated Rust expression so
# we can wrap each as `F::new(<lit>)`. Spec:
#   * mandatory decimal point — pure-integer literals (`0`, `2`) used as
#     u32/usize args or as identifier suffixes MUST NOT be wrapped, and they
#     have no dot.
#   * optional exponent — maple2c emits `0.21687e-1` style scientific notation.
#   * the `(?<![A-Za-z_\d.])` / `(?![A-Za-z_\d.])` manual boundaries refuse to
#     bite into an identifier like `param_cx_0` and refuse to match the trailing
#     exponent of `0.5e2` as a second literal.
# Validated 2026-05-15 against the spike file `crates/kernels/math/tests/
# spike_cse_emit_q01.rs` (Q4a/b/c) and the q01 canary set.
_F64_LITERAL_RE = re.compile(
    r'(?<![A-Za-z_\d.])(\d+\.\d+(?:[eE][+-]?\d+)?)(?![A-Za-z_\d.])'
)

# D-06 long-literal precision-split cutoff. Literals with > this many
# significant digits get F::cast_from(<lit>_f64) so the source f64 bit-pattern
# is preserved verbatim when compiled under f64 mode (f32 mode accepts the
# narrowing cost per the 1e-6 tolerance in D-19a).
_LONG_LITERAL_SIGNIFICANT_DIGIT_THRESHOLD = 8

# D-10 chunk-to-chunk call sites (inside a generic chunk body) propagate the
# caller's F via ::<F>. The naming convention from `_cse_chunk_part` produces
# `<func>_<level>_<spin>_part<N>_<suffix>_chunk<idx>` — the `_chunk\d+` tail
# is the discriminator.
_CHUNK_TO_CHUNK_CALL_RE = re.compile(r'\b([a-z_][a-z_0-9]*_chunk\d+)\(')

# D-08 math/ helper call sites. Negative lookbehind on `::` keeps the regex
# from biting into a fully-qualified `f64::ln(` or an already-substituted
# `pow_1_3::<F>(`. The identifier shape is the same as a Rust snake_case fn.
_HELPER_CALL_RE = re.compile(r'(?<!::)\b([a-z_][a-z_0-9]*)\(')


def _significant_digit_count(lit: str) -> int:
    """Count significant digits in an f64 literal string (decimal-form input).

    `lit` is the raw match from `_F64_LITERAL_RE` — always contains a `.` and
    may carry an exponent. Strip the decimal point, any leading sign/zeros, and
    the exponent suffix, then return the remaining digit count.
    """
    digits = lit.replace('.', '').lstrip('-+0').split('e')[0].split('E')[0]
    return len(digits)


def _wrap_f64_literals_v2(rust_expr: str,
                           ctx: PositionContext,
                           hoisted: dict) -> str:
    """D-04..D-10 chunk-body emit pass.

    Steps:
      A. Rule-4 guard — doc-comments and string-literal spans return verbatim.
      B. D-05 hoisted named-const substitution — replace each NAMED_CONSTS
         key occurrence with its hoisted local-binding name.
      C. D-06 f64-literal wrap — `F::new(<lit>)` for short literals, or
         `F::cast_from(<lit>_f64)` for long (> 8 sig digit) literals to
         preserve the source f64 bit pattern under f64 mode.
      D. D-08 native helper turbofish — `helper::<F>(` for every D-09
         allowlist member at call sites.
      E. D-10 chunk-to-chunk turbofish — `..._chunk<idx>::<F>(` for
         chunk-to-chunk calls inside generic chunk bodies. The wrapper→chunk
         call site (in `_build_chunk_wrapper`) STAYS `::<f64>` because the
         wrapper itself is concrete-f64 (Array<f64> buffer-typed I/O); see
         the inline nuance comment at that emit site.
    """
    # Step A — Rule-4 hands-off.
    if ctx.in_doc_comment or ctx.in_string_literal:
        return rust_expr

    out = rust_expr

    # Step B — hoisted named-const substitution. Iteration is over the
    # caller-supplied `hoisted` map (ordered insertion), so substitutions are
    # deterministic. `re.escape` is mandatory because `f64::MAX` contains `:`.
    for sym, local_name in hoisted.items():
        out = re.sub(rf'\b{re.escape(sym)}\b', local_name, out)

    # Step C — f64-literal wrap with Rule-2 / Rule-3 length split.
    def _wrap_literal(m):
        lit = m.group(1)
        if _significant_digit_count(lit) > _LONG_LITERAL_SIGNIFICANT_DIGIT_THRESHOLD:
            return f"F::cast_from({lit}_f64)"
        return f"F::new({lit})"
    out = _F64_LITERAL_RE.sub(_wrap_literal, out)

    # Step D — D-08 native math/ helper turbofish via D-09 allowlist.
    def _wrap_helper(m):
        name = m.group(1)
        if is_generic_helper_call(name):
            return f"{name}::<F>("
        return m.group(0)
    out = _HELPER_CALL_RE.sub(_wrap_helper, out)

    # Step E — D-10 chunk-to-chunk turbofish (caller and callee both <F: Float>
    # inside the chunk body). Applied AFTER the helper pass so a chunk fn name
    # never collides with an allowlist entry (chunk fns end in `_chunk\d+`,
    # which is not present in any allowlist member).
    out = _CHUNK_TO_CHUNK_CALL_RE.sub(lambda m: f"{m.group(1)}::<F>(", out)
    return out


def _wrap_f64_literals(rust_expr: str) -> str:
    """Backwards-compat shim — pre-Phase-11.1 callers used the no-context form.

    Defaults to a let-rhs/F enclosing context with an empty hoist map; this
    keeps the legacy "wrap short literal as F::new" behaviour intact for any
    external smoke test that imports the old name. New emit-path callers
    should use :func:`_wrap_f64_literals_v2` directly with their own
    :class:`PositionContext`.
    """
    default_ctx = PositionContext(
        parent_kind="let_rhs",
        enclosing_return_type="F",
        enclosing_let_type="F",
        in_doc_comment=False,
        in_string_literal=False,
    )
    return _wrap_f64_literals_v2(rust_expr, default_ctx, {})


@dataclass
class FamilyAdapter:
    """Family-specific callables/data the translator hands to this module.

    All callables mirror the existing translator primitives; the adapter only
    *normalises* the small signature differences between the three translators
    (e.g. LDA `build_dependency_graph` returns a 2-tuple and takes no `is_pol`;
    GGA/MGGA return a 3-tuple — `build_dep_graph` here is always
    `(compute_lines, is_pol) -> (var_order, var_deps)`).
    """
    family: str                       # "lda" | "gga" | "mgga"
    file_header: str                  # the #![allow(...)] file header line
    src_dir_label: str                # e.g. "gga_exc" — for file doc-comments
    is_routed: bool                   # cached_routed_funcnames membership
    needs_libm: bool                  # whether this functional pulls in libm
    imports_str: str                  # `use cubecl::prelude::*;` + math primitives
    all_params: object                # scan_params result (opaque, passed through)
    bodies: dict                      # {(level, spin): body_text}
    max_order: int
    level_ord: dict                   # level -> int
    level_outputs: dict               # level -> [output buffer fields]
    levels: list                      # the level list for this functional
    translate_line: object            # fn(expr:str, is_pol:bool) -> rust expr str
    parse: object                     # fn(body, level, spin, is_vxc_only) -> (compute, outs)
    estimate: object                  # fn(compute, outs) -> int
    gen_fn: object                    # the family generate_function
    split_by_output: object           # fn(compute, outs, is_pol) -> splits
    merge_small: object               # fn(splits, threshold) -> splits
    build_dep_graph: object           # fn(compute, is_pol) -> (var_order, var_deps)
    transitive_deps: object           # fn(vars, var_deps) -> set
    ow_var: object                    # OutputWrite/tuple -> var name
    ow_field: object                  # -> output field
    ow_component: object              # -> component index
    pol_dims: dict = field(default_factory=dict)


def _parse_assign(line):
    s = line.rstrip(';').strip()
    m = _ASSIGN_RE.match(s)
    return (m.group(1), m.group(2)) if m else (None, None)


def _assemble_file(adapter, func_name, level, spin, fn_code, *, part=None):
    """Wrap a generate_function body in the standard file header + imports."""
    head = [f"//! {func_name.upper()} {level} {spin} kernel."]
    if part is not None:
        idx, total, suffix, bufs = part
        head[0] = (f"//! {func_name.upper()} {level} {spin} kernel — "
                   f"split part {idx}/{total} ({suffix}).")
        head.append(f"//! Split sub-kernel: outputs [{', '.join(bufs)}].")
    head.append("//!")
    head.append(f"//! Auto-translated from "
                f"`libxc-master/src/maple2c/{adapter.src_dir_label}/{func_name}.c`.")
    head.append("//! Preserves exact maple2c variable names and FP operation order.")
    return ("\n".join(head) + "\n\n" + adapter.file_header + "\n\n"
            + adapter.imports_str + "\n\n" + fn_code + "\n")


def _extract_signature(fn_code):
    """Split a generated `#[...]\\n#[cube...]\\npub fn NAME(\\n  args\\n) {` block
    into (header_through_open_brace, [arg_names])."""
    brace = fn_code.find(") {")
    if brace < 0:
        return fn_code, []
    header = fn_code[:brace + 3]
    args = []
    for ln in header.splitlines():
        m = re.match(r'\s+(\w+)\s*:\s*', ln)
        if m:
            args.append(m.group(1))
    return header, args


def _build_part_wrapper(adapter, func_name, level, spin, output,
                        full_fn_code, part_fn_codes):
    """The `<output>/mod.rs` wrapper for a per-component-split output: the
    canonical entry signature (from the unsuffixed generate_function output)
    with a body that just calls each partN with its own arg subset. Each part
    does its own ABSOLUTE_POS guard, so the wrapper carries no guard (q02 shape).
    """
    header, _wrapper_args = _extract_signature(full_fn_code)
    n = len(part_fn_codes)
    lines = [
        f"//! {func_name.upper()} {level} {spin} kernel — {output} "
        f"(nested-by-output, {n} parts).",
        adapter.file_header,
        "",
    ]
    for i in range(n):
        lines.append(f"mod part{i};")
    lines.append("")
    lines.append(adapter.imports_str)
    lines.append("")
    part_calls = []
    for i, pcode in enumerate(part_fn_codes):
        _ph, pargs = _extract_signature(pcode)
        m = re.search(r'pub fn (\w+)\s*\(', pcode)
        pfn = m.group(1) if m else f"{func_name}_{level}_{spin}_part{i}"
        lines.append(f"use part{i}::{pfn};")
        part_calls.append(f"    {pfn}({', '.join(pargs)});")
    lines.append("")
    lines.append(header)
    lines.extend(part_calls)
    lines.append("}")
    return "\n".join(lines) + "\n"


def _cse_chunk_part(adapter, func_name, level, spin, output, part_idx, suffix,
                    compute_lines, output_writes, part_out_bufs,
                    split_threshold):
    """CSE-subdivide a single over-cap output-component PART into D-02
    tuple-return ``<F: Float>`` chunks.

    Returns ``(part_wrapper_src, [chunk_src, ...])`` for
    ``emit.emit_output_dir`` (the part becomes a ``<output>/part{idx}/``
    directory — the third nesting level), or ``None`` if the CSE pass could
    not subdivide it (<=1 chunk), in which case the caller falls back to a
    flat (still-oversized) part and logs the miss as an 11-02 <-> 11-03 retune
    signal.

    Best-effort ambient-input threading (pol loads + dens/zeta thresholds) —
    see the D-02 ABI RISK note at the top of this module; ``param_*`` threading
    is not yet covered and may surface as a STEP-5 / 11-04 compile error.
    """
    is_pol = (spin == "pol")
    _vo, var_deps = adapter.build_dep_graph(compute_lines, is_pol)
    est = lambda ls: len(ls) + 25
    chunks = cse.partition_compute_lines(compute_lines, var_deps, est,
                                         chunk_max_lines=split_threshold)
    if len(chunks) <= 1:
        return None

    defined_all = {v for v in (_parse_assign(l)[0] for l in compute_lines) if v}
    pol_loads = _POL_LOADS.get(adapter.family, []) if is_pol else []
    ambient_pool = set(pol_loads) | {"dens_threshold", "zeta_threshold"}
    ow_vars = [adapter.ow_var(ow) for ow in output_writes]

    chunk_srcs = []
    wrapper_calls = []
    for ch in chunks:
        chunk_fn = (f"{func_name}_{level}_{spin}_part{part_idx}_{suffix}"
                    f"_chunk{ch.index}")

        # --- Pass 1: translate every line once, accumulating referenced ids
        # and a (var, rust_expr) list for the body emit pass. Storing the
        # translated RHS up front lets us:
        #   * compute chunk_defined / out_vars / ret BEFORE the body emit (so
        #     the PositionContext can carry the enclosing return type for the
        #     D-04 / D-07 classifier);
        #   * build a joined body preview that collect_named_const_uses can
        #     scan in one pass for D-05 hoisting (Phase 11.1 D-04/D-05).
        translated = []   # list of (var, rust_expr) for body emit, or None
        referenced = set()
        for ln in ch.lines:
            var, expr = _parse_assign(ln)
            if var is None:
                translated.append(None)
                continue
            rust_expr = adapter.translate_line(expr, is_pol)
            referenced.update(_IDENT_RE.findall(rust_expr))
            if var in ch.inputs:
                translated.append(None)
                continue
            translated.append((var, rust_expr))

        # --- Compute chunk_defined / out_vars / ret (moved up from below the
        # body loop so the PositionContext can use `ret` as the enclosing
        # return type). The decision tree itself (single output -> scalar F,
        # else tuple-return) is unchanged.
        ambient_args = sorted(i for i in (referenced & ambient_pool)
                              if i not in defined_all)
        in_args = list(ch.inputs) + ambient_args
        chunk_defined = {v for v in (_parse_assign(l)[0] for l in ch.lines)
                         if v and v not in ch.inputs}
        out_vars = list(ch.outputs)
        for ov in ow_vars:
            if ov in chunk_defined and ov not in out_vars:
                out_vars.append(ov)
        sig_in = ", ".join(f"{a}: F" for a in in_args)
        # q01 Fix #3: single-output chunks return scalar F (CubeCL 0.10
        # macro return-type inference fails on ANY `let` inside `-> (F,)`).
        if len(out_vars) == 1:
            ret = "F"
            ret_expr = out_vars[0]
            wrapper_bind = out_vars[0]
        else:
            ret = "(" + ", ".join("F" for _ in out_vars) + ")"
            ret_expr = "(" + ", ".join(out_vars) + ")"
            wrapper_bind = ret_expr

        # --- D-05 hoisting prelude: pre-scan the joined translated body for
        # named-const references, then emit `let <local> = F::cast_from(SYM);`
        # bindings at the top of the chunk body. The same `hoisted` map is
        # passed into _wrap_f64_literals_v2 so each in-body occurrence of the
        # const becomes the local-binding identifier (substitution-on-emit).
        joined_body_preview = "\n".join(
            f"let {var} = {rust_expr};"
            for entry in translated
            if entry is not None
            for var, rust_expr in [entry]
        )
        hoist_uses = collect_named_const_uses(joined_body_preview)
        hoisted = {sym: NAMED_CONSTS[sym] for sym in hoist_uses
                   if sym in NAMED_CONSTS}

        # --- Emit prelude lines (one let-binding per hoisted const), then
        # body lines with D-04..D-10 wrapping. PositionContext carries the
        # enclosing return type so the classifier (used inside v2) can route
        # named-const-in-F vs bare-literal-in-tuple correctly. Maple-translated
        # chunk bodies are pure arithmetic — no doc-comments, no string
        # literals — so the Rule-4 guards stay False.
        body = []
        for sym, local_name in hoisted.items():
            body.append(HOIST_PRELUDE_TEMPLATE.format(name=local_name,
                                                      symbol=sym))
        ctx = PositionContext(
            parent_kind="let_rhs",
            enclosing_return_type=ret,
            enclosing_let_type="F",
            in_doc_comment=False,
            in_string_literal=False,
        )
        for entry in translated:
            if entry is None:
                continue
            var, rust_expr = entry
            rust_expr = _wrap_f64_literals_v2(rust_expr, ctx, hoisted)
            body.append(f"    let {var} = {rust_expr};")

        chunk_srcs.append(
            f"//! {func_name.upper()} {level} {spin} — {output} part "
            f"{part_idx} ({suffix}) CSE chunk {ch.index}/{len(chunks)} "
            f"(D-02 tuple-return <F: Float>).\n"
            f"{adapter.file_header}\n\n{adapter.imports_str}\n\n"
            f"#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]\n"
            f"#[cube]\n"
            f"pub fn {chunk_fn}<F: Float>({sig_in}) -> {ret} {{\n"
            + ("\n".join(body) + "\n" if body else "")
            + f"    {ret_expr}\n}}\n"
        )
        wrapper_calls.append((wrapper_bind, chunk_fn, ", ".join(in_args)))

    wrapper = _build_chunk_wrapper(adapter, func_name, level, spin, output,
                                   part_out_bufs, output_writes, wrapper_calls,
                                   is_pol, len(chunks),
                                   fn_suffix=f"_part{part_idx}_{suffix}",
                                   is_part=True)

    # If the CSE attempt cannot get its OWN output under the cap — the chunk
    # count made the wrapper itself oversized, or a chunk stayed oversized —
    # discard it and signal a flat fallback. This is the irreducibly-dense
    # single-output 4th-derivative case (kcis/kcisk/revtpss/tpssloc lxc_pol,
    # lda_c_pk09 kxc_pol, ...): the D-02 tuple-return ABI cannot subdivide
    # those (the cross-cut arity runs to thousands). A flat single-`#[cube]`
    # emit is a known quantity; the chunked form would just be a 4000+-call
    # wrapper that is itself a proc-macro fan-out problem. Those flat files
    # become documented D-LOCK-B exceptions — see tools/kernel_size_exceptions.txt.
    wrapper_lines = wrapper.count("\n") + 1
    max_chunk_lines = max((c.count("\n") + 1 for c in chunk_srcs), default=0)
    if wrapper_lines > split_threshold or max_chunk_lines > split_threshold:
        return None
    return wrapper, chunk_srcs


def _build_chunk_wrapper(adapter, func_name, level, spin, output, out_bufs,
                         output_writes, wrapper_calls, is_pol, n_chunks,
                         fn_suffix="", is_part=False):
    """`#[cube]` wrapper over a set of D-02 CSE tuple-return chunks: loads
    inputs, calls each chunk destructuring its tuple return, writes outputs.

    With ``is_part=True`` this builds a per-component PART wrapper
    (``<output>/part{idx}/mod.rs``, fn ``..._part{idx}_{suffix}``) — always
    plain ``#[cube]``, because a ``_part`` helper is never a launch entry point
    (`audit_cube_launch.sh` baseline must not move). With ``is_part=False`` it
    builds a whole-output entry wrapper and the routing verdict picks
    ``#[cube(launch_unchecked)]`` vs ``#[cube]``."""
    fn_name = f"{func_name}_{level}_{spin}{fn_suffix}"
    cube_attr = "#[cube]" if is_part else (
        "#[cube(launch_unchecked)]" if adapter.is_routed else "#[cube]")
    input_arrays = _INPUT_ARRAYS[adapter.family]
    lines = [
        f"//! {func_name.upper()} {level} {spin} kernel — {output} "
        f"(D-02 CSE-chunked, {n_chunks} chunks).",
        adapter.file_header,
        "",
    ]
    for i in range(n_chunks):
        lines.append(f"mod chunk{i};")
    lines.append("")
    lines.append(adapter.imports_str)
    lines.append("")
    for _bind, chunk_fn, _args in wrapper_calls:
        mod_i = chunk_fn.rsplit("chunk", 1)[1]
        lines.append(f"use chunk{mod_i}::{chunk_fn};")
    lines.append("")
    lines.append("#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]")
    lines.append(cube_attr)
    lines.append(f"pub fn {fn_name}(")
    for arr in input_arrays:
        lines.append(f"    {arr}: &Array<f64>,")
    for buf in out_bufs:
        lines.append(f"    {buf}: &mut Array<f64>,")
    lines.append("    dens_threshold: f64,")
    lines.append("    zeta_threshold: f64,")
    lines.append(") {")
    bounds = out_bufs[0] if out_bufs else "vrho"
    lines.append("    let ip = ABSOLUTE_POS;")
    lines.append(f"    if ip < {bounds}.len() {{")
    if is_pol:
        for ld in _POL_LOADS.get(adapter.family, []):
            base, idx = ld[:-1], int(ld[-1])
            stride = _LOAD_STRIDE.get(base, {}).get(adapter.family, 2)
            off = f" + {idx}" if idx else ""
            lines.append(f"        let {ld} = {base}[ip * {stride}{off}];")
    # D-10 NUANCE: wrapper → chunk call site STAYS `::<f64>`. The wrapper
    # itself is concrete-f64 (see the `&Array<f64>` typed input/output buffers
    # above), and Rule 9 concrete-caller form calls a generic `<F: Float>`
    # callee with `::<f64>`. Chunk-to-chunk calls INSIDE a chunk body are a
    # different position — those are handled by
    # `_wrap_f64_literals_v2`'s `_CHUNK_TO_CHUNK_CALL_RE` pass and emit
    # `::<F>` to propagate the chunk's F generic.
    for bind, chunk_fn, args in wrapper_calls:
        lines.append(f"        let {bind} = {chunk_fn}::<f64>({args});")
    for ow in output_writes:
        var, fld, comp = adapter.ow_var(ow), adapter.ow_field(ow), adapter.ow_component(ow)
        pd = adapter.pol_dims.get(fld, 1)
        if is_pol and pd > 1:
            off = "" if comp == 0 else f" + {comp}"
            lines.append(f"        {fld}[ip * {pd}{off}] += {var};")
        else:
            lines.append(f"        {fld}[ip] += {var};")
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines) + "\n"


def emit_functional(adapter, func_name, is_vxc_only, split_threshold):
    """Family-agnostic per-functional emission driver. Returns the list of
    emitted output module names. Writes the complete per-functional subcrate
    (Cargo.toml + lib.rs + per-output files) via translate_v2.emit."""
    output_modules = []
    re_assign = re.compile(r'(\w+)\s*=')

    for spin in ("unpol", "pol"):
        is_pol = (spin == "pol")
        for level in adapter.levels:
            key = (level, spin)
            if key not in adapter.bodies:
                continue
            if adapter.level_ord[level] > adapter.max_order:
                continue
            compute, outs = adapter.parse(adapter.bodies[key], level, spin,
                                          is_vxc_only)
            output = f"{level}_{spin}"
            est = adapter.estimate(compute, outs)

            if est <= split_threshold:
                fn_code = adapter.gen_fn(func_name, level, spin, compute, outs,
                                         adapter.all_params, is_vxc_only)
                emit.emit_single_output(
                    adapter.family, func_name, output,
                    _assemble_file(adapter, func_name, level, spin, fn_code))
                output_modules.append(output)
                continue

            # oversized: per-output-array cut, merge, per-component sub-split
            splits = adapter.split_by_output(compute, outs, is_pol)
            splits = adapter.merge_small(splits, split_threshold)
            final = []
            for suffix, sub_c, sub_o, sub_b in splits:
                sub_est = adapter.estimate(sub_c, sub_o)
                if sub_est > split_threshold and len(sub_o) > 1:
                    for ow in sub_o:
                        var = adapter.ow_var(ow)
                        fld = adapter.ow_field(ow)
                        comp = adapter.ow_component(ow)
                        _vo, vdeps = adapter.build_dep_graph(sub_c, is_pol)
                        ovars = {var} | set(vdeps.get(var, set()))
                        needed = adapter.transitive_deps(ovars, vdeps)
                        cc = [cl for cl in sub_c
                              if re_assign.match(cl.rstrip(';').strip())
                              and re_assign.match(cl.rstrip(';').strip()).group(1) in needed]
                        final.append((f"{fld}_{comp}", cc, [ow], [fld]))
                    final = adapter.merge_small(final, split_threshold)
                else:
                    final.append((suffix, sub_c, sub_o, sub_b))

            # nested-by-output emit: each part is either a flat per-component
            # `#[cube]` fn, or — when a single output component is STILL over
            # the cap after the per-component cut — a D-02 CSE-chunked part
            # directory (`<output>/part{idx}/mod.rs` + `chunk{n}.rs`). The old
            # `len(final) == 1` whole-output CSE special case falls out of this
            # loop naturally as a one-element `final`.
            parts = []
            part_sig_codes = []
            for idx, (suffix, sub_c, sub_o, sub_b) in enumerate(final):
                sub_est = adapter.estimate(sub_c, sub_o)
                cse_part = None
                if sub_est > split_threshold and len(sub_o) == 1:
                    cse_part = _cse_chunk_part(
                        adapter, func_name, level, spin, output, idx, suffix,
                        sub_c, sub_o, sub_b, split_threshold)
                if cse_part is not None:
                    wrapper_src, chunk_srcs = cse_part
                    parts.append({"kind": "cse", "wrapper": wrapper_src,
                                  "chunks": chunk_srcs})
                    part_sig_codes.append(wrapper_src)
                else:
                    if sub_est > split_threshold:
                        # CSE could not get this component under the cap (<=1
                        # chunk, or its own wrapper/chunks went oversized). Emit
                        # flat — an irreducibly-dense single-output component;
                        # the resulting over-cap file is a documented D-LOCK-B
                        # exception (tools/kernel_size_exceptions.txt).
                        print(f"  WARN: {adapter.family}/{func_name} {output} "
                              f"part{idx} ({suffix}) over cap (~{sub_est} est), "
                              f"not CSE-reducible — flat emit (D-LOCK-B exception)",
                              file=sys.stderr)
                    fn_suffix = f"_part{idx}_{suffix}"
                    fn_code = adapter.gen_fn(func_name, level, spin, sub_c,
                                             sub_o, adapter.all_params,
                                             is_vxc_only, fn_suffix=fn_suffix,
                                             out_bufs_override=sub_b)
                    flat_src = _assemble_file(
                        adapter, func_name, level, spin, fn_code,
                        part=(idx, len(final), suffix, sub_b))
                    parts.append({"kind": "flat", "src": flat_src})
                    part_sig_codes.append(flat_src)
            full_fn = adapter.gen_fn(func_name, level, spin, compute, outs,
                                     adapter.all_params, is_vxc_only)
            wrapper = _build_part_wrapper(adapter, func_name, level, spin,
                                          output, full_fn, part_sig_codes)
            emit.emit_output_dir(adapter.family, func_name, output,
                                 wrapper, parts)
            output_modules.append(output)

    emit.emit_cargo_toml(adapter.family, func_name, adapter.needs_libm)
    emit.emit_lib_rs(adapter.family, func_name, output_modules)
    return output_modules


# --- self-test ---------------------------------------------------------------
def _selftest():
    """Structural smoke: drive emit_functional with a synthetic single-output
    and a synthetic over-cap functional; assert the q02 nested layout."""
    import shutil
    import tempfile

    tmp = tempfile.mkdtemp(prefix="per_functional_selftest_")
    ok = True

    class OW:
        def __init__(self, var, field, component=0):
            self.var, self.field, self.component = var, field, component

    def fake_gen_fn(func, level, spin, compute, outs, params, is_vxc,
                    fn_suffix="", out_bufs_override=None):
        name = f"{func}_{level}_{spin}{fn_suffix}"
        attr = "#[cube]" if fn_suffix.startswith("_part") else "#[cube(launch_unchecked)]"
        bufs = out_bufs_override or ["zk"]
        sig = "\n".join(f"    {b}: &mut Array<f64>," for b in bufs)
        return (f"#[allow(unused_variables, non_snake_case)]\n{attr}\n"
                f"pub fn {name}(\n    rho: &Array<f64>,\n{sig}\n"
                f"    dens_threshold: f64,\n    zeta_threshold: f64,\n) {{\n"
                f"    let ip = ABSOLUTE_POS;\n    if ip < zk.len() {{ }}\n}}\n")

    def fake_split(compute, outs, is_pol):
        # split into two per-output groups
        half = len(compute) // 2
        return [("zk", compute[:half], [outs[0]], ["zk"]),
                ("vrho", compute[half:], [outs[1]], ["vrho"])]

    def fake_dep(compute, is_pol):
        vo, vd = [], {}
        for ln in compute:
            v, e = _parse_assign(ln)
            if v:
                vo.append(v)
                vd[v] = set(re.findall(r'\bt\w+\b', e or "")) - {v}
        return vo, vd

    try:
        emit.set_kernels_root(tmp)

        # --- case 1: small single-output functional ---
        small_compute = ["t1 = rho0 + rho1;", "t2 = t1 * 2.0;"]
        adapter_small = FamilyAdapter(
            family="gga", file_header="#![allow(unused_imports)]",
            src_dir_label="gga_exc", is_routed=True, needs_libm=False,
            imports_str="use cubecl::prelude::*;", all_params=[],
            bodies={("exc", "unpol"): "BODY"}, max_order=4,
            level_ord={"exc": 0, "vxc": 1}, level_outputs={"exc": ["zk"]},
            levels=["exc"], translate_line=lambda e, p: e,
            parse=lambda b, l, s, v: (small_compute, [OW("t2", "zk")]),
            estimate=lambda c, o: len(c) + 25,
            gen_fn=fake_gen_fn, split_by_output=fake_split,
            merge_small=lambda s, t: s, build_dep_graph=fake_dep,
            transitive_deps=lambda vs, vd: set(vs),
            ow_var=lambda o: o.var, ow_field=lambda o: o.field,
            ow_component=lambda o: o.component, pol_dims={"zk": 1})
        mods = emit_functional(adapter_small, "gga_x_smoke", False, 4500)
        d = emit.subcrate_dir("gga", "gga_x_smoke")
        if mods != ["exc_unpol"] or not (d / "src" / "exc_unpol.rs").exists():
            print(f"SELFTEST FAIL: small functional layout wrong: {mods}")
            ok = False
        if not (d / "Cargo.toml").exists() or not (d / "src" / "lib.rs").exists():
            print("SELFTEST FAIL: small functional missing Cargo.toml / lib.rs")
            ok = False

        # --- case 2: over-cap functional -> per-component parts + wrapper ---
        big_compute = [f"t{k} = t{k-1} + 1.0;" for k in range(2, 260)]
        big_compute.insert(0, "t1 = rho0 + rho1;")
        adapter_big = FamilyAdapter(
            family="gga", file_header="#![allow(unused_imports)]",
            src_dir_label="gga_exc", is_routed=True, needs_libm=True,
            imports_str="use cubecl::prelude::*;", all_params=[],
            bodies={("vxc", "unpol"): "BODY"}, max_order=4,
            level_ord={"vxc": 1}, level_outputs={"vxc": ["zk", "vrho"]},
            levels=["vxc"], translate_line=lambda e, p: e,
            parse=lambda b, l, s, v: (big_compute, [OW("t1", "zk"), OW("t259", "vrho")]),
            estimate=lambda c, o: len(c) + 25,
            gen_fn=fake_gen_fn, split_by_output=fake_split,
            merge_small=lambda s, t: s, build_dep_graph=fake_dep,
            transitive_deps=lambda vs, vd: set(vs),
            ow_var=lambda o: o.var, ow_field=lambda o: o.field,
            ow_component=lambda o: o.component, pol_dims={"zk": 1, "vrho": 2})
        # threshold 200: the two per-component halves (~155 est each) stay
        # UNDER the cap, so this case keeps covering the flat nested-by-output
        # path (the CSE-chunked-part path is exercised by case 3 below).
        mods = emit_functional(adapter_big, "gga_x_big", False, 200)
        db = emit.subcrate_dir("gga", "gga_x_big")
        wrap = db / "src" / "vxc_unpol" / "mod.rs"
        if not wrap.exists() or "mod part0;" not in wrap.read_text():
            print("SELFTEST FAIL: over-cap functional missing nested wrapper")
            ok = False
        if not (db / "src" / "vxc_unpol" / "part0.rs").exists():
            print("SELFTEST FAIL: over-cap functional missing flat part files")
            ok = False
        if (db / "src" / "vxc_unpol" / "part0").is_dir():
            print("SELFTEST FAIL: in-cap part0 wrongly emitted as a directory")
            ok = False
        cargo = (db / "Cargo.toml").read_text()
        if 'libm = "0.2"' not in cargo:
            print("SELFTEST FAIL: needs_libm not propagated to Cargo.toml")
            ok = False

        # --- case 3: single output component STILL over cap after the
        # per-component cut -> D-02 CSE-chunked part directory (third level) ---
        cse_compute = ["t1 = rho0 + rho1;"]
        cse_compute += [f"t{k} = t{k-1} + 1.0;" for k in range(2, 600)]
        adapter_cse = FamilyAdapter(
            family="mgga", file_header="#![allow(unused_imports)]",
            src_dir_label="mgga_exc", is_routed=True, needs_libm=False,
            imports_str="use cubecl::prelude::*;", all_params=[],
            bodies={("lxc", "pol"): "BODY"}, max_order=4,
            level_ord={"lxc": 3}, level_outputs={"lxc": ["v4rho4"]},
            levels=["lxc"], translate_line=lambda e, p: e,
            parse=lambda b, l, s, v: (cse_compute, [OW("t599", "v4rho4")]),
            estimate=lambda c, o: len(c) + 25,
            gen_fn=fake_gen_fn,
            split_by_output=lambda c, o, p: [("v4rho4_0", c, o, ["v4rho4"])],
            merge_small=lambda s, t: s, build_dep_graph=fake_dep,
            transitive_deps=lambda vs, vd: set(vs),
            ow_var=lambda o: o.var, ow_field=lambda o: o.field,
            ow_component=lambda o: o.component, pol_dims={"v4rho4": 1})
        emit_functional(adapter_cse, "mgga_c_cse", False, 120)
        dc = emit.subcrate_dir("mgga", "mgga_c_cse")
        pdir = dc / "src" / "lxc_pol" / "part0"
        if not (dc / "src" / "lxc_pol" / "mod.rs").exists():
            print("SELFTEST FAIL: CSE functional missing output mod.rs")
            ok = False
        if not pdir.is_dir() or not (pdir / "mod.rs").exists():
            print("SELFTEST FAIL: CSE part not emitted as a part0/ directory")
            ok = False
        chunk_files = sorted(pdir.glob("chunk*.rs")) if pdir.is_dir() else []
        if len(chunk_files) < 2:
            print(f"SELFTEST FAIL: CSE part not subdivided into chunks: "
                  f"{[f.name for f in chunk_files]}")
            ok = False
        if pdir.is_dir():
            pw = (pdir / "mod.rs").read_text()
            if "mod chunk0;" not in pw or "launch_unchecked" in pw:
                print("SELFTEST FAIL: CSE part wrapper malformed "
                      "(missing mod decl, or carries a launch attr)")
                ok = False
        for f in dc.rglob("*.rs"):
            n = len(f.read_text().splitlines())
            if n > 120 + 60:  # chunk_max_lines + wrapper/header headroom
                print(f"SELFTEST FAIL: CSE output file over cap: {f} ({n} lines)")
                ok = False
    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    if ok:
        print("SELFTEST PASS: emit_functional drives single-file, flat "
              "nested-by-output, and D-02 CSE-chunked-part layouts")
        return 0
    return 1


if __name__ == "__main__":
    if "--selftest" in sys.argv:
        sys.exit(_selftest())
    print("usage: python3 tools/translate_v2/per_functional.py --selftest")
    sys.exit(2)
