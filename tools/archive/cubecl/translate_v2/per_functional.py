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
import os
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
    # Decimal-point form: `0.5`, `0.1e-3`, `137.0359996287515`
    # OR integer-mantissa exponent form: `1e-21`, `2e5` (Phase 11.1-01-fix4 —
    # Maple translations include exponent-only floats; the original regex
    # required a decimal point and missed these, surfacing as E0277
    # "cannot multiply {float} by F" in lda_c_pk09/fxc_pol/part2/chunk517.rs).
    r'(?<![A-Za-z_\d.])(\d+\.\d+(?:[eE][+-]?\d+)?|\d+[eE][+-]?\d+)(?![A-Za-z_\d.])'
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
# [A-Za-z_]: the allowlist has uppercase callables (Heaviside) — a
# lowercase-only pattern let `Heaviside::<f64>(` through un-retargeted
# inside <F: Float> chunk bodies (E0308, found on mgga_x_pbe_gx lxc_pol
# chunk-first). Non-allowlist matches are left untouched by the callback.
_HELPER_CALL_RE = re.compile(r'(?<!::)\b([A-Za-z_][A-Za-z_0-9]*)\(')

# Phase 11.1-01 fix1 — family translators (translate_lda_v2 / translate_gga /
# translate_mgga) emit `name::<f64>(...)` for math/ helper calls because the
# FLAT emission path needs concrete-f64 turbofish (the flat fn signature is
# `pub fn ..._pol(rho: &Array<f64>, ...)`, not generic). For the CHUNKED path
# the same translated line lands inside a `<F: Float>` chunk fn — calling
# `pow_1_3::<f64>(t1: F)` would fail at any F != f64. This regex picks up the
# `name::<f64>(` upstream form so the chunked emit can retarget it to
# `name::<F>(` for every allowlist member.
_FN_F64_TURBOFISH_RE = re.compile(r'\b([A-Za-z_][A-Za-z_0-9]*)::<f64>\(')

# Phase 11.1-01 fix3 — family translators map C `sqrt`/`log`/`exp`/`atan` etc.
# to `f64::sqrt(`/`f64::ln(`/`f64::exp(`/`f64::atan(`. Correct for the flat
# emission path (concrete-f64). Inside a generic `<F: Float>` chunk body,
# the same call shape fails because `f64::sqrt` expects f64 and the arg is
# F-typed. CubeCL's `Float` trait exposes associated methods (`F::sqrt`,
# `F::ln`, `F::exp`, ...) AND associated constants (`F::EPSILON`, `F::MAX`,
# ...) — retarget all `f64::IDENT` to `F::IDENT` inside chunked bodies.
_F64_QUALIFIED_RE = re.compile(r'\bf64::([A-Za-z_][A-Za-z_0-9]*)\b')

# 260520-c91: hierarchical CSE meta-grouping default fan-out (leaves per meta).
# Matches MAX_TUPLE_ARITY=12 so a meta's tuple-return arity cannot exceed the
# CubeCL ABI cap even in the worst case (every leaf in a meta produces exactly
# one cross-meta output). Tunable via LIBXC_RS_HIERARCHICAL_META_FANOUT env
# (lower values reduce per-meta body size at the cost of more metas).
META_FANOUT_DEFAULT = 12


def _meta_fanout() -> int:
    """Read the META_FANOUT env override; default to META_FANOUT_DEFAULT.

    A misformed env value (non-int, < 1) is silently coerced to the default —
    these are batch-tunable knobs, not user-typed args, and a stricter parse
    here would surface as a hard regen failure rather than a translator-side
    warning. The hierarchical branch is opt-in; if the user sets the env var
    they accept the responsibility for a sensible value.
    """
    raw = os.environ.get("LIBXC_RS_HIERARCHICAL_META_FANOUT")
    if not raw:
        return META_FANOUT_DEFAULT
    try:
        v = int(raw)
        return v if v >= 1 else META_FANOUT_DEFAULT
    except ValueError:
        return META_FANOUT_DEFAULT


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

    # Step C — f64-literal wrap. ALL literals now route through
    # `F::cast_from(<lit>_f64)`. The former short-literal `F::new(<lit>)` branch
    # (Rule 2, <= 8 sig digits) was wrong on two counts: (1) CubeCL's
    # `Float::new(val: f32)` narrows its argument to f32, so any non-exact-in-f32
    # short coefficient (e.g. `0.82785e-1`, `0.301925e0`) was silently truncated
    # to f32 precision inside the f64-concrete kernels — a latent accuracy bug;
    # (2) bare `F::new(<lit>)` trips rustc's f32-fallback lint #154024 (set to
    # become a hard error). `F::cast_from(<lit>_f64)` preserves the exact source
    # f64 bit pattern for every literal and is lint-clean. The sig-digit split is
    # retired (kept _significant_digit_count for callers / historical reference).
    def _wrap_literal(m):
        lit = m.group(1)
        return f"F::cast_from({lit}_f64)"
    out = _F64_LITERAL_RE.sub(_wrap_literal, out)

    # Step D-pre1 — retarget upstream `name::<f64>(` (emitted by family
    # translators for the flat path) to `name::<F>(` for every allowlist
    # member. The flat emission stays correct (concrete-f64 wrapper), and
    # the chunked emission now propagates F as required.
    def _retarget_f64(m):
        name = m.group(1)
        if is_generic_helper_call(name):
            return f"{name}::<F>("
        return m.group(0)
    out = _FN_F64_TURBOFISH_RE.sub(_retarget_f64, out)

    # Step D-pre2 — retarget `f64::IDENT` to `F::IDENT` (Phase 11.1-01 fix3).
    # Family translators emit `f64::sqrt(`, `f64::ln(`, `f64::EPSILON`, etc.
    # for the flat path. Chunked bodies use the Float-trait associated method
    # / constant form (`F::sqrt`, `F::ln`, `F::EPSILON`). This swap is sound
    # because the chunk body is `<F: Float>` generic and every `f64::IDENT`
    # in the family-translator math_map has a `Float::IDENT` counterpart in
    # the CubeCL `Float` trait.
    out = _F64_QUALIFIED_RE.sub(r'F::\1', out)

    # Step D — D-08 native math/ helper turbofish via D-09 allowlist.
    # Catches any bare allowlist call that did not carry an upstream `::<f64>`
    # (defensive — translator-emitted form is always `::<f64>` now).
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


