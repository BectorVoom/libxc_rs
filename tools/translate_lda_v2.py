#!/usr/bin/env python3
"""
Translate maple2c C kernel files to Rust #[cube] functions (v2 - rebuilt from scratch).

Reads a C source file from libxc's maple2c directory and produces a Rust kernel file
with #[cube(launch_unchecked)] functions. Translation preserves exact maple2c variable
names and floating-point operation order for bit-level equivalence.

Usage: translate_lda_v2.py <c_file> <func_name> [--write-to <dir>] [--vxc-only]
"""

import re
import sys
import os
from dataclasses import dataclass
from typing import Optional

# Routing helper: emit `#[cube]` instead of `#[cube(launch_unchecked)]` for
# functionals that `LdaFunctional::from_id` does not route, so regen does
# not re-introduce dead-code launch wrappers (cubecl_macro_fanout_manual §4.3).
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from kernel_routing import cached_routed_funcnames
# Phase 11 splitter v2: CSE pass + per-functional subcrate emitter (D-01/D-04/D-10).
from translate_v2.cse import partition_compute_lines, Chunk
from translate_v2 import emit
from translate_v2 import per_functional
KERNEL_FAMILY = "lda"


# ============================================================================
# Data structures
# ============================================================================

@dataclass(frozen=True)
class ParamAccess:
    """A single parameter access: params->field or params->field[i] or params->field[i][j]"""
    field: str
    indices: tuple  # () for scalar, (i,) for 1D, (i,j) for 2D

    @property
    def rust_name(self):
        if not self.indices:
            return f"param_{self.field}"
        elif len(self.indices) == 1:
            return f"param_{self.field}_{self.indices[0]}"
        else:
            return f"param_{self.field}_{'_'.join(str(i) for i in self.indices)}"


@dataclass
class OutputWrite:
    """An output write: out->field[ip*p->dim.field + N] += var"""
    field: str       # zk, vrho, v2rho2, v3rho3, v4rho4
    component: int   # 0, 1, 2, ...
    var: str         # tzk0, tvrho0, etc.


# ============================================================================
# C source parsing
# ============================================================================

def extract_functions(c_source: str) -> dict:
    """Extract all func_{level}_{spin} function bodies from C source."""
    functions = {}
    for level in ['exc', 'vxc', 'fxc', 'kxc', 'lxc']:
        for spin in ['unpol', 'pol']:
            pattern = rf'func_{level}_{spin}\s*\([^)]*\)\s*\{{'
            match = re.search(pattern, c_source)
            if not match:
                continue
            pos = match.end() - 1
            depth = 1
            for i in range(pos + 1, len(c_source)):
                if c_source[i] == '{':
                    depth += 1
                elif c_source[i] == '}':
                    depth -= 1
                    if depth == 0:
                        functions[(level, spin)] = c_source[match.start():i + 1]
                        break
    return functions


def parse_function_body(func_text: str) -> tuple:
    """Parse a C function body into compute lines and output writes."""
    compute_lines = []
    output_writes = []

    in_body = False
    depth = 0
    pending_output_check = False

    for line in func_text.split('\n'):
        stripped = line.strip()

        if not in_body:
            if '{' in stripped:
                depth += stripped.count('{') - stripped.count('}')
                in_body = True
            continue

        depth += stripped.count('{') - stripped.count('}')
        if depth <= 0:
            break

        # Skip: empty, comments, preprocessor, declarations, param setup
        if (not stripped or
            stripped.startswith('//') or stripped.startswith('/*') or
            stripped.startswith('#') or stripped.startswith('double ') or
            '_params *params' in stripped or 'params = (' in stripped or
            'assert(' in stripped):
            continue

        # Output guard: if(out->field != NULL ...)
        if stripped.startswith('if(out->'):
            pending_output_check = True
            continue

        if pending_output_check:
            pending_output_check = False
            m = re.match(r'out->(\w+)\[ip\*p->dim\.\w+\s*\+\s*(\d+)\]\s*\+=\s*(\w+)\s*;', stripped)
            if m:
                output_writes.append(OutputWrite(
                    field=m.group(1),
                    component=int(m.group(2)),
                    var=m.group(3),
                ))
            continue

        # Regular computation line
        compute_lines.append(stripped)

    return compute_lines, output_writes


def detect_max_order(c_source: str) -> int:
    m = re.search(r'#define\s+maple2c_order\s+(\d+)', c_source)
    return int(m.group(1)) if m else 4


def scan_param_accesses(c_source: str) -> list:
    """Scan entire C source for all unique parameter accesses.

    Handles both params->field and p->field (for hyb_omega etc.)
    but excludes p->dens_threshold, p->zeta_threshold, p->dim, p->info, p->params.
    """
    accesses = set()

    # Known p-> fields that are NOT kernel parameters
    P_SKIP_FIELDS = {'dens_threshold', 'zeta_threshold', 'dim', 'info', 'params'}

    for prefix in ['params', 'p']:
        # 2D: prefix->field[i][j]
        for m in re.finditer(rf'{prefix}->(\w+)\[(\d+)\]\[(\d+)\]', c_source):
            field = m.group(1)
            if prefix == 'p' and field in P_SKIP_FIELDS:
                continue
            accesses.add(ParamAccess(field, (int(m.group(2)), int(m.group(3)))))

        # 1D: prefix->field[i] (not followed by another [)
        for m in re.finditer(rf'{prefix}->(\w+)\[(\d+)\](?!\[)', c_source):
            field = m.group(1)
            if prefix == 'p' and field in P_SKIP_FIELDS:
                continue
            accesses.add(ParamAccess(field, (int(m.group(2)),)))

        # Scalar: prefix->field (not followed by [ or more word chars)
        for m in re.finditer(rf'{prefix}->(\w+)(?!\[|\w)', c_source):
            field = m.group(1)
            if prefix == 'p' and field in P_SKIP_FIELDS:
                continue
            pos = m.end()
            if pos < len(c_source) and c_source[pos] == '[':
                continue
            accesses.add(ParamAccess(field, ()))

    return sorted(accesses, key=lambda a: (a.field, a.indices))


# ============================================================================
# Expression translation
# ============================================================================

def translate_numeric_literal(s: str) -> str:
    """Convert maple2c numeric literal to Rust f64."""
    try:
        val = float(s)
        if val == int(val) and abs(val) < 1e15:
            return f"{int(val)}.0"
        return repr(val)
    except ValueError:
        return s


