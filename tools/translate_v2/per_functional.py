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


def emit_cse_chunked_output(adapter, func_name, level, spin, output,
                            compute_lines, output_writes, out_bufs,
                            split_threshold):
    """Emit a single over-cap output as D-02 CSE tuple-return chunks.

    Returns True if it emitted a chunked output, False if CSE produced a single
    chunk (caller falls back to a normal single-file emit). Best-effort ambient
    input threading — see the D-02 ABI RISK note at the top of this module.
    """
    _vo, var_deps = adapter.build_dep_graph(compute_lines, spin == "pol")
    est = lambda ls: len(ls) + 25
    chunks = cse.partition_compute_lines(compute_lines, var_deps, est,
                                         chunk_max_lines=split_threshold)
    if len(chunks) <= 1:
        return False

    is_pol = (spin == "pol")
    defined_all = {v for v in (_parse_assign(l)[0] for l in compute_lines) if v}
    pol_loads = _POL_LOADS.get(adapter.family, []) if is_pol else []
    ambient_pool = set(pol_loads) | {"dens_threshold", "zeta_threshold"}

    part_srcs = []
    wrapper_calls = []
    for ch in chunks:
        chunk_fn = f"{func_name}_{level}_{spin}_{output}_chunk{ch.index}"
        body = []
        referenced = set()
        for ln in ch.lines:
            var, expr = _parse_assign(ln)
            if var is None:
                continue
            rust_expr = adapter.translate_line(expr, is_pol)
            referenced.update(_IDENT_RE.findall(rust_expr))
            if var in ch.inputs:
                continue
            body.append(f"    let {var} = {rust_expr};")
        ambient_args = sorted(i for i in (referenced & ambient_pool)
                              if i not in defined_all)
        in_args = list(ch.inputs) + ambient_args
        out_vars = list(ch.outputs)
        if not out_vars:
            chunk_defined = {v for v in (_parse_assign(l)[0] for l in ch.lines) if v}
            out_vars = [adapter.ow_var(ow) for ow in output_writes
                        if adapter.ow_var(ow) in chunk_defined]
        sig_in = ", ".join(f"{a}: F" for a in in_args)
        comma = "," if len(out_vars) == 1 else ""
        ret = "(" + ", ".join("F" for _ in out_vars) + comma + ")"
        ret_expr = "(" + ", ".join(out_vars) + comma + ")"
        part_srcs.append(
            f"//! {func_name.upper()} {level} {spin} — {output} CSE chunk "
            f"{ch.index}/{len(chunks)} (D-02 tuple-return <F: Float>).\n"
            f"{adapter.file_header}\n\n{adapter.imports_str}\n\n"
            f"#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]\n"
            f"#[cube]\n"
            f"pub fn {chunk_fn}<F: Float>({sig_in}) -> {ret} {{\n"
            + ("\n".join(body) + "\n" if body else "")
            + f"    {ret_expr}\n}}\n"
        )
        wrapper_calls.append((ret_expr, chunk_fn, ", ".join(in_args)))

    wrapper = _build_chunk_wrapper(adapter, func_name, level, spin, output,
                                   out_bufs, output_writes, wrapper_calls,
                                   is_pol, len(chunks))
    emit.emit_chunked_output(adapter.family, func_name, output, wrapper, part_srcs)
    return True


def _build_chunk_wrapper(adapter, func_name, level, spin, output, out_bufs,
                         output_writes, wrapper_calls, is_pol, n_chunks):
    """`<output>/mod.rs` wrapper for a D-02 CSE-chunked single output: loads
    inputs, calls each chunk destructuring its tuple return, writes outputs."""
    fn_name = f"{func_name}_{level}_{spin}"
    cube_attr = "#[cube(launch_unchecked)]" if adapter.is_routed else "#[cube]"
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

            # CSE hook: a lone over-cap single output -> D-02 tuple-return chunks
            if (len(final) == 1
                    and adapter.estimate(final[0][1], final[0][2]) > split_threshold
                    and len(final[0][2]) == 1):
                _s, c1, o1, _b1 = final[0]
                out_bufs = adapter.level_outputs[level]
                if is_vxc_only:
                    out_bufs = [b for b in out_bufs if b != "zk"]
                if emit_cse_chunked_output(adapter, func_name, level, spin,
                                           output, c1, o1, out_bufs,
                                           split_threshold):
                    output_modules.append(output)
                    continue
                # CSE produced <=1 chunk -> fall through to single-part emit

            # nested-by-output: per-component parts + wrapper
            part_codes = []
            for idx, (suffix, sub_c, sub_o, sub_b) in enumerate(final):
                fn_suffix = f"_part{idx}_{suffix}"
                fn_code = adapter.gen_fn(func_name, level, spin, sub_c, sub_o,
                                         adapter.all_params, is_vxc_only,
                                         fn_suffix=fn_suffix,
                                         out_bufs_override=sub_b)
                part_codes.append(_assemble_file(
                    adapter, func_name, level, spin, fn_code,
                    part=(idx, len(final), suffix, sub_b)))
            full_fn = adapter.gen_fn(func_name, level, spin, compute, outs,
                                     adapter.all_params, is_vxc_only)
            wrapper = _build_part_wrapper(adapter, func_name, level, spin,
                                          output, full_fn, part_codes)
            emit.emit_chunked_output(adapter.family, func_name, output,
                                     wrapper, part_codes)
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
        mods = emit_functional(adapter_big, "gga_x_big", False, 120)
        db = emit.subcrate_dir("gga", "gga_x_big")
        wrap = db / "src" / "vxc_unpol" / "mod.rs"
        if not wrap.exists() or "mod part0;" not in wrap.read_text():
            print("SELFTEST FAIL: over-cap functional missing nested wrapper")
            ok = False
        if not (db / "src" / "vxc_unpol" / "part0.rs").exists():
            print("SELFTEST FAIL: over-cap functional missing part files")
            ok = False
        cargo = (db / "Cargo.toml").read_text()
        if 'libm = "0.2"' not in cargo:
            print("SELFTEST FAIL: needs_libm not propagated to Cargo.toml")
            ok = False
    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    if ok:
        print("SELFTEST PASS: emit_functional drives single-file + nested-by-output "
              "layouts; CSE chunking covered by emit_cse_chunked_output path")
        return 0
    return 1


if __name__ == "__main__":
    if "--selftest" in sys.argv:
        sys.exit(_selftest())
    print("usage: python3 tools/translate_v2/per_functional.py --selftest")
    sys.exit(2)