@dataclass
class MetaGroup:
    """260520-c91: one meta-group in a hierarchical CSE wrapper.

    A meta-group bundles `META_FANOUT` consecutive leaf chunks under a single
    `#[cube] fn ..._meta{M}` wrapper. The meta fn takes the union of its
    leaves' inputs and returns the union of those leaf outputs that ANY
    downstream consumer (later meta OR the wrapper's `output_writes`) reads.
    Cross-meta tuple arity is bounded by 12 (MAX_TUPLE_ARITY).

    Fields:
      meta_idx     -- 0-based sequence index within the part (deterministic).
      wrapper_calls-- the slice of the 5-tuple wrapper_calls list bundled here.
      meta_inputs  -- sorted list of identifier names this meta must receive.
      meta_outputs -- ordered list of identifier names this meta must return
                       (length <= MAX_TUPLE_ARITY by construction).
      bind_decls   -- per-leaf wrapper_bind expressions (single-name or tuple)
                       so the meta body can `let r = chunkN::<F>(args);` and
                       still match the leaf's signature.
    """
    meta_idx: int
    wrapper_calls: list
    meta_inputs: list
    meta_outputs: list
    bind_decls: list


def _group_leaves_into_metas(wrapper_calls, output_writes, ow_var_fn,
                              meta_fanout):
    """260520-c91: hierarchical meta-grouping pass.

    Greedy walk over `wrapper_calls` (5-tuple shape: (bind, chunk_fn, args_str,
    out_vars, in_args)) in emission order. For each leaf K compute the set of
    its outputs that are referenced by any leaf K+1..N-1's in_args OR by any
    output_writes entry — these are the "future-reads" that the meta must
    surface as a cross-meta output if K's meta closes before they are consumed.

    Returns a list of MetaGroup, or None if at any open-meta-of-size-1 the
    live-out set already exceeds 12 (the CubeCL ABI cap). In that case the
    caller falls back to the existing flat / Path A behavior.

    Algorithm:
      1. Pre-compute, for each leaf K, the set `future_reads[K]` =
         union(out_vars[K]) ∩ (union of in_args[K+1..]) ∪ set(ow_var(ow) for
         ow in output_writes).
      2. Walk leaves in order, accumulating a current meta. For each candidate
         leaf, compute the meta's live-out = union over leaves-in-meta of
         their future-reads MINUS any of those names that are consumed by a
         later leaf within the SAME meta (those don't cross the meta
         boundary). If accepting the leaf would push live-out > 12, close
         the meta first.
      3. Close the meta on size == meta_fanout OR on live-set overflow.
      4. If a 1-leaf meta already has live-out > 12, return None (fallback).

    The algorithm reuses the data already produced by the leaf chunker — no
    separate dep-graph pass needed.
    """
    n = len(wrapper_calls)
    if n == 0:
        return []

    # output_writes consumers (always live across all metas).
    ow_consumers = {ow_var_fn(ow) for ow in output_writes}

    # For each leaf K, set of its in_args (its out_vars are read on demand
    # from wrapper_calls[K][3] inside _flush / _meta_outputs_after_add).
    leaf_ins = [set(wc[4]) for wc in wrapper_calls]

    # Build a "consumed-after-K" running set so future_reads[K] is computable
    # in O(N). Walk right-to-left: future_consumers[K] = leaf_ins[K+1] ∪
    # future_consumers[K+1] ∪ ow_consumers.
    future_consumers = [set() for _ in range(n + 1)]
    future_consumers[n] = set(ow_consumers)
    for K in range(n - 1, -1, -1):
        future_consumers[K] = leaf_ins[K] | future_consumers[K + 1]
    # Now future_consumers[K+1] is "anyone reading after leaf K".

    groups = []
    cur_leaves = []          # indices K bundled into current meta

    def _flush(current):
        """Close current meta into a MetaGroup. `current` is a list of leaf
        indices. Recomputes meta_inputs / meta_outputs / bind_decls from
        wrapper_calls slices to keep the data structure self-contained."""
        slice_wcs = [wrapper_calls[K] for K in current]
        # meta_outputs = union of each leaf's outputs that are read by anyone
        # AFTER this meta (i.e. in any later meta OR ow_writes). Order is
        # determined by emission order within the meta + first appearance.
        last_K = current[-1]
        downstream = future_consumers[last_K + 1]
        meta_outputs_set = set()
        meta_outputs_ordered = []
        for K in current:
            for v in wrapper_calls[K][3]:  # out_vars in declared order
                if v in downstream and v not in meta_outputs_set:
                    meta_outputs_set.add(v)
                    meta_outputs_ordered.append(v)
        # meta_inputs = union of each leaf's in_args that is NOT produced by
        # a leaf in this same meta (those are local lets, not meta inputs).
        produced_in_meta = set()
        for K in current:
            produced_in_meta.update(wrapper_calls[K][3])
        meta_inputs_set = set()
        meta_inputs_ordered = []
        for K in current:
            for a in wrapper_calls[K][4]:  # in_args declared order
                if a not in produced_in_meta and a not in meta_inputs_set:
                    meta_inputs_set.add(a)
                    meta_inputs_ordered.append(a)
        # bind_decls: re-emit each leaf's wrapper_bind (single name or tuple
        # form) so the meta body looks identical to today's leaf wrapper body.
        bind_decls = [wrapper_calls[K][0] for K in current]
        return MetaGroup(
            meta_idx=len(groups),
            wrapper_calls=slice_wcs,
            meta_inputs=meta_inputs_ordered,
            meta_outputs=meta_outputs_ordered,
            bind_decls=bind_decls,
        )

    def _meta_outputs_after_add(current_plus_one):
        """Compute would-be meta_outputs if the meta closed RIGHT NOW with
        `current_plus_one` (a list of indices including the new candidate).
        Returns the count (used for the live-set guard)."""
        last_K = current_plus_one[-1]
        downstream = future_consumers[last_K + 1]
        seen = set()
        for K in current_plus_one:
            for v in wrapper_calls[K][3]:
                if v in downstream:
                    seen.add(v)
        return len(seen)

    for K in range(n):
        candidate = cur_leaves + [K]
        live = _meta_outputs_after_add(candidate)
        if live > 12:
            # Would overflow if we add this leaf.
            if not cur_leaves:
                # 1-leaf meta already at >12 outputs — irreducible at this
                # level. Signal fallback.
                return None
            # Close current meta, then start a new one with K.
            groups.append(_flush(cur_leaves))
            cur_leaves = [K]
            # Re-check the new 1-leaf meta's live-out:
            live_solo = _meta_outputs_after_add(cur_leaves)
            if live_solo > 12:
                return None
        else:
            cur_leaves = candidate
        if len(cur_leaves) >= meta_fanout:
            groups.append(_flush(cur_leaves))
            cur_leaves = []

    if cur_leaves:
        groups.append(_flush(cur_leaves))

    # Renumber meta_idx after the in-place mutations above (the dataclass
    # `meta_idx` was set during _flush against pre-renumber `len(groups)`,
    # which is correct — but re-assert for safety in case future edits
    # reorder).
    for i, g in enumerate(groups):
        g.meta_idx = i
    return groups