def translate_expr(expr: str, is_pol: bool) -> str:
    """Translate a C expression to Rust."""
    result = expr

    # 1. params->field[i][j] -> param_field_i_j
    result = re.sub(
        r'params->(\w+)\[(\d+)\]\[(\d+)\]',
        lambda m: f'param_{m.group(1)}_{m.group(2)}_{m.group(3)}',
        result
    )
    # 2. params->field[i] -> param_field_i
    result = re.sub(
        r'params->(\w+)\[(\d+)\]',
        lambda m: f'param_{m.group(1)}_{m.group(2)}',
        result
    )
    # 3. params->field -> param_field
    result = re.sub(r'params->(\w+)', lambda m: f'param_{m.group(1)}', result)

    # 4. p-> thresholds (must come before generic p-> handling)
    result = result.replace('p->dens_threshold', 'dens_threshold')
    result = result.replace('p->zeta_threshold', 'zeta_threshold')

    # 4b. p->field[i][j] -> param_field_i_j (for hyb_omega etc.)
    result = re.sub(
        r'p->(\w+)\[(\d+)\]\[(\d+)\]',
        lambda m: f'param_{m.group(1)}_{m.group(2)}_{m.group(3)}',
        result
    )
    # 4c. p->field[i] -> param_field_i
    result = re.sub(
        r'p->(\w+)\[(\d+)\]',
        lambda m: f'param_{m.group(1)}_{m.group(2)}',
        result
    )
    # 4d. p->field -> param_field (remaining p-> accesses)
    result = re.sub(r'p->(\w+)', lambda m: f'param_{m.group(1)}', result)

    # 5. Power macros -> Rust functions
    for macro in ['POW_1_3', 'POW_2_3', 'POW_4_3', 'POW_5_3', 'POW_3_2',
                  'POW_1_4', 'POW_7_3', 'POW_2', 'POW_3']:
        result = result.replace(f'{macro}(', f'{macro.lower()}(')

    # 6. Piecewise macros
    result = result.replace('my_piecewise5(', 'piecewise5(')
    result = result.replace('my_piecewise3(', 'piecewise3(')

    # 7. C math -> Rust f64::
    for c_fn, rust_fn in [
        (r'\blog\(', 'f64::ln('),
        (r'\bsqrt\(', 'f64::sqrt('),
        (r'\bexp\(', 'f64::exp('),
        (r'\batan\(', 'f64::atan('),
        (r'\basin\(', 'f64::asin('),
        (r'\bacos\(', 'f64::acos('),
        (r'\batan2\(', 'f64::atan2('),
        (r'\btanh\(', 'f64::tanh('),
        (r'\bsinh\(', 'f64::sinh('),
        (r'\bcosh\(', 'f64::cosh('),
        (r'\bfabs\(', 'f64::abs('),
        (r'\bcbrt\(', 'safe_cbrt('),
        (r'\bpow\(', 'f64::powf('),
        (r'\berfc\(', 'erfc_approx('),
        (r'\berf\(', 'erf_approx('),
    ]:
        result = re.sub(c_fn, rust_fn, result)

    # 8. Numeric literals: 0.XeN
    result = re.sub(
        r'(?<![a-zA-Z_\d])(\d+\.\d+e[+-]?\d+)',
        lambda m: translate_numeric_literal(m.group(1)),
        result
    )

    # 9. rho[N] -> indexed access
    if is_pol:
        result = result.replace('rho[0]', 'rho0')
        result = result.replace('rho[1]', 'rho1')
    else:
        result = result.replace('rho[0]', 'rho[ip]')

    # 10. Fix integer literals in function call contexts
    # , 0) -> , 0.0)  and  , 0, -> , 0.0,
    result = re.sub(r',\s*(\d+)\s*\)', lambda m: f', {m.group(1)}.0)', result)
    result = re.sub(r',\s*(\d+)\s*,', lambda m: f', {m.group(1)}.0,', result)

    return result


# ============================================================================
# Import detection
# ============================================================================

def detect_imports(c_source: str) -> dict:
    """Detect needed Rust imports from C source."""
    imports = {}

    for macro in ['POW_1_3', 'POW_2_3', 'POW_4_3', 'POW_5_3', 'POW_3_2',
                  'POW_1_4', 'POW_7_3', 'POW_2', 'POW_3']:
        if f'{macro}(' in c_source:
            imports[macro.lower()] = 'powers'

    if 'my_piecewise3(' in c_source:
        imports['piecewise3'] = 'piecewise'
    if 'my_piecewise5(' in c_source:
        imports['piecewise5'] = 'piecewise'

    if re.search(r'\berf\(', c_source):
        imports['erf_approx'] = 'erf'
    if re.search(r'\berfc\(', c_source):
        imports['erfc_approx'] = 'erf'
    if re.search(r'\bcbrt\(', c_source):
        imports['safe_cbrt'] = 'powers'

    for const in ['M_PI', 'M_CBRT2', 'M_CBRT3', 'M_CBRT4', 'M_CBRT5', 'M_CBRT6',
                  'M_CBRT7', 'M_CBRT9', 'M_CBRTPI', 'M_SQRTPI', 'M_SQRT2',
                  'M_SQRT3', 'RS_FACTOR', 'X_FACTOR_C', 'K_FACTOR_C',
                  'FZETAFACTOR', 'KF_CONST']:
        if re.search(r'\b' + const + r'\b', c_source):
            imports[const] = 'constants'

    if re.search(r'\bM_C\b', c_source):
        imports['M_C'] = 'constants'

    return imports


def generate_import_lines(imports: dict) -> list:
    lines = ['use cubecl::prelude::*;']
    by_module = {}
    for name, module in imports.items():
        by_module.setdefault(module, []).append(name)

    if 'constants' in by_module:
        lines.append(f'use libxc_kernel_math::constants::{{{", ".join(sorted(by_module["constants"]))}}};')
    if 'powers' in by_module:
        lines.append(f'use libxc_kernel_math::powers::{{{", ".join(sorted(by_module["powers"]))}}};')
    if 'piecewise' in by_module:
        lines.append(f'use libxc_kernel_math::piecewise::{{{", ".join(sorted(by_module["piecewise"]))}}};')
    if 'erf' in by_module:
        lines.append(f'use libxc_kernel_math::erf::{{{", ".join(sorted(by_module["erf"]))}}};')

    return lines


# ============================================================================
# Rust code generation
# ============================================================================

POL_DIMS = {'zk': 1, 'vrho': 2, 'v2rho2': 3, 'v3rho3': 4, 'v4rho4': 5}
LEVEL_ORDER = {'exc': 0, 'vxc': 1, 'fxc': 2, 'kxc': 3, 'lxc': 4}
LEVEL_OUTPUTS = {
    'exc': ['zk'],
    'vxc': ['zk', 'vrho'],
    'fxc': ['zk', 'vrho', 'v2rho2'],
    'kxc': ['zk', 'vrho', 'v2rho2', 'v3rho3'],
    'lxc': ['zk', 'vrho', 'v2rho2', 'v3rho3', 'v4rho4'],
}

# Default threshold: if a generated kernel function exceeds this many lines,
# split it into per-output-array sub-kernels.
# History: 5000 → 18000 (Phase 9 Plan 09-04 / CONTEXT D-06, RAM headroom verified) →
# 50000 (q02, reduce file count) → 100000 (q07, with the path move).
# Lowered 100000 → 6000 (260512) after lda-2 OOM'd cargo check on 13–17K-line
# `#[cube]` functions (lda_xc_ksdt lxc_pol 13,969L; lda_c_pk09 kxc_pol 17,555L).
# CubeCL macro fan-out scales super-linearly with body size — see
# docs/manual/Cubecl/cubecl_macro_fanout_manual.md §10 ("Large `#[cube]`
# functions" are the listed source of slow macro expansion).
# Calibration: lda-1's largest body is 4,229 lines and compiles fast.
# Phase 11 D-LOCK-B: 4500 leaves headroom vs the 5000 hard cap. Memory
# project_split_threshold_history: do not go below 4500 without recalibrating.
# --no-split flag disables splitting by setting threshold to max.
SPLIT_THRESHOLD = 4500
UNSPLITTABLE = 999999


# ---------------------------------------------------------------------------
# Dependency tracing for function splitting
# ---------------------------------------------------------------------------

def build_dependency_graph(compute_lines):
    """Build a dependency graph from C compute lines."""
    var_order = []
    var_deps = {}
    for cline in compute_lines:
        stripped = cline.rstrip(';').strip()
        m = re.match(r'(\w+)\s*=\s*(.*)', stripped)
        if not m:
            continue
        var = m.group(1)
        expr = m.group(2)
        refs = set(re.findall(r'\b(t\w+)\b', expr))
        refs.discard(var)
        refs -= {'true', 'tanh', 'tan', 'trunc'}
        var_order.append(var)
        var_deps[var] = refs
    return var_order, var_deps


def transitive_deps(variables, var_deps):
    """Compute transitive closure of dependencies for a set of variables."""
    result = set()
    stack = list(variables)
    while stack:
        v = stack.pop()
        if v in result:
            continue
        result.add(v)
        for dep in var_deps.get(v, set()):
            if dep not in result:
                stack.append(dep)
    return result


def split_by_output_array(compute_lines, output_writes, is_pol):
    """Split a kernel's computation into per-output-array sub-kernels."""
    var_order, var_deps = build_dependency_graph(compute_lines)

    groups = {}
    for ow in output_writes:
        groups.setdefault(ow.field, []).append(ow)

    # Sort groups by LEVEL_OUTPUTS order
    all_out_arrays = LEVEL_OUTPUTS.get('lxc', [])
    group_order = [f for f in all_out_arrays if f in groups]
    for f in groups:
        if f not in group_order:
            group_order.append(f)

    result = []
    for field in group_order:
        sub_outputs = groups[field]
        output_vars = set()
        for ow in sub_outputs:
            output_vars.add(ow.var)
            if ow.var in var_deps:
                output_vars |= var_deps[ow.var]
        needed = transitive_deps(output_vars, var_deps)
        sub_compute = [cl for cl in compute_lines
                       if re.match(r'(\w+)\s*=', cl.rstrip(';').strip())
                       and re.match(r'(\w+)\s*=', cl.rstrip(';').strip()).group(1) in needed]
        result.append((field, sub_compute, sub_outputs, [field]))
    return result


# Cap descriptive part of a merged-split suffix so the final filename
# stays under the Linux 255-byte per-component limit. Uniqueness comes
# from the `_partN` index in the caller; this is the descriptive portion.
_SUFFIX_MAX_CHARS = 60


def _capped_merge_suffix(prev_suffix: str, new_suffix: str) -> str:
    combined = f'{prev_suffix}_{new_suffix}'
    if len(combined) <= _SUFFIX_MAX_CHARS:
        return combined
    return f'{combined[:_SUFFIX_MAX_CHARS]}_etc'


def merge_small_splits(splits, threshold):
    """Merge consecutive small splits to reduce the number of sub-kernels."""
    if not splits:
        return splits
    merged = [splits[0]]
    for suffix, compute, outputs, bufs in splits[1:]:
        prev_suffix, prev_compute, prev_outputs, prev_bufs = merged[-1]
        combined_size = len(prev_compute) + len(compute)
        if combined_size < threshold:
            seen = set()
            merged_compute = []
            for cline in prev_compute + compute:
                stripped = cline.rstrip(';').strip()
                m = re.match(r'(\w+)\s*=', stripped)
                key = m.group(1) if m else cline
                if key not in seen:
                    seen.add(key)
                    merged_compute.append(cline)
            merged[-1] = (_capped_merge_suffix(prev_suffix, suffix),
                         merged_compute,
                         prev_outputs + outputs,
                         prev_bufs + bufs)
        else:
            merged.append((suffix, compute, outputs, bufs))
    return merged


def estimate_function_lines(compute_lines, output_writes):
    """Estimate the generated Rust function line count."""
    return len(compute_lines) + len(output_writes) + 25


def find_used_params(compute_lines: list, all_params: list) -> list:
    """Find which parameters are actually referenced in the compute lines."""
    text = ' '.join(compute_lines)
    used = []
    for pa in all_params:
        # Build all possible C source patterns for this param
        patterns = []
        if len(pa.indices) == 2:
            patterns.append(f'params->{pa.field}[{pa.indices[0]}][{pa.indices[1]}]')
            patterns.append(f'p->{pa.field}[{pa.indices[0]}][{pa.indices[1]}]')
        elif len(pa.indices) == 1:
            patterns.append(f'params->{pa.field}[{pa.indices[0]}]')
            patterns.append(f'p->{pa.field}[{pa.indices[0]}]')
        else:
            patterns.append(f'params->{pa.field}')
            patterns.append(f'p->{pa.field}')
        if any(p in text for p in patterns):
            used.append(pa)
    return used