def _build_meta_fn(adapter, func_name, level, spin, fn_suffix, meta_group):
    """260520-c91: emit one `#[cube] fn ..._meta{M}<F: Float>(...)` source.

    The meta fn body mirrors today's leaf-wrapper body shape — a sequence of
    `let r = chunkN::<F>(args);` lines — but lives one tier above the leaves
    so the top wrapper sees ~N_LEAVES / META_FANOUT meta calls instead of
    N_LEAVES leaf calls. Each chunk-call uses `::<F>` (not `::<f64>`) because
    the meta fn is GENERIC `<F: Float>` (Rule 9 cross-fn turbofish: generic
    caller calling a generic callee propagates F).

    Note on F vs f64 at the meta boundary: the top wrapper still calls each
    meta with `::<f64>` (concrete-f64 → generic, Rule 10 form), so the meta
    body's F is concretized to f64 at the top boundary — identical to how
    today's top wrapper concretizes the leaves' F at the wrapper→leaf call.
    """
    meta_idx = meta_group.meta_idx
    meta_fn = f"{func_name}_{level}_{spin}{fn_suffix}_meta{meta_idx}"
    sig_in = ", ".join(f"{a}: F" for a in meta_group.meta_inputs)
    n_outs = len(meta_group.meta_outputs)
    if n_outs == 0:
        # Defensive: should not happen for a closed meta (the grouper would
        # have collapsed an empty-output meta into a sibling), but emit a
        # valid unit-return signature so a downstream regen doesn't crash.
        ret = "()"
        ret_expr = "()"
    elif n_outs == 1:
        # Single-output meta: scalar F return (CubeCL 0.10 single-element
        # tuple-return inference bug — same constraint as the leaf chunker).
        ret = "F"
        ret_expr = meta_group.meta_outputs[0]
    else:
        ret = "(" + ", ".join("F" for _ in meta_group.meta_outputs) + ")"
        ret_expr = "(" + ", ".join(meta_group.meta_outputs) + ")"

    # mod chunk{n}; declarations for the chunks consumed by this meta.
    # Note: chunk module indices are reset PER META (D-LOCK-D determinism,
    # see emit.py top-comment) — they map to the per-meta directory
    # `<output>/part{idx}/meta{M}/chunk{n}.rs`.
    mod_decls = []
    use_decls = []
    body_calls = []
    for nn, wc in enumerate(meta_group.wrapper_calls):
        bind, chunk_fn, args_str = wc[0], wc[1], wc[2]
        mod_decls.append(f"mod chunk{nn};")
        use_decls.append(f"use chunk{nn}::{chunk_fn};")
        # Rule 9 form: generic caller → generic callee uses `::<F>` so F
        # propagates from this meta down into the leaf.
        body_calls.append(f"        let {bind} = {chunk_fn}::<F>({args_str});")

    src_lines = [
        f"//! {func_name.upper()} {level} {spin} kernel — {fn_suffix} "
        f"meta{meta_idx} (260520-c91 hierarchical CSE).",
        adapter.file_header,
        "",
    ]
    src_lines.extend(mod_decls)
    src_lines.append("")
    src_lines.append(adapter.imports_str)
    src_lines.append("")
    src_lines.extend(use_decls)
    src_lines.append("")
    src_lines.append("#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]")
    src_lines.append("#[cube]")
    src_lines.append(f"pub fn {meta_fn}<F: Float>({sig_in}) -> {ret} {{")
    src_lines.extend(body_calls)
    src_lines.append(f"    {ret_expr}")
    src_lines.append("}")
    return meta_fn, "\n".join(src_lines) + "\n"


def _build_hier_wrapper(adapter, func_name, level, spin, output, out_bufs,
                         output_writes, meta_groups, is_pol, fn_suffix,
                         params=None):
    """260520-c91: top-level part wrapper that calls metas instead of leaves.

    Structurally identical to `_build_chunk_wrapper` (same header / imports /
    ABSOLUTE_POS / ip < buf.len() guard / param_* threading / output_writes
    emission), only the inner-body call shape changes:

      Today (leaf wrapper):
        let r = chunk0::<f64>(args);     # x N leaves
        ...
        vrho[ip] += t1234;               # output_writes

      Hierarchical (this fn):
        let (a, b, c) = meta0::<f64>(args);   # x M metas
        ...
        vrho[ip] += t1234;               # output_writes (unchanged)

    The meta-call form picks scalar-F vs tuple destructure to match
    `_build_meta_fn`'s return shape (scalar for 1-output metas — matches the
    CubeCL 0.10 single-element-tuple inference workaround the leaf chunker
    already applies).

    Cross-fn turbofish: top wrapper is concrete-f64 (its `&Array<f64>` typed
    I/O fixes F = f64), so meta calls use `::<f64>` (Rule 10 form). Inside
    each meta, leaf calls use `::<F>` (Rule 9 form, handled in `_build_meta_fn`).
    """
    fn_name = f"{func_name}_{level}_{spin}{fn_suffix}"
    # is_part=True for an `_part*` helper — never a launch entry point.
    cube_attr = "#[cube]"
    input_arrays = _INPUT_ARRAYS[adapter.family]
    n_metas = len(meta_groups)
    lines = [
        f"//! {func_name.upper()} {level} {spin} kernel — {output} "
        f"(260520-c91 hierarchical CSE, {n_metas} metas).",
        adapter.file_header,
        "",
    ]
    for i in range(n_metas):
        lines.append(f"mod meta{i};")
    lines.append("")
    lines.append(adapter.imports_str)
    lines.append("")
    # Each meta module exposes its `..._meta{M}` function via the meta wrapper
    # `mod.rs`. Use the meta_fn name as recorded in the meta group's bookkeeping.
    for i, mg in enumerate(meta_groups):
        meta_fn_name = (f"{func_name}_{level}_{spin}{fn_suffix}_meta{i}")
        lines.append(f"use meta{i}::{meta_fn_name};")
    lines.append("")
    lines.append("#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]")
    lines.append(cube_attr)
    lines.append(f"pub fn {fn_name}(")
    for arr in input_arrays:
        lines.append(f"    {arr}: &Array<f64>,")
    for buf in out_bufs:
        lines.append(f"    {buf}: &mut Array<f64>,")
    for p in (params or []):
        lines.append(f"    {p}: f64,")
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
    # Meta calls — concretize F → f64 at the top→meta boundary (Rule 10).
    for i, mg in enumerate(meta_groups):
        meta_fn_name = f"{func_name}_{level}_{spin}{fn_suffix}_meta{i}"
        args_str = ", ".join(mg.meta_inputs)
        n_outs = len(mg.meta_outputs)
        if n_outs == 0:
            # Unit return (defensive — see _build_meta_fn).
            lines.append(f"        {meta_fn_name}::<f64>({args_str});")
        elif n_outs == 1:
            bind = mg.meta_outputs[0]
            lines.append(
                f"        let {bind} = {meta_fn_name}::<f64>({args_str});")
        else:
            bind = "(" + ", ".join(mg.meta_outputs) + ")"
            lines.append(
                f"        let {bind} = {meta_fn_name}::<f64>({args_str});")
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