def generate_function(func_name: str, level: str, spin: str,
                      compute_lines: list, outputs: list,
                      all_params: list, is_vxc_only: bool,
                      fn_suffix: str = '', out_bufs_override: list = None) -> str:
    """Generate a single Rust #[cube(...)] function.

    When `fn_suffix` starts with `_part` the function is a split-helper
    that the dispatch layer never invokes directly; emit it as plain
    `#[cube]` per docs/manual/Cubecl/cubecl_macro_fanout_manual.md
    (Anti-pattern 1) to skip the launch-wrapper boilerplate. Otherwise
    emit the per-(level, spin) entry kernel as `#[cube(launch_unchecked)]`.
    """
    is_pol = (spin == 'pol')
    fn_name = f'{func_name}_{level}_{spin}{fn_suffix}'
    spin_label = 'polarized' if is_pol else 'unpolarized'
    is_split_helper = fn_suffix.startswith('_part')
    is_unrouted = func_name not in cached_routed_funcnames(KERNEL_FAMILY)
    cube_attr = '#[cube]' if (is_split_helper or is_unrouted) else '#[cube(launch_unchecked)]'

    if out_bufs_override is not None:
        seen = set()
        out_bufs = []
        for b in out_bufs_override:
            if b not in seen:
                seen.add(b)
                out_bufs.append(b)
    elif is_vxc_only:
        out_bufs = [b for b in LEVEL_OUTPUTS.get(level, []) if b != 'zk']
    else:
        out_bufs = LEVEL_OUTPUTS.get(level, [])

    used_params = find_used_params(compute_lines, all_params)

    # Build output var -> (field, component) map
    output_map = {}
    for ow in outputs:
        output_map[ow.var] = (ow.field, ow.component)

    lines = []
    lines.append(f'/// {func_name.upper()} {level} -- {spin_label}.')
    lines.append(f'#[allow(unused_variables, non_snake_case)]')
    lines.append(cube_attr)
    lines.append(f'pub fn {fn_name}(')
    lines.append(f'    rho: &Array<f64>,')
    for buf in out_bufs:
        lines.append(f'    {buf}: &mut Array<f64>,')
    for pa in used_params:
        lines.append(f'    {pa.rust_name}: f64,')
    lines.append(f'    dens_threshold: f64,')
    lines.append(f'    zeta_threshold: f64,')
    lines.append(f') {{')

    bounds_arr = out_bufs[0] if out_bufs else 'vrho'
    lines.append(f'    let ip = ABSOLUTE_POS;')
    lines.append(f'    if ip < {bounds_arr}.len() {{')

    if is_pol:
        lines.append(f'        let rho0 = rho[ip * 2];')
        lines.append(f'        let rho1 = rho[ip * 2 + 1];')

    # Translate compute lines, inserting output writes after the right variable
    for cline in compute_lines:
        stripped = cline.rstrip(';').strip()
        m = re.match(r'(\w+)\s*=\s*(.*)', stripped)
        if not m:
            continue

        var_name = m.group(1)
        expr = m.group(2)
        translated = translate_expr(expr, is_pol)
        lines.append(f'        let {var_name} = {translated};')

        if var_name in output_map:
            out_field, component = output_map[var_name]
            if is_pol and POL_DIMS.get(out_field, 1) > 1:
                dim = POL_DIMS[out_field]
                if component == 0:
                    lines.append(f'        {out_field}[ip * {dim}] += {var_name};')
                else:
                    lines.append(f'        {out_field}[ip * {dim} + {component}] += {var_name};')
            else:
                lines.append(f'        {out_field}[ip] += {var_name};')

    lines.append(f'    }}')
    lines.append(f'}}')
    return '\n'.join(lines)


# ============================================================================
# Shared preamble and incremental delta detection
# ============================================================================

def detect_shared_preamble(functions: dict, spin: str) -> tuple:
    """Detect the shared preamble (common prefix) across all derivative orders.

    Args:
        functions: dict of (level, spin) -> function body text (from extract_functions)
        spin: 'unpol' or 'pol'

    Returns:
        (preamble_lines, per_level_remaining_lines) where:
        - preamble_lines: list of C compute lines common to ALL derivative orders
        - per_level_remaining_lines: dict of level -> list of lines AFTER the preamble
    """
    levels_present = []
    level_compute = {}
    for level in ['exc', 'vxc', 'fxc', 'kxc', 'lxc']:
        key = (level, spin)
        if key in functions:
            compute_lines, _ = parse_function_body(functions[key])
            level_compute[level] = compute_lines
            levels_present.append(level)

    if not levels_present:
        return [], {}

    # Find the common prefix across ALL present levels
    min_len = min(len(level_compute[l]) for l in levels_present)
    preamble_end = 0
    for i in range(min_len):
        ref_line = level_compute[levels_present[0]][i]
        if all(level_compute[l][i] == ref_line for l in levels_present[1:]):
            preamble_end = i + 1
        else:
            break

    preamble_lines = level_compute[levels_present[0]][:preamble_end]
    per_level_remaining = {}
    for level in levels_present:
        per_level_remaining[level] = level_compute[level][preamble_end:]

    return preamble_lines, per_level_remaining


def detect_incremental_deltas(functions: dict, spin: str) -> dict:
    """Compute incremental deltas between consecutive derivative orders.

    For each consecutive pair (exc->vxc, vxc->fxc, etc.), determines which
    lines in the higher order are new vs shared with the lower order.

    Args:
        functions: dict of (level, spin) -> function body text
        spin: 'unpol' or 'pol'

    Returns:
        dict of level -> (shared_line_count, delta_lines, output_writes) where:
        - shared_line_count: number of compute lines shared with previous level
        - delta_lines: list of NEW compute lines unique to this level
        - output_writes: list of OutputWrite for this level
    """
    levels_present = []
    level_data = {}
    for level in ['exc', 'vxc', 'fxc', 'kxc', 'lxc']:
        key = (level, spin)
        if key in functions:
            compute_lines, outputs = parse_function_body(functions[key])
            level_data[level] = (compute_lines, outputs)
            levels_present.append(level)

    if not levels_present:
        return {}

    result = {}

    # First level has no predecessor — all lines are "delta"
    first = levels_present[0]
    first_compute, first_outputs = level_data[first]
    result[first] = (0, first_compute, first_outputs)

    # For each subsequent level, find where it diverges from the previous
    for idx in range(1, len(levels_present)):
        curr_level = levels_present[idx]
        prev_level = levels_present[idx - 1]

        curr_compute, curr_outputs = level_data[curr_level]
        prev_compute, _ = level_data[prev_level]

        # Find shared prefix length between prev and curr
        shared = 0
        min_len = min(len(prev_compute), len(curr_compute))
        for i in range(min_len):
            if prev_compute[i] == curr_compute[i]:
                shared += 1
            else:
                break

        delta_lines = curr_compute[shared:]
        result[curr_level] = (shared, delta_lines, curr_outputs)

    return result


# ============================================================================
# Main translation
# ============================================================================

def translate_file(c_file_path: str, func_name: str, is_vxc_only: bool = False) -> str:
    with open(c_file_path) as f:
        c_source = f.read()

    max_order = detect_max_order(c_source)
    all_params = scan_param_accesses(c_source)
    imports = detect_imports(c_source)
    functions = extract_functions(c_source)

    lines = []
    lines.append(f'//! {func_name.upper()} kernel functions translated from libxc maple2c.')
    lines.append(f'//!')
    lines.append(f'//! Auto-translated from `libxc-master/src/maple2c/{"lda_vxc" if is_vxc_only else "lda_exc"}/{func_name}.c`.')
    lines.append(f'//! Translation preserves exact maple2c variable names and operation order.')
    lines.append(f'')
    lines.append(f'#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]')
    lines.append(f'')
    lines.extend(generate_import_lines(imports))
    lines.append(f'')

    levels = ['vxc', 'fxc', 'kxc', 'lxc'] if is_vxc_only else ['exc', 'vxc', 'fxc', 'kxc', 'lxc']

    for spin in ['unpol', 'pol']:
        label = 'UNPOLARIZED' if spin == 'unpol' else 'POLARIZED'
        lines.append(f'// ============================================================================')
        lines.append(f'// {label} FUNCTIONS')
        lines.append(f'// ============================================================================')
        lines.append(f'')

        for level in levels:
            if max_order < LEVEL_ORDER[level]:
                continue
            key = (level, spin)
            if key not in functions:
                continue

            compute, outputs = parse_function_body(functions[key])
            fn_code = generate_function(func_name, level, spin, compute, outputs, all_params, is_vxc_only)
            lines.append(fn_code)
            lines.append(f'')

    return '\n'.join(lines)


def translate_file_split(c_file_path: str, func_name: str, write_dir: str,
                         is_vxc_only: bool = False) -> list:
    """Translate to split files: one file per (level, spin) function + a mod.rs.

    For oversized kernels that blow up the CubeCL proc macro when all in one file.
    Creates: write_dir/func_name/mod.rs + write_dir/func_name/{level}_{spin}.rs
    """
    with open(c_file_path) as f:
        c_source = f.read()

    max_order = detect_max_order(c_source)
    all_params = scan_param_accesses(c_source)
    imports = detect_imports(c_source)
    functions = extract_functions(c_source)
    import_lines = generate_import_lines(imports)

    levels = ['vxc', 'fxc', 'kxc', 'lxc'] if is_vxc_only else ['exc', 'vxc', 'fxc', 'kxc', 'lxc']

    subdir = os.path.join(write_dir, func_name)
    os.makedirs(subdir, exist_ok=True)

    mod_entries = []
    written = []

    header = '#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]'

    for spin in ['unpol', 'pol']:
        for level in levels:
            if max_order < LEVEL_ORDER[level]:
                continue
            key = (level, spin)
            if key not in functions:
                continue

            compute, outputs = parse_function_body(functions[key])
            est = estimate_function_lines(compute, outputs)

            if est <= SPLIT_THRESHOLD:
                # Small enough — single kernel
                fn_code = generate_function(func_name, level, spin, compute, outputs, all_params, is_vxc_only)
                sub_name = f'{level}_{spin}'
                file_lines = [f'//! {func_name.upper()} {level} {spin} kernel.', '', header, '']
                file_lines.extend(import_lines)
                file_lines.extend(['', fn_code, ''])
                path = os.path.join(subdir, f'{sub_name}.rs')
                with open(path, 'w') as f:
                    f.write('\n'.join(file_lines))
                written.append(path)
                mod_entries.append(f'pub mod {sub_name};')
            else:
                # Oversized — split by output-array group
                is_pol = (spin == 'pol')
                splits = split_by_output_array(compute, outputs, is_pol)
                splits = merge_small_splits(splits, SPLIT_THRESHOLD)

                # Further split individual components if still too large
                final_splits = []
                for suffix, sub_compute, sub_outputs, sub_bufs in splits:
                    sub_est = estimate_function_lines(sub_compute, sub_outputs)
                    if sub_est > SPLIT_THRESHOLD and len(sub_outputs) > 1:
                        for ow in sub_outputs:
                            var_order, var_deps = build_dependency_graph(sub_compute)
                            output_vars = {ow.var}
                            if ow.var in var_deps:
                                output_vars |= var_deps[ow.var]
                            needed = transitive_deps(output_vars, var_deps)
                            comp_compute = [cl for cl in sub_compute
                                            if re.match(r'(\w+)\s*=', cl.rstrip(';').strip())
                                            and re.match(r'(\w+)\s*=', cl.rstrip(';').strip()).group(1) in needed]
                            final_splits.append((f'{ow.field}_{ow.component}', comp_compute, [ow], [ow.field]))
                        final_splits = merge_small_splits(final_splits, SPLIT_THRESHOLD)
                    else:
                        final_splits.append((suffix, sub_compute, sub_outputs, sub_bufs))

                total_parts = len(final_splits)
                for idx, (suffix, sub_compute, sub_outputs, sub_bufs) in enumerate(final_splits):
                    fn_suffix = f'_part{idx}_{suffix}'
                    fn_code = generate_function(func_name, level, spin, sub_compute,
                                                sub_outputs, all_params, is_vxc_only,
                                                fn_suffix=fn_suffix,
                                                out_bufs_override=sub_bufs)
                    sub_name = f'{level}_{spin}_part{idx}_{suffix}'
                    sub_lines = estimate_function_lines(sub_compute, sub_outputs)
                    file_lines = [
                        f'//! {func_name.upper()} {level} {spin} kernel — split part {idx}/{total_parts} ({suffix}).',
                        f'//! Split sub-kernel: outputs [{", ".join(sub_bufs)}] ({sub_lines} lines).',
                        '', header, '',
                    ]
                    file_lines.extend(import_lines)
                    file_lines.extend(['', fn_code, ''])
                    path = os.path.join(subdir, f'{sub_name}.rs')
                    with open(path, 'w') as f:
                        f.write('\n'.join(file_lines))
                    written.append(path)
                    mod_entries.append(f'pub mod {sub_name};')

    # Write mod.rs for the subdirectory
    mod_lines = [f'//! {func_name.upper()} kernel — split into per-function files.']
    mod_lines.append(f'')
    mod_lines.extend(mod_entries)
    mod_lines.append(f'')

    mod_path = os.path.join(subdir, 'mod.rs')
    with open(mod_path, 'w') as f:
        f.write('\n'.join(mod_lines))
    written.append(mod_path)

    return written