def _cse_chunk_part(adapter, func_name, level, spin, output, part_idx, suffix,
                    compute_lines, output_writes, part_out_bufs,
                    split_threshold, *, fn_suffix=None, is_part=True,
                    entry_header=None):
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

    Chunk-first generalization (2026-08-15): with the keyword args at their
    defaults this behaves EXACTLY as before (per-component part path — the
    default-path bytes are regression-locked by whole-tree byte-parity).
    ``_chunk_first_output`` calls it with ``fn_suffix=""``, ``is_part=False``
    and ``entry_header=<canonical flat signature>`` to chunk a WHOLE output as
    one sequential pipeline: chunk fns are named ``{func}_{level}_{spin}_chunk{N}``
    and the wrapper is built by ``_build_chunk_entry_wrapper`` around the
    canonical entry header (identical ABI + routing attribute to the flat
    emit). The hierarchical branch is part-path-only (``is_part`` gated).
    """
    is_pol = (spin == "pol")
    if fn_suffix is None:
        fn_suffix = f"_part{part_idx}_{suffix}"
    part_label = (f"part {part_idx} ({suffix})" if is_part
                  else "whole-output chunk-first")
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

    # --- Phase 11.1-02-fix6: param_* threading for CSE-chunked emit.
    # Translators emit `param_field`, `param_field_i`, `param_field_i_j`,
    # `param_cx_<i>`, `param_cc_<i>` for parameter references (from
    # `params->field` / `p->field[i]` / etc. in the Maple source). Per the
    # D-02 ABI RISK note (top of file), the original CSE wrapper did NOT
    # thread these — they're not in `_POL_LOADS` nor in `dens_threshold` /
    # `zeta_threshold`. Surfaces as E0425 "cannot find value `param_beta`
    # in this scope" inside chunk bodies. Pre-scan the whole compute_lines
    # set, collect every `param_*` identifier referenced after
    # `adapter.translate_line`, and extend `ambient_pool` so the per-chunk
    # `referenced & ambient_pool` filter threads each used param as an
    # `in_args` entry. The wrapper sig (in `_build_chunk_wrapper`) also
    # needs to declare these as `f64` parameters — see `wrapper_params_*`.
    param_pool = set()
    for ln in compute_lines:
        _v, _expr = _parse_assign(ln)
        if _expr is None:
            continue
        rust_expr = adapter.translate_line(_expr, is_pol)
        param_pool.update(
            re.findall(r'\bparam_[A-Za-z_][A-Za-z0-9_]*\b', rust_expr)
        )
    ambient_pool = ambient_pool | param_pool

    chunk_srcs = []
    wrapper_calls = []
    for ch in chunks:
        chunk_fn = (f"{func_name}_{level}_{spin}{fn_suffix}"
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
            f"//! {func_name.upper()} {level} {spin} — {output} "
            f"{part_label} CSE chunk {ch.index}/{len(chunks)} "
            f"(D-02 tuple-return <F: Float>).\n"
            f"{adapter.file_header}\n\n{adapter.imports_str}\n\n"
            f"#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]\n"
            f"#[cube]\n"
            f"pub fn {chunk_fn}<F: Float>({sig_in}) -> {ret} {{\n"
            + ("\n".join(body) + "\n" if body else "")
            + f"    {ret_expr}\n}}\n"
        )
        # 260520-c91: track out_vars + in_args as structured fields so the
        # hierarchical meta-grouper (downstream) can compute cross-meta
        # live-sets from variable names directly. wrapper_bind / args strings
        # remain for the existing leaf-wrapper emit path; nothing else changes.
        wrapper_calls.append((wrapper_bind, chunk_fn, ", ".join(in_args),
                              list(out_vars), list(in_args)))

    if entry_header is not None:
        wrapper = _build_chunk_entry_wrapper(
            adapter, func_name, level, spin, output, entry_header,
            part_out_bufs, output_writes, wrapper_calls, is_pol, len(chunks))
    else:
        wrapper = _build_chunk_wrapper(adapter, func_name, level, spin, output,
                                       part_out_bufs, output_writes,
                                       wrapper_calls,
                                       is_pol, len(chunks),
                                       fn_suffix=fn_suffix,
                                       is_part=is_part,
                                       params=sorted(param_pool))

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

    # 260520-c91 hierarchical branch — env-gated, additive. Fires BEFORE the
    # Path A wrapper-cap calculation so it is reachable independent of Path
    # A's raised cap (the predecessor 260520-c91 placed it inside the cap-
    # rejection block, where Path A's raised cap swallowed exactly the
    # wrappers it was meant to subdivide — see SUMMARY for the gate defect).
    #
    # Gate: LIBXC_RS_HIERARCHICAL_CSE=1 AND wrapper_lines > split_threshold
    # (the BASE 4500 cap, NOT Path A's raised 15000). This catches every
    # wrapper that Path A would have raised the cap for, plus the cases
    # where Path A itself was insufficient.
    #
    # Behavior: bundle ~META_FANOUT (default 12) leaves under a single
    # `#[cube] fn ..._meta{M}` wrapper; the top wrapper then calls metas
    # instead of leaves. For tpssloc part21 (3221 leaves), this collapses
    # a 9699-line top wrapper into ~270 meta calls (~800 lines) plus ~270
    # meta bodies of ~13 lines each — no `#[cube] fn` body exceeds ~500
    # lines. See _group_leaves_into_metas + _build_hier_wrapper.
    #
    # Fallback: if the grouper can't keep any meta's live-out <= 12
    # (returns None) we log CSE-HIER-REJECT and drop through to the
    # existing Path A / flat-fallback chain below — no regression.
    if (os.environ.get("LIBXC_RS_HIERARCHICAL_CSE")
            and is_part
            and wrapper_lines > split_threshold):
        meta_groups = _group_leaves_into_metas(
            wrapper_calls, output_writes, adapter.ow_var, _meta_fanout())
        if meta_groups is not None:
            # Build per-meta `#[cube] fn` sources keyed by meta_idx.
            meta_sources = []
            for mg in meta_groups:
                _meta_fn_name, meta_src = _build_meta_fn(
                    adapter, func_name, level, spin,
                    fn_suffix=f"_part{part_idx}_{suffix}",
                    meta_group=mg)
                meta_sources.append({
                    "wrapper": meta_src,
                    # Chunks belonging to this meta — re-slice from
                    # the global chunk_srcs list. Order is preserved
                    # because the grouper walked wrapper_calls in
                    # emission order, and chunk_srcs / wrapper_calls
                    # were built in lock-step in the loop above.
                    "chunks": [
                        chunk_srcs[wrapper_calls.index(wc)]
                        for wc in mg.wrapper_calls
                    ],
                })
            hier_wrapper = _build_hier_wrapper(
                adapter, func_name, level, spin, output, part_out_bufs,
                output_writes, meta_groups, is_pol,
                fn_suffix=f"_part{part_idx}_{suffix}",
                params=sorted(param_pool))
            # Sentinel 3-tuple — `emit_functional` discriminates by
            # length. The metas list carries per-meta sources +
            # chunk sources for emit.emit_output_dir's `cse-hier` kind.
            return hier_wrapper, chunk_srcs, meta_sources
        else:
            # Grouper rejected (1-leaf meta with > 12 outputs). Log a
            # short signal so a future tune can find the case, then
            # drop through to the existing Path A / flat-fallback path.
            print(
                f"  CSE-HIER-REJECT {adapter.family}/{func_name} "
                f"{level}_{spin} part{part_idx} ({suffix}): "
                f"meta grouper could not keep live-out <= 12 "
                f"(falling back to flat / Path A behavior)",
                file=sys.stderr,
            )

    # 260520-a0c: decouple wrapper cap from chunk cap when explicitly enabled.
    #
    # For dense single-output 4th-derivative components (mgga_c_tpssloc lxc_pol
    # v4rho4_*/v4rho3sigma_*, also mgga_c_kcis/revtpss/rmggac lxc_pol, lda_c_pk09
    # kxc_pol), MAX_TUPLE_ARITY=12 forces the chunker to cut every ~27 lines →
    # thousands of tiny chunks → wrapper grows to 6-10K lines of `mod chunk{i};`
    # + `use chunk{i}::...;` + simple let-call plumbing. The wrapper body is
    # CHEAP to macro-expand (each statement is a plain call+bind, no nested RHS
    # arithmetic), so it doesn't drive the per-`#[cube]` RAM peak the way the
    # flat-emit fallback does (which has 10K let-bindings with deeply-nested
    # RHS expressions, OOMs at ~25 GB RSS during cubecl-macros 0.10 expansion).
    #
    # Set LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER=1 to raise the wrapper cap to a
    # higher hard limit (15000 lines, above the observed tpssloc part21
    # max of 9699L — see .planning/quick/260520-a0c-.../260520-a0c-cse-diag.log).
    # Default behavior unchanged so an unintentional full-tree regen does not
    # mass-flip the D-LOCK-B set; opt-in keeps the blast radius scoped to the
    # functional being regenerated (today: mgga_c_tpssloc).
    if os.environ.get("LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER"):
        wrapper_cap = 15000
    else:
        wrapper_cap = split_threshold
    if wrapper_lines > wrapper_cap or max_chunk_lines > split_threshold:
        if os.environ.get("LIBXC_RS_CSE_DIAG"):
            chunk_sizes = sorted(
                (c.count("\n") + 1 for c in chunk_srcs), reverse=True)
            top5 = chunk_sizes[:5]
            print(
                f"  CSE-REJECT {adapter.family}/{func_name} {level}_{spin} "
                f"part{part_idx} ({suffix}): wrapper={wrapper_lines}L "
                f"(cap={wrapper_cap}), n_chunks={len(chunks)}, "
                f"max_chunk={max_chunk_lines}L (cap={split_threshold}), "
                f"top5_chunk_lines={top5}",
                file=sys.stderr,
            )
        return None
    return wrapper, chunk_srcs


def _build_chunk_wrapper(adapter, func_name, level, spin, output, out_bufs,
                         output_writes, wrapper_calls, is_pol, n_chunks,
                         fn_suffix="", is_part=False, params=None):
    """`#[cube]` wrapper over a set of D-02 CSE tuple-return chunks: loads
    inputs, calls each chunk destructuring its tuple return, writes outputs.

    With ``is_part=True`` this builds a per-component PART wrapper
    (``<output>/part{idx}/mod.rs``, fn ``..._part{idx}_{suffix}``) — always
    plain ``#[cube]``, because a ``_part`` helper is never a launch entry point
    (`audit_cube_launch.sh` baseline must not move). With ``is_part=False`` it
    builds a whole-output entry wrapper and the routing verdict picks
    ``#[cube(launch_unchecked)]`` vs ``#[cube]``.

    ``params`` (Phase 11.1-02-fix6): sorted list of every `param_*`
    identifier referenced by any chunk in the body. Each is declared as
    `f64` in the wrapper signature, between `out_bufs` and the threshold
    params. The chunks receive them as F-typed args via the
    `ambient_pool` mechanism in `_cse_chunk_part`; the wrapper passes its
    own `param_*: f64` values through the `chunk_fn::<f64>(...)`
    concretization (D-10 nuance) so the chunk's F generic unifies with f64.
    """
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
    for wc in wrapper_calls:
        # 260520-c91: wrapper_calls may be a 3-tuple (legacy) or 5-tuple
        # (new — carries out_vars + in_args for the hier grouper). Only
        # chunk_fn is needed here, so positional indexing keeps both shapes
        # working without a version flag.
        chunk_fn = wc[1]
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
    # Phase 11.1-02-fix6: declare every param_* referenced anywhere in any
    # chunk body so the wrapper has the params in scope to thread to chunks.
    for p in (params or []):
        lines.append(f"    {p}: f64,")
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
    for wc in wrapper_calls:
        bind, chunk_fn, args = wc[0], wc[1], wc[2]
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


def _build_chunk_entry_wrapper(adapter, func_name, level, spin, output,
                               entry_header, out_bufs, output_writes,
                               wrapper_calls, is_pol, n_chunks):
    """Chunk-first ENTRY wrapper: the canonical flat entry signature
    (``entry_header`` — verbatim from ``_extract_signature`` on the unsuffixed
    ``generate_function`` output, so ABI + routing attribute are identical to
    the flat emit by construction) over a sequential D-02 chunk pipeline.

    Body shape mirrors ``_build_chunk_wrapper``: ABSOLUTE_POS guard, pol
    loads, one ``let bind = chunk{i}::<f64>(args);`` per chunk (Rule 10 —
    concrete-f64 caller into ``<F: Float>`` chunks), then the output writes.
    Unlike the per-component part layout, every value is computed exactly once
    and threaded forward — zero cross-part recomputation."""
    lines = [
        f"//! {func_name.upper()} {level} {spin} kernel — {output} "
        f"(chunk-first sequential CSE pipeline, {n_chunks} chunks).",
        adapter.file_header,
        "",
    ]
    for i in range(n_chunks):
        lines.append(f"mod chunk{i};")
    lines.append("")
    lines.append(adapter.imports_str)
    lines.append("")
    for wc in wrapper_calls:
        chunk_fn = wc[1]
        mod_i = chunk_fn.rsplit("chunk", 1)[1]
        lines.append(f"use chunk{mod_i}::{chunk_fn};")
    lines.append("")
    lines.append(entry_header)
    bounds = out_bufs[0] if out_bufs else "vrho"
    lines.append("    let ip = ABSOLUTE_POS;")
    lines.append(f"    if ip < {bounds}.len() {{")
    if is_pol:
        for ld in _POL_LOADS.get(adapter.family, []):
            base, idx = ld[:-1], int(ld[-1])
            stride = _LOAD_STRIDE.get(base, {}).get(adapter.family, 2)
            off = f" + {idx}" if idx else ""
            lines.append(f"        let {ld} = {base}[ip * {stride}{off}];")
    for wc in wrapper_calls:
        bind, chunk_fn, args = wc[0], wc[1], wc[2]
        lines.append(f"        let {bind} = {chunk_fn}::<f64>({args});")
    for ow in output_writes:
        var, fld, comp = (adapter.ow_var(ow), adapter.ow_field(ow),
                          adapter.ow_component(ow))
        pd = adapter.pol_dims.get(fld, 1)
        if is_pol and pd > 1:
            off = "" if comp == 0 else f" + {comp}"
            lines.append(f"        {fld}[ip * {pd}{off}] += {var};")
        else:
            lines.append(f"        {fld}[ip] += {var};")
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines) + "\n"


def _cse_chunk_output_struct(adapter, func_name, level, spin, output,
                             compute_lines, output_writes, out_bufs,
                             split_threshold, entry_header):
    """Chunk-first STRUCT-interface pipeline over a whole output.

    The D-02 tuple-return ABI caps chunk interfaces at 12 values, but these
    kernels' dataflow is far wider (gga_c_pbe lxc_pol: median 836 values live
    at a cut point) — tuple chunking degenerates into thousands of micro-
    chunks. Struct returns have no such ceiling: each chunk returns ONE
    generated ``#[derive(CubeType)] pub struct Chunk{i}Out<F: Float>`` whose
    fields are the chunk's crossing values (validated: a 400-field generic
    struct returned from a ``#[cube]`` fn compiles and evaluates correctly on
    the CPU runtime). The wrapper binds ``let o{i} = chunk{i}::<f64>(...)``
    and later chunks / output writes reference crossing values as
    ``o{i}.tN`` — every value is computed exactly once (the per-component
    part layout recomputes 50-92% of them, measured 1.70x runtime on
    gga_c_pbe lxc_pol).

    Returns ``(entry_wrapper_src, [chunk_src, ...])`` or ``None`` (chunker
    degenerated to <= 1 chunk, or a chunk/wrapper exceeded the cap)."""
    is_pol = (spin == "pol")
    _vo, var_deps = adapter.build_dep_graph(compute_lines, is_pol)
    est = lambda ls: len(ls) + 25
    chunks = cse.partition_compute_lines(compute_lines, var_deps, est,
                                         chunk_max_lines=split_threshold,
                                         max_tuple_arity=None,
                                         natural_breakpoints=False,
                                         max_out_arity=512)
    if len(chunks) <= 1:
        return None

    defined_all = {v for v in (_parse_assign(l)[0] for l in compute_lines) if v}
    pol_loads = _POL_LOADS.get(adapter.family, []) if is_pol else []
    ambient_pool = set(pol_loads) | {"dens_threshold", "zeta_threshold"}
    ow_vars = [adapter.ow_var(ow) for ow in output_writes]

    param_pool = set()
    for ln in compute_lines:
        _v, _expr = _parse_assign(ln)
        if _expr is None:
            continue
        rust_expr = adapter.translate_line(_expr, is_pol)
        param_pool.update(
            re.findall(r'\bparam_[A-Za-z_][A-Za-z0-9_]*\b', rust_expr)
        )
    ambient_pool = ambient_pool | param_pool

    # var -> wrapper-side reference for values crossing chunk boundaries:
    # "o{i}.tN" for struct fields, bare name for scalar single-output chunks.
    home = {}
    chunk_srcs = []
    wrapper_calls = []   # (bind_stmt_lhs, chunk_fn, mapped_args_str)
    for ch in chunks:
        chunk_fn = f"{func_name}_{level}_{spin}_chunk{ch.index}"
        struct_name = f"Chunk{ch.index}Out"

        translated = []
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
        if len(out_vars) == 1:
            ret = "F"
            ret_expr = out_vars[0]
            struct_def = ""
        else:
            ret = f"{struct_name}<F>"
            field_decls = "\n".join(f"    pub {v}: F," for v in out_vars)
            struct_def = (f"#[derive(CubeType)]\n"
                          f"pub struct {struct_name}<F: Float> {{\n"
                          f"{field_decls}\n}}\n\n")
            ret_expr = (f"{struct_name}::<F> {{ "
                        + ", ".join(f"{v}: {v}" for v in out_vars) + " }")

        joined_body_preview = "\n".join(
            f"let {var} = {rust_expr};"
            for entry in translated
            if entry is not None
            for var, rust_expr in [entry]
        )
        hoist_uses = collect_named_const_uses(joined_body_preview)
        hoisted = {sym: NAMED_CONSTS[sym] for sym in hoist_uses
                   if sym in NAMED_CONSTS}

        body = []
        for sym, local_name in hoisted.items():
            body.append(HOIST_PRELUDE_TEMPLATE.format(name=local_name,
                                                      symbol=sym))
        ctx = PositionContext(
            parent_kind="let_rhs",
            enclosing_return_type="F",
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
            f"//! {func_name.upper()} {level} {spin} — {output} chunk-first "
            f"struct-interface chunk {ch.index}/{len(chunks)}.\n"
            f"{adapter.file_header}\n\n{adapter.imports_str}\n\n"
            f"{struct_def}"
            f"#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]\n"
            f"#[cube]\n"
            f"pub fn {chunk_fn}<F: Float>({sig_in}) -> {ret} {{\n"
            + ("\n".join(body) + "\n" if body else "")
            + f"    {ret_expr}\n}}\n"
        )

        mapped_args = ", ".join(home.get(a, a) for a in in_args)
        if len(out_vars) == 1:
            bind = out_vars[0]
            # scalar output stays a bare wrapper local
        else:
            bind = f"o{ch.index}"
            for v in out_vars:
                home[v] = f"{bind}.{v}"
        wrapper_calls.append((bind, chunk_fn, mapped_args))

    # --- entry wrapper (canonical flat header + guard + loads + pipeline) ----
    lines = [
        f"//! {func_name.upper()} {level} {spin} kernel — {output} "
        f"(chunk-first struct-interface pipeline, {len(chunks)} chunks).",
        adapter.file_header,
        "",
    ]
    for i in range(len(chunks)):
        lines.append(f"mod chunk{i};")
    lines.append("")
    lines.append(adapter.imports_str)
    lines.append("")
    for _bind, chunk_fn, _args in wrapper_calls:
        mod_i = chunk_fn.rsplit("chunk", 1)[1]
        lines.append(f"use chunk{mod_i}::{chunk_fn};")
    lines.append("")
    lines.append(entry_header)
    bounds = out_bufs[0] if out_bufs else "vrho"
    lines.append("    let ip = ABSOLUTE_POS;")
    lines.append(f"    if ip < {bounds}.len() {{")
    if is_pol:
        for ld in _POL_LOADS.get(adapter.family, []):
            base, idx = ld[:-1], int(ld[-1])
            stride = _LOAD_STRIDE.get(base, {}).get(adapter.family, 2)
            off = f" + {idx}" if idx else ""
            lines.append(f"        let {ld} = {base}[ip * {stride}{off}];")
    for bind, chunk_fn, args in wrapper_calls:
        lines.append(f"        let {bind} = {chunk_fn}::<f64>({args});")
    for ow in output_writes:
        var, fld, comp = (adapter.ow_var(ow), adapter.ow_field(ow),
                          adapter.ow_component(ow))
        ref = home.get(var, var)
        pd = adapter.pol_dims.get(fld, 1)
        if is_pol and pd > 1:
            off = "" if comp == 0 else f" + {comp}"
            lines.append(f"        {fld}[ip * {pd}{off}] += {ref};")
        else:
            lines.append(f"        {fld}[ip] += {ref};")
    lines.append("    }")
    lines.append("}")
    wrapper = "\n".join(lines) + "\n"

    wrapper_lines = wrapper.count("\n") + 1
    max_chunk_lines = max((c.count("\n") + 1 for c in chunk_srcs), default=0)
    # Struct defs + long signatures add real lines; allow modest headroom over
    # the split threshold for the chunk cap (the RSS driver is the fn BODY).
    if wrapper_lines > split_threshold or \
            max_chunk_lines > split_threshold + 1000:
        if os.environ.get("LIBXC_RS_CSE_DIAG"):
            print(
                f"  CHUNK-FIRST-REJECT {adapter.family}/{func_name} "
                f"{level}_{spin}: wrapper={wrapper_lines}L "
                f"n_chunks={len(chunks)} max_chunk={max_chunk_lines}L "
                f"(cap={split_threshold})",
                file=sys.stderr,
            )
        return None
    return wrapper, chunk_srcs


def _chunk_first_output(adapter, func_name, level, spin, output,
                        compute_lines, outs, split_threshold, is_vxc_only):
    """Chunk-first (2026-08-15): try to emit an over-threshold POLARIZED
    output as ONE whole-output sequential CSE chunk pipeline instead of
    independent per-component parts.

    Why: per-component parts each recompute the shared transitive dependency
    closure — measured 64.8% redundant let-defs tree-wide, 6-12x on the
    sharded monsters (memory project-variable-recomputation-analysis). A
    sequential pipeline computes every value once and threads live values
    through tuple returns (arity <= 12; fn params unbounded).

    Pol-only: unpol bodies embed ``rho[ip]``/``sigma[ip]`` element reads
    directly in expressions, which cannot cross into scalar ``<F: Float>``
    chunk fns (pol bodies read pre-loaded ``rho0``/``sigma0``... scalars that
    thread as ambient args). Unpol outputs (26 of 264 split outputs, all
    small) keep the per-component path.

    Returns ``(entry_wrapper_src, [chunk_src, ...])`` for
    ``emit.emit_chunk_pipeline_output``, or ``None`` to fall back to the
    per-component split (chunker degenerated, or a chunk/wrapper stayed over
    cap — same rejection rules as the part path)."""
    if spin != "pol":
        return None
    # Canonical ABI source of truth: the flat entry's own signature (routing
    # attribute included). Generating the flat source is string work only.
    full_fn_code = adapter.gen_fn(func_name, level, spin, compute_lines, outs,
                                  adapter.all_params, is_vxc_only)
    entry_header, args = _extract_signature(full_fn_code)
    input_arrays = set(_INPUT_ARRAYS[adapter.family])
    out_bufs = [a for a in args
                if a not in input_arrays and not a.startswith("param_")
                and a not in ("dens_threshold", "zeta_threshold")]
    return _cse_chunk_output_struct(adapter, func_name, level, spin, output,
                                    compute_lines, outs, out_bufs,
                                    split_threshold, entry_header)


def _chunk_wrapper_const(adapter, sub_c, out_bufs, is_pol):
    """Constant (non-per-chunk) line count of a ``_build_chunk_wrapper`` — used
    by ``count_functional`` to predict the CSE-REJECT (``wrapper > cap``). The
    per-chunk cost is ~3 lines (``mod`` + ``use`` + ``let``-call); this is
    everything else: header, imports, signature, input/output/pol/param/threshold
    decls, braces. Calibrated against LIBXC_RS_CSE_DIAG (pk09: 3*2254 + 26 =
    6788). Exactness barely matters — real components sit far from the
    ``n_chunks ~= (cap-const)/3`` reject boundary (tens of chunks vs thousands)."""
    n_in = len(_INPUT_ARRAYS.get(adapter.family, []))
    n_pol = len(_POL_LOADS.get(adapter.family, [])) if is_pol else 0
    n_params = len(set(re.findall(r'params->[A-Za-z_]\w*', " ".join(sub_c))))
    imports_lines = adapter.imports_str.count("\n") + 1
    return 16 + imports_lines + n_in + len(out_bufs) + n_params + 2 + n_pol


def count_functional(adapter, func_name, is_vxc_only, split_threshold):
    """Dry-run part counter — the cheap twin of ``emit_functional``.

    Returns ``(part_count, max_part_lines, max_part_irreducible)``: the number
    of ``#[cube]`` fns the real emit WOULD produce at ``split_threshold``, the
    largest part's estimated line count, and whether that largest part is a
    CSE-REJECTED (over-threshold but emitted flat) D-LOCK-B part — i.e.
    genuinely irreducible, so lowering the threshold cannot shrink it (the
    `needs-sharding` signal). It mirrors ``emit_functional``'s branch structure but
    NEVER assembles Rust source or writes files — for CSE parts it calls
    ``cse.partition_compute_lines`` directly (a cheap chunk PLAN of line lists)
    instead of ``_cse_chunk_part`` (which translates+wraps+stringifies every
    chunk). That is what makes a full-tree sweep affordable and keeps an
    exploding functional from OOM-ing the host (no thousands of source strings,
    no thousands of file writes).

    Counting convention (matches the `#[cube]` occurrences the real emit writes):
      * flat single-output (est <= threshold)         -> 1
      * oversized output                              -> 1 (output wrapper)
        + per part: flat part                         -> 1
                    CSE part (chunks > 1)             -> 1 (part wrapper) + Nchunks
                    CSE fell back to flat (chunks<=1) -> 1
    Assumes the default (non-hierarchical) CSE path, i.e. LIBXC_RS_HIERARCHICAL_CSE
    unset — the same env adaptive_split runs under. See emit_functional for the
    authoritative control flow; keep the two in sync.
    """
    re_assign = re.compile(r'(\w+)\s*=')
    chunk_est = lambda ls: len(ls) + 25      # same est as _cse_chunk_part
    parts = 0
    max_lines = 0
    max_irred = False   # is the current max_lines an over-threshold CSE-rejected part?

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
            est = adapter.estimate(compute, outs)

            if est <= split_threshold:
                parts += 1
                if est > max_lines:
                    max_lines, max_irred = est, False
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

            parts += 1   # output-level wrapper (_build_part_wrapper)
            for suffix, sub_c, sub_o, sub_b in final:
                sub_est = adapter.estimate(sub_c, sub_o)
                if sub_est > split_threshold and len(sub_o) == 1:
                    _vo, vdeps = adapter.build_dep_graph(sub_c, is_pol)
                    chunks = cse.partition_compute_lines(
                        sub_c, vdeps, chunk_est,
                        chunk_max_lines=split_threshold)
                    n_ch = len(chunks)
                    max_chunk = max((c.est_lines for c in chunks), default=0)
                    # Replicate _cse_chunk_part's CSE-REJECT -> flat (D-LOCK-B
                    # exception): <=1 chunk, OR the chunk-wrapper would exceed
                    # the cap (it is ~3 lines/chunk — `mod`/`use`/`let call` —
                    # so a component cut into thousands of tiny chunks reverts
                    # to one flat part; this is the dominant pk09/tpssloc case),
                    # OR a single chunk still exceeds the cap. Wrapper cap =
                    # split_threshold under the default env (no
                    # LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER / HIERARCHICAL_CSE).
                    wrapper_est = 3 * n_ch + _chunk_wrapper_const(
                        adapter, sub_c, sub_b, is_pol)
                    if (n_ch <= 1 or wrapper_est > split_threshold
                            or max_chunk > split_threshold):
                        parts += 1                 # CSE-rejected -> flat
                        if sub_est > max_lines:
                            max_lines, max_irred = sub_est, True   # irreducible
                    else:
                        parts += 1 + n_ch          # part wrapper + chunks
                        if max_chunk > max_lines:
                            max_lines, max_irred = max_chunk, False
                else:
                    parts += 1
                    if sub_est > max_lines:
                        max_lines, max_irred = sub_est, False

    return parts, max_lines, max_irred


def emit_functional(adapter, func_name, is_vxc_only, split_threshold):
    """Family-agnostic per-functional emission driver. Returns the list of
    emitted output module names. Writes the complete per-functional subcrate
    (Cargo.toml + lib.rs + per-output files) via translate_v2.emit.

    Phase 11.1-02-fix5: wipes <func>/src/ before any emit so stale `partN.rs`
    files from a prior regen (where the part was flat) cannot collide with
    the new regen's `partN/mod.rs` (CSE-chunked) — E0761 ambiguity.
    Discovered on gga_c_ft97 in the 11.1-02 G1 sample sweep.
    """
    emit.clean_subcrate_src(adapter.family, func_name)
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

            # Chunk-first (env-gated pilot, 2026-08-15): whole-output
            # sequential CSE pipeline — zero cross-part recomputation. On
            # rejection (unpol / degenerate chunking / over-cap wrapper or
            # chunk) fall through to the per-component path unchanged.
            if os.environ.get("LIBXC_RS_CHUNK_FIRST"):
                cf = _chunk_first_output(adapter, func_name, level, spin,
                                         output, compute, outs,
                                         split_threshold, is_vxc_only)
                if cf is not None:
                    wrapper_src, chunk_srcs = cf
                    emit.emit_chunk_pipeline_output(
                        adapter.family, func_name, output,
                        wrapper_src, chunk_srcs)
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
                    # 260520-c91: length-based discriminant — 2-tuple is the
                    # legacy flat-CSE return; 3-tuple carries the new
                    # hierarchical meta layer. No version bumping of the
                    # _cse_chunk_part signature.
                    if len(cse_part) == 3:
                        wrapper_src, _chunk_srcs, meta_sources = cse_part
                        parts.append({"kind": "cse-hier",
                                      "wrapper": wrapper_src,
                                      "metas": meta_sources})
                        part_sig_codes.append(wrapper_src)
                    else:
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
    # Incremental-sync: close the write-recording session opened by
    # clean_subcrate_src — deletes stale files this regen did not produce.
    emit.finalize_subcrate(adapter.family, func_name)
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