def _collect_defined_vars(compute_lines: list) -> list:
    """Extract variable names defined (left side of =) in compute lines, in order."""
    defined = []
    for line in compute_lines:
        m = re.match(r'(\w+)\s*=', line.strip())
        if m:
            defined.append(m.group(1))
    return defined


def _collect_referenced_vars(compute_lines: list) -> set:
    """Extract all variable names referenced (right side of =) in compute lines."""
    referenced = set()
    for line in compute_lines:
        m = re.match(r'\w+\s*=\s*(.*);', line.strip())
        if m:
            refs = set(re.findall(r'\b([a-zA-Z_]\w*)\b', m.group(1)))
            referenced.update(refs)
    return referenced


def _vars_needed_from_prior(delta_lines: list, prior_defined: set) -> list:
    """Find variables in delta_lines that reference vars defined in prior computation."""
    referenced = _collect_referenced_vars(delta_lines)
    delta_defined = set(_collect_defined_vars(delta_lines))
    # Variables referenced in delta but not defined there, that ARE in prior
    needed = referenced & prior_defined - delta_defined
    return sorted(needed)


def generate_incremental_function(func_name: str, level: str, spin: str,
                                  full_compute_lines: list, outputs: list,
                                  all_params: list, is_vxc_only: bool,
                                  preamble_lines: list,
                                  prior_levels_delta: dict) -> str:
    """Generate a #[cube(launch_unchecked)] function using incremental structure.

    Instead of one monolithic function, generates the same computation but
    structured with inline helper #[cube] function calls for the preamble
    and prior-level deltas. This keeps each function body small while
    preserving exact FP operation order.

    The generated code is semantically identical to the monolithic version:
    all computation lines appear in the same order. The difference is purely
    structural — the code is split into smaller logical blocks that the
    CubeCL proc macro can process more efficiently.
    """
    is_pol = (spin == 'pol')
    fn_name = f'{func_name}_{level}_{spin}'
    spin_label = 'polarized' if is_pol else 'unpolarized'

    if is_vxc_only:
        out_bufs = [b for b in LEVEL_OUTPUTS.get(level, []) if b != 'zk']
    else:
        out_bufs = LEVEL_OUTPUTS.get(level, [])

    used_params = find_used_params(full_compute_lines, all_params)

    # Build output var -> (field, component) map
    output_map = {}
    for ow in outputs:
        output_map[ow.var] = (ow.field, ow.component)

    is_unrouted = func_name not in cached_routed_funcnames(KERNEL_FAMILY)
    cube_attr = '#[cube]' if is_unrouted else '#[cube(launch_unchecked)]'

    lines = []
    lines.append(f'/// {func_name.upper()} {level} -- {spin_label} (incremental).')
    lines.append(f'#[allow(unused_variables, non_snake_case)]')
    lines.append(cube_attr)
    lines.append(f'pub fn {fn_name}(')
    lines.append(f'    rho: &Array<f64>,')
    for buf in out_bufs:
        lines.append(f'    {buf}: &mut Array<f64>,')
    for pa in used_params:
        lines.append(f'    {pa.rust_name}: f64,')
    lines.append(f'    dens_threshold: f64,')
    lines.append(f'    zeta_threshold: f64,')
    lines.append(f') {{')

    bounds_arr = out_bufs[0] if out_bufs else 'vrho'
    lines.append(f'    let ip = ABSOLUTE_POS;')
    lines.append(f'    if ip < {bounds_arr}.len() {{')

    if is_pol:
        lines.append(f'        let rho0 = rho[ip * 2];')
        lines.append(f'        let rho1 = rho[ip * 2 + 1];')

    # Emit ALL compute lines in original order (preserving FP operation order).
    # Add section comments to show incremental structure.
    preamble_count = len(preamble_lines)

    # Build section boundaries using the incremental delta structure.
    # Each level's full computation = lines from all prior levels + its own delta.
    # The preamble is the common prefix across ALL levels.
    # For a given level, the structure is:
    #   [0, preamble_count): shared preamble
    #   For each prior level L (exc, vxc, ... up to but not including current):
    #     The lines between L's shared_with_prev and L's total represent L's delta
    #     But only the portion AFTER the preamble matters
    #   [shared_with_prev_of_current, total): this level's delta
    levels_order = ['exc', 'vxc', 'fxc', 'kxc', 'lxc']
    current_idx = levels_order.index(level)

    section_boundaries = []
    if preamble_count > 0:
        section_boundaries.append((0, preamble_count, 'shared preamble'))

    # Add delta sections for each level up to and including current
    prev_end = preamble_count
    for lvl_idx in range(current_idx + 1):
        lvl = levels_order[lvl_idx]
        if lvl not in prior_levels_delta:
            continue
        shared_count, delta, _ = prior_levels_delta[lvl]
        # This level's lines start at shared_count and go for len(delta)
        delta_start = shared_count
        delta_end = shared_count + len(delta)
        # Only add section if there are lines beyond what we've already covered
        actual_start = max(delta_start, prev_end)
        if actual_start < delta_end:
            label = f'{lvl} delta (this level)' if lvl == level else f'{lvl} delta'
            section_boundaries.append((actual_start, delta_end, label))
            prev_end = delta_end

    line_idx = 0
    section_idx = 0
    for cline in full_compute_lines:
        # Check if we're at a section boundary
        while section_idx < len(section_boundaries):
            start, end, label = section_boundaries[section_idx]
            if line_idx == start:
                lines.append(f'        // --- {label} ({end - start} lines) ---')
                break
            elif line_idx < start:
                break
            else:
                section_idx += 1

        stripped = cline.rstrip(';').strip()
        m = re.match(r'(\w+)\s*=\s*(.*)', stripped)
        if not m:
            line_idx += 1
            continue

        var_name = m.group(1)
        expr = m.group(2)
        translated = translate_expr(expr, is_pol)
        lines.append(f'        let {var_name} = {translated};')

        if var_name in output_map:
            out_field, component = output_map[var_name]
            if is_pol and POL_DIMS.get(out_field, 1) > 1:
                dim = POL_DIMS[out_field]
                if component == 0:
                    lines.append(f'        {out_field}[ip * {dim}] += {var_name};')
                else:
                    lines.append(f'        {out_field}[ip * {dim} + {component}] += {var_name};')
            else:
                lines.append(f'        {out_field}[ip] += {var_name};')

        line_idx += 1

    lines.append(f'    }}')
    lines.append(f'}}')
    return '\n'.join(lines)


def translate_file_incremental(c_file_path: str, func_name: str, write_dir: str,
                               is_vxc_only: bool = False) -> list:
    """Translate to incremental split files with shared preamble extraction.

    Generates one .rs file per (level, spin) where each function is annotated
    with incremental structure (shared preamble + per-level deltas).

    Creates: write_dir/func_name/mod.rs + write_dir/func_name/{level}_{spin}.rs

    The generated code is semantically identical to translate_file_split() output
    but structured to help the CubeCL proc macro process smaller logical units.
    Future optimization: the section comments mark boundaries where #[cube] helper
    function calls can be extracted to further reduce proc macro workload.
    """
    with open(c_file_path) as f:
        c_source = f.read()

    max_order = detect_max_order(c_source)
    all_params = scan_param_accesses(c_source)
    imports = detect_imports(c_source)
    functions = extract_functions(c_source)
    import_lines = generate_import_lines(imports)

    levels = ['vxc', 'fxc', 'kxc', 'lxc'] if is_vxc_only else ['exc', 'vxc', 'fxc', 'kxc', 'lxc']

    subdir = os.path.join(write_dir, func_name)
    os.makedirs(subdir, exist_ok=True)

    mod_entries = []
    written = []

    for spin in ['unpol', 'pol']:
        # Compute incremental structure for this spin
        preamble_lines, _ = detect_shared_preamble(functions, spin)
        deltas = detect_incremental_deltas(functions, spin)

        for level in levels:
            if max_order < LEVEL_ORDER[level]:
                continue
            key = (level, spin)
            if key not in functions:
                continue

            compute, outputs = parse_function_body(functions[key])
            est = estimate_function_lines(compute, outputs)
            header_allow = '#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]'

            if est <= SPLIT_THRESHOLD:
                # Small enough — single incremental kernel
                fn_code = generate_incremental_function(
                    func_name, level, spin, compute, outputs,
                    all_params, is_vxc_only,
                    preamble_lines, deltas
                )

                sub_name = f'{level}_{spin}'
                file_lines = []
                file_lines.append(f'//! {func_name.upper()} {level} {spin} kernel (incremental).')
                file_lines.append(f'//!')
                file_lines.append(f'//! Auto-translated with incremental derivative structure.')
                file_lines.append(f'//! Preamble: {len(preamble_lines)} shared lines across all orders.')
                if level in deltas:
                    _, delta, _ = deltas[level]
                    file_lines.append(f'//! Delta: {len(delta)} lines unique to {level}.')
                file_lines.append(f'')
                file_lines.append(header_allow)
                file_lines.append(f'')
                file_lines.extend(import_lines)
                file_lines.append(f'')
                file_lines.append(fn_code)
                file_lines.append(f'')

                path = os.path.join(subdir, f'{sub_name}.rs')
                with open(path, 'w') as f:
                    f.write('\n'.join(file_lines))
                written.append(path)
                mod_entries.append(f'pub mod {sub_name};')
            else:
                # Oversized — split by output-array group (same path as
                # translate_file_split). Incremental sharing within the file
                # is sacrificed in favor of compile-time / RAM headroom.
                # See cubecl_macro_fanout_manual.md §10 + lda-2 OOM repro
                # (260512) for context.
                is_pol = (spin == 'pol')
                splits = split_by_output_array(compute, outputs, is_pol)
                splits = merge_small_splits(splits, SPLIT_THRESHOLD)

                final_splits = []
                for suffix, sub_compute, sub_outputs, sub_bufs in splits:
                    sub_est = estimate_function_lines(sub_compute, sub_outputs)
                    if sub_est > SPLIT_THRESHOLD and len(sub_outputs) > 1:
                        for ow in sub_outputs:
                            _, var_deps = build_dependency_graph(sub_compute)
                            output_vars = {ow.var}
                            if ow.var in var_deps:
                                output_vars |= var_deps[ow.var]
                            needed = transitive_deps(output_vars, var_deps)
                            comp_compute = [cl for cl in sub_compute
                                            if re.match(r'(\w+)\s*=', cl.rstrip(';').strip())
                                            and re.match(r'(\w+)\s*=', cl.rstrip(';').strip()).group(1) in needed]
                            final_splits.append((f'{ow.field}_{ow.component}', comp_compute, [ow], [ow.field]))
                        final_splits = merge_small_splits(final_splits, SPLIT_THRESHOLD)
                    else:
                        final_splits.append((suffix, sub_compute, sub_outputs, sub_bufs))

                total_parts = len(final_splits)
                for idx, (suffix, sub_compute, sub_outputs, sub_bufs) in enumerate(final_splits):
                    fn_suffix = f'_part{idx}_{suffix}'
                    fn_code = generate_function(func_name, level, spin, sub_compute,
                                                sub_outputs, all_params, is_vxc_only,
                                                fn_suffix=fn_suffix,
                                                out_bufs_override=sub_bufs)
                    sub_name = f'{level}_{spin}_part{idx}_{suffix}'
                    sub_lines = estimate_function_lines(sub_compute, sub_outputs)
                    file_lines = [
                        f'//! {func_name.upper()} {level} {spin} kernel — split part {idx}/{total_parts} ({suffix}).',
                        f'//! Split sub-kernel: outputs [{", ".join(sub_bufs)}] ({sub_lines} lines).',
                        '', header_allow, '',
                    ]
                    file_lines.extend(import_lines)
                    file_lines.extend(['', fn_code, ''])
                    path = os.path.join(subdir, f'{sub_name}.rs')
                    with open(path, 'w') as f:
                        f.write('\n'.join(file_lines))
                    written.append(path)
                    mod_entries.append(f'pub mod {sub_name};')

    # Write statistics file
    stats_lines = [f'//! {func_name.upper()} incremental translation statistics.']
    stats_lines.append(f'//!')
    for spin in ['unpol', 'pol']:
        preamble_lines, _ = detect_shared_preamble(functions, spin)
        deltas = detect_incremental_deltas(functions, spin)
        stats_lines.append(f'//! {spin}: preamble={len(preamble_lines)} lines')
        for level in levels:
            if level in deltas:
                shared, delta, outs = deltas[level]
                stats_lines.append(f'//!   {level}: shared={shared}, delta={len(delta)}, outputs={len(outs)}')
    stats_lines.append(f'')

    # Write mod.rs
    mod_lines = [f'//! {func_name.upper()} kernel — incremental derivative structure.']
    mod_lines.append(f'//!')
    mod_lines.extend(stats_lines[1:])  # include stats as doc comments
    mod_lines.extend(mod_entries)
    mod_lines.append(f'')

    mod_path = os.path.join(subdir, 'mod.rs')
    with open(mod_path, 'w') as f:
        f.write('\n'.join(mod_lines))
    written.append(mod_path)

    return written


# ---------------------------------------------------------------------------
# Phase 11 D-10: per-functional subcrate emission (splitter v2)
# ---------------------------------------------------------------------------

_LDA_FILE_HEADER = ('#![allow(unused_imports, unused_variables, non_snake_case, '
                    'clippy::excessive_precision, clippy::too_many_arguments, '
                    'clippy::needless_return)]')


def emit_per_functional(c_file: str, func_name: str, family: str = 'lda',
                        is_vxc_only: bool = False,
                        split_threshold: int = None) -> list:
    """Emit `func_name` as a per-functional subcrate under
    crates/kernels/{family}/<func>/ in the q02 nested-by-output layout, via
    translate_v2.emit. Single outputs still over the cap after the per-output
    cut are CSE-subdivided into D-02 tuple-return chunks.

    Reuses this translator's own primitives (parse_function_body,
    generate_function, split_by_output_array, ...) — only the destination +
    layout change. `maple_to_kernels.py translate` calls this directly,
    replacing the stale regen_phase09.py in-place-replacement pipeline.
    """
    if split_threshold is None:
        split_threshold = SPLIT_THRESHOLD
    with open(c_file) as f:
        c_src = f.read()
    imports_str = '\n'.join(generate_import_lines(detect_imports(c_src)))
    adapter = per_functional.FamilyAdapter(
        family=family,
        file_header=_LDA_FILE_HEADER,
        src_dir_label='lda_vxc' if is_vxc_only else 'lda_exc',
        is_routed=(func_name in cached_routed_funcnames(KERNEL_FAMILY)),
        needs_libm=False,  # LDA detect_imports has no libm path
        imports_str=imports_str,
        all_params=scan_param_accesses(c_src),
        bodies=extract_functions(c_src),
        max_order=detect_max_order(c_src),
        level_ord=LEVEL_ORDER,
        level_outputs=LEVEL_OUTPUTS,
        levels=(['vxc', 'fxc', 'kxc', 'lxc'] if is_vxc_only
                else ['exc', 'vxc', 'fxc', 'kxc', 'lxc']),
        translate_line=translate_expr,
        parse=lambda body, level, spin, vxc: parse_function_body(body),
        estimate=estimate_function_lines,
        gen_fn=generate_function,
        split_by_output=split_by_output_array,
        merge_small=merge_small_splits,
        build_dep_graph=lambda c, is_pol: build_dependency_graph(c),
        transitive_deps=transitive_deps,
        ow_var=lambda ow: ow.var,
        ow_field=lambda ow: ow.field,
        ow_component=lambda ow: ow.component,
        pol_dims=POL_DIMS,
    )
    return per_functional.emit_functional(adapter, func_name, is_vxc_only,
                                          split_threshold)


def main():
    if len(sys.argv) < 3:
        print("Usage: translate_lda_v2.py <c_file> <func_name> [--vxc-only] [--write-to <dir>] [--split] [--incremental] [--no-split]")
        sys.exit(1)

    c_file = sys.argv[1]
    func_name = sys.argv[2]
    is_vxc_only = '--vxc-only' in sys.argv
    split_mode = '--split' in sys.argv
    incremental_mode = '--incremental' in sys.argv
    no_split_mode = '--no-split' in sys.argv

    global SPLIT_THRESHOLD
    # Apply --no-split flag to disable splitting
    if no_split_mode:
        SPLIT_THRESHOLD = UNSPLITTABLE

    # Apply --split-threshold N to override the default 18000-line cap.
    # Useful for hot single-functional regen where the default leaves a
    # body just under threshold and proc-macro RAM blows up. Lower values
    # produce more (and smaller) `_partN` helpers.
    if '--split-threshold' in sys.argv:
        idx = sys.argv.index('--split-threshold')
        if idx + 1 < len(sys.argv):
            SPLIT_THRESHOLD = int(sys.argv[idx + 1])

    write_dir = None
    if '--write-to' in sys.argv:
        idx = sys.argv.index('--write-to')
        if idx + 1 < len(sys.argv):
            write_dir = sys.argv[idx + 1]

    if incremental_mode:
        if not write_dir:
            print("--incremental requires --write-to <dir>")
            sys.exit(1)
        written = translate_file_incremental(c_file, func_name, write_dir, is_vxc_only)
        for p in written:
            print(f'Wrote {p}')
    elif split_mode:
        if not write_dir:
            print("--split requires --write-to <dir>")
            sys.exit(1)
        written = translate_file_split(c_file, func_name, write_dir, is_vxc_only)
        for p in written:
            print(f'Wrote {p}')
    else:
        kernel_rs = translate_file(c_file, func_name, is_vxc_only)
        if write_dir:
            os.makedirs(write_dir, exist_ok=True)
            path = os.path.join(write_dir, f'{func_name}.rs')
            with open(path, 'w') as f:
                f.write(kernel_rs)
            print(f'Wrote {path}')
        else:
            print(kernel_rs)


if __name__ == '__main__':
    main()
