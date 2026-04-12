#!/usr/bin/env python3
"""
GGA maple2c-to-Rust translator v2 — rebuilt from scratch.

Translates C kernel files from libxc maple2c to Rust #[cube(launch_unchecked)]
functions. Generates one .rs file per (derivative_level, spin_mode) pair inside
a per-functional directory.

Usage:
    translate_gga_v2.py <c_file> <func_name> --write-to <dir> [--vxc-only]
    translate_gga_v2.py --batch --write-to <dir>
"""

import re
import sys
import os
from pathlib import Path

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------

LEVELS = ['exc', 'vxc', 'fxc', 'kxc', 'lxc']
LEVEL_ORD = {l: i for i, l in enumerate(LEVELS)}

# Cumulative output buffers per derivative level
LEVEL_OUTPUTS = {
    'exc': ['zk'],
    'vxc': ['zk', 'vrho', 'vsigma'],
    'fxc': ['zk', 'vrho', 'vsigma', 'v2rho2', 'v2rhosigma', 'v2sigma2'],
    'kxc': ['zk', 'vrho', 'vsigma', 'v2rho2', 'v2rhosigma', 'v2sigma2',
            'v3rho3', 'v3rho2sigma', 'v3rhosigma2', 'v3sigma3'],
    'lxc': ['zk', 'vrho', 'vsigma', 'v2rho2', 'v2rhosigma', 'v2sigma2',
            'v3rho3', 'v3rho2sigma', 'v3rhosigma2', 'v3sigma3',
            'v4rho4', 'v4rho3sigma', 'v4rho2sigma2', 'v4rhosigma3', 'v4sigma4'],
}

# Polarized component count per output field (unpolarized always 1)
POL_DIMS = {
    'zk': 1,
    'vrho': 2, 'vsigma': 3,
    'v2rho2': 3, 'v2rhosigma': 6, 'v2sigma2': 6,
    'v3rho3': 4, 'v3rho2sigma': 9, 'v3rhosigma2': 12, 'v3sigma3': 10,
    'v4rho4': 5, 'v4rho3sigma': 12, 'v4rho2sigma2': 18, 'v4rhosigma3': 20, 'v4sigma4': 15,
}

FILE_HEADER = (
    '#![allow(unused_imports, unused_variables, non_snake_case, '
    'clippy::excessive_precision, clippy::too_many_arguments, '
    'clippy::needless_return)]'
)

# ---------------------------------------------------------------------------
# C source extraction
# ---------------------------------------------------------------------------

def read_c_source(path: str) -> str:
    with open(path) as f:
        return f.read()


def detect_max_order(c_src: str) -> int:
    m = re.search(r'#define\s+maple2c_order\s+(\d+)', c_src)
    return int(m.group(1)) if m else 4


def extract_function_bodies(c_src: str) -> dict:
    """Return {(level, spin): body_text} for every func_{level}_{spin} found."""
    bodies = {}
    for level in LEVELS:
        for spin in ('unpol', 'pol'):
            pat = rf'func_{level}_{spin}\s*\([^)]*\)\s*\{{'
            m = re.search(pat, c_src)
            if not m:
                continue
            start = m.end() - 1
            depth, i = 1, start + 1
            while i < len(c_src) and depth > 0:
                if c_src[i] == '{':   depth += 1
                elif c_src[i] == '}': depth -= 1
                i += 1
            bodies[(level, spin)] = c_src[start:i]
    return bodies


# ---------------------------------------------------------------------------
# Parameter detection
# ---------------------------------------------------------------------------

def scan_params(c_src: str) -> list:
    """Return sorted unique list of (field, indices_tuple) for all parameter accesses."""
    skip = {'dens_threshold', 'zeta_threshold', 'dim', 'info', 'params'}
    accesses = set()
    for pfx in ('params', 'p'):
        # 2D: pfx->field[i][j]
        for m in re.finditer(rf'{pfx}->(\w+)\[(\d+)\]\[(\d+)\]', c_src):
            fld = m.group(1)
            if pfx == 'p' and fld in skip: continue
            accesses.add((fld, (int(m.group(2)), int(m.group(3)))))
        # 1D: pfx->field[i] (not followed by [)
        for m in re.finditer(rf'{pfx}->(\w+)\[(\d+)\](?!\[)', c_src):
            fld = m.group(1)
            if pfx == 'p' and fld in skip: continue
            accesses.add((fld, (int(m.group(2)),)))
        # Scalar: pfx->field (not followed by [ or word char)
            # Handle carefully to not match sub-patterns of 1D/2D
        for m in re.finditer(rf'{pfx}->(\w+)(?!\[|\w)', c_src):
            fld = m.group(1)
            if pfx == 'p' and fld in skip: continue
            # Verify it's not followed by [ (the lookahead should handle this)
            accesses.add((fld, ()))
    return sorted(accesses)


def param_rust_name(field: str, indices: tuple) -> str:
    if not indices:
        return f"param_{field}"
    return f"param_{field}_{'_'.join(str(i) for i in indices)}"


# ---------------------------------------------------------------------------
# Import detection
# ---------------------------------------------------------------------------

def detect_imports(c_src: str) -> list:
    """Return list of (rust_name, module) pairs needed."""
    imports = []
    power_fns = ['POW_1_3', 'POW_2_3', 'POW_4_3', 'POW_5_3', 'POW_3_2',
                 'POW_1_4', 'POW_7_3', 'POW_2', 'POW_3']
    for fn in power_fns:
        if f'{fn}(' in c_src:
            imports.append((fn.lower(), 'crate::math::powers'))

    if re.search(r'\bcbrt\(', c_src):
        imports.append(('safe_cbrt', 'crate::math::powers'))

    if 'my_piecewise3(' in c_src:
        imports.append(('piecewise3', 'crate::math::piecewise'))
    if 'my_piecewise5(' in c_src:
        imports.append(('piecewise5', 'crate::math::piecewise'))

    if re.search(r'\berf\(', c_src):
        imports.append(('erf_approx', 'crate::math::erf'))
    if re.search(r'\berfc\(', c_src):
        imports.append(('erfc_approx', 'crate::math::erf'))

    consts = ['M_PI', 'M_CBRT2', 'M_CBRT3', 'M_CBRT4', 'M_CBRT5', 'M_CBRT6',
              'M_CBRT7', 'M_CBRT9', 'M_CBRTPI', 'M_SQRTPI', 'M_SQRT2',
              'M_SQRT3', 'RS_FACTOR', 'X_FACTOR_C', 'K_FACTOR_C',
              'FZETAFACTOR', 'KF_CONST', 'M_C']
    for c in consts:
        if re.search(r'\b' + c + r'\b', c_src):
            imports.append((c, 'crate::math::constants'))

    if re.search(r'\bLambertW\(', c_src):
        imports.append(('lambert_w', 'crate::math::lambert_w'))
    if re.search(r'\bxc_E1_scaled\(', c_src):
        imports.append(('xc_e1_scaled', 'crate::math::expint_e1'))
    if re.search(r'\bxc_erfcx\(', c_src):
        imports.append(('xc_erfcx', 'crate::math::special'))
    if re.search(r'\bxc_dilogarithm\(', c_src):
        imports.append(('xc_dilogarithm', 'crate::math::special'))
    if re.search(r'\bxbspline\(', c_src):
        imports.append(('case21_xbspline', 'crate::math::bspline'))
    if re.search(r'\bcbspline\(', c_src):
        imports.append(('case21_cbspline', 'crate::math::bspline'))
    if re.search(r'\bxc_integrate\(func0', c_src):
        imports.append(('xc_integrate_func0', 'crate::math::integrate'))
    if re.search(r'\bxc_integrate\(func1', c_src):
        imports.append(('xc_integrate_func1', 'crate::math::integrate'))

    return imports


def format_imports(imports: list) -> str:
    lines = ['use cubecl::prelude::*;']
    by_mod = {}
    for name, mod_path in imports:
        by_mod.setdefault(mod_path, set()).add(name)
    for mod_path in sorted(by_mod):
        names = ', '.join(sorted(by_mod[mod_path]))
        lines.append(f'use {mod_path}::{{{names}}};')
    return '\n'.join(lines)


# ---------------------------------------------------------------------------
# Expression translation
# ---------------------------------------------------------------------------

def translate_line(line: str, is_pol: bool) -> str:
    """Translate a single C computation line to Rust."""
    s = line

    # --- Parameter accesses: params->field[i][j] → param_field_i_j ---
    s = re.sub(r'params->(\w+)\[(\d+)\]\[(\d+)\]',
               lambda m: f'param_{m.group(1)}_{m.group(2)}_{m.group(3)}', s)
    s = re.sub(r'params->(\w+)\[(\d+)\](?!\[)',
               lambda m: f'param_{m.group(1)}_{m.group(2)}', s)
    s = re.sub(r'params->(\w+)', lambda m: f'param_{m.group(1)}', s)

    # --- p-> threshold accesses ---
    s = s.replace('p->dens_threshold', 'dens_threshold')
    s = s.replace('p->zeta_threshold', 'zeta_threshold')

    # --- p-> parameter accesses (same as params->) ---
    s = re.sub(r'p->(\w+)\[(\d+)\]\[(\d+)\]',
               lambda m: f'param_{m.group(1)}_{m.group(2)}_{m.group(3)}', s)
    s = re.sub(r'p->(\w+)\[(\d+)\](?!\[)',
               lambda m: f'param_{m.group(1)}_{m.group(2)}', s)
    s = re.sub(r'p->(\w+)', lambda m: f'param_{m.group(1)}', s)

    # --- Power macros ---
    for macro in ['POW_1_3', 'POW_2_3', 'POW_4_3', 'POW_5_3', 'POW_3_2',
                  'POW_1_4', 'POW_7_3', 'POW_2', 'POW_3']:
        s = s.replace(f'{macro}(', f'{macro.lower()}(')

    # --- Piecewise macros ---
    s = s.replace('my_piecewise5(', 'piecewise5(')
    s = s.replace('my_piecewise3(', 'piecewise3(')

    # --- C math functions → Rust ---
    math_map = [
        (r'\blog\(',    'f64::ln('),
        (r'\bsqrt\(',   'f64::sqrt('),
        (r'\bexp\(',    'f64::exp('),
        (r'\batan2\(',  'f64::atan2('),
        (r'\batan\(',   'f64::atan('),
        (r'\basin\(',   'f64::asin('),
        (r'\bacos\(',   'f64::acos('),
        (r'\bsin\(',    'f64::sin('),
        (r'\bcos\(',    'f64::cos('),
        (r'\btanh\(',   'f64::tanh('),
        (r'\bsinh\(',   'f64::sinh('),
        (r'\bcosh\(',   'f64::cosh('),
        (r'\bfabs\(',   'f64::abs('),
        (r'\bcbrt\(',   'safe_cbrt('),
        (r'\bpow\(',    'f64::powf('),
        (r'\berfc\(',   'erfc_approx('),
        (r'\berf\(',    'erf_approx('),
        (r'\bLambertW\(',      'lambert_w('),
        (r'\bxc_E1_scaled\(',  'xc_e1_scaled('),
        (r'\bxc_erfcx\(',      'xc_erfcx('),
        (r'\bxc_dilogarithm\(','xc_dilogarithm('),
    ]
    for c_pat, rust_fn in math_map:
        s = re.sub(c_pat, rust_fn, s)

    # --- xc_integrate(func0/func1, NULL, 0.0, upper) → specialized Rust calls ---
    # The beta parameter comes from params->beta, already translated to param_beta
    s = re.sub(r'\bxc_integrate\(func0,\s*NULL,\s*[^,]+,\s*([^)]+)\)',
               r'xc_integrate_func0(\1, param_beta)', s)
    s = re.sub(r'\bxc_integrate\(func1,\s*NULL,\s*[^,]+,\s*([^)]+)\)',
               r'xc_integrate_func1(\1, param_beta)', s)

    # --- C constants ---
    s = re.sub(r'\bDBL_EPSILON\b', 'f64::EPSILON', s)
    s = re.sub(r'\bSQRT_DBL_EPSILON\b', 'f64::sqrt(f64::EPSILON)', s)

    # --- B-spline helper calls (case21) ---
    cx_args = ", ".join(f"param_cx_{i}" for i in range(10))
    cc_args = ", ".join(f"param_cc_{i}" for i in range(10))
    # Use __IDER_N__ placeholder to protect integer ider arg from the float conversion below
    s = re.sub(r'\bxbspline\(([^,]+),\s*(\d+),\s*params\)',
               lambda m: f'case21_xbspline({m.group(1)}, __IDER_{m.group(2)}__, {cx_args})', s)
    s = re.sub(r'\bcbspline\(([^,]+),\s*(\d+),\s*params\)',
               lambda m: f'case21_cbspline({m.group(1)}, __IDER_{m.group(2)}__, {cc_args})', s)

    # --- Numeric literal translation ---
    # maple2c uses 0.XYZeN notation. Rust accepts this directly.
    # We only simplify obvious integer values like 0.2e1 → 2.0, 0.1e1 → 1.0
    def simplify_literal(m):
        txt = m.group(1)
        try:
            val = float(txt)
            if val == int(val) and abs(val) < 1e15 and len(txt) < 12:
                return f"{int(val)}.0"
        except ValueError:
            pass
        return txt
    s = re.sub(r'(?<![a-zA-Z_\d])(\d+\.\d+e[+-]?\d+)', simplify_literal, s)
    s = re.sub(r'(?<![a-zA-Z_\d])0\.e0\b', '0.0', s)

    # --- Input array access ---
    if is_pol:
        s = s.replace('rho[0]', 'rho0')
        s = s.replace('rho[1]', 'rho1')
        s = s.replace('sigma[0]', 'sigma0')
        s = s.replace('sigma[1]', 'sigma1')
        s = s.replace('sigma[2]', 'sigma2')
    else:
        s = s.replace('rho[0]', 'rho[ip]')
        s = s.replace('sigma[0]', 'sigma[ip]')

    # --- Integer literal arguments in function calls ---
    # Convert , 0) → , 0.0) and , 0, → , 0.0, for f64 function arguments
    # But NOT inside array indexing [] or variable names
    # We do this carefully: only inside function call parentheses
    s = re.sub(r',\s*(\d+)\s*\)', lambda m: f', {m.group(1)}.0)', s)
    s = re.sub(r',\s*(\d+)\s*,', lambda m: f', {m.group(1)}.0,', s)

    # --- Restore bspline ider placeholders to integer literals ---
    s = re.sub(r'__IDER_(\d+)__', r'\1', s)

    return s


# ---------------------------------------------------------------------------
# Body parsing
# ---------------------------------------------------------------------------

def parse_body(body_text: str, level: str, spin: str, is_vxc_only: bool):
    """Parse C function body into (compute_lines, output_writes).

    compute_lines: list of raw C computation lines (before translation)
    output_writes: list of (field, component, var_name) tuples
    """
    computes = []
    outputs = []

    lines = body_text.split('\n')
    i, in_body, depth = 0, False, 0

    while i < len(lines):
        raw = lines[i]
        stripped = raw.strip()

        if not in_body:
            if '{' in stripped:
                depth += stripped.count('{') - stripped.count('}')
                in_body = True
            i += 1
            continue

        depth += stripped.count('{') - stripped.count('}')
        if depth <= 0:
            break

        # Skip: empty, comments, preprocessor, variable decls, param setup, asserts
        if (not stripped or
            stripped.startswith('//') or stripped.startswith('/*') or
            stripped.startswith('#') or stripped.startswith('double ') or
            '_params *params' in stripped or 'params = (' in stripped or
            'assert(' in stripped):
            i += 1
            continue

        # Output guard: if(out->field != NULL ...)
        if stripped.startswith('if(out->'):
            # Next non-empty line should be the output write
            j = i + 1
            while j < len(lines):
                nxt = lines[j].strip()
                if not nxt:
                    j += 1
                    continue
                # Pattern: out->field[ip*p->dim.field + N] += var;
                m = re.match(
                    r'out->(\w+)\[ip\s*\*\s*p->dim\.\w+\s*\+\s*(\d+)\]\s*\+=\s*(\w+)\s*;',
                    nxt
                )
                if m:
                    outputs.append((m.group(1), int(m.group(2)), m.group(3)))
                break
            i = j + 1
            continue

        computes.append(stripped)
        i += 1

    return computes, outputs


# ---------------------------------------------------------------------------
# Rust function generation
# ---------------------------------------------------------------------------

def find_used_params(compute_lines: list, all_params: list) -> list:
    """Filter params to only those referenced in the compute lines."""
    text = ' '.join(compute_lines)
    used = []
    for field, indices in all_params:
        patterns = []
        if len(indices) == 2:
            patterns += [f'params->{field}[{indices[0]}][{indices[1]}]',
                        f'p->{field}[{indices[0]}][{indices[1]}]']
        elif len(indices) == 1:
            patterns += [f'params->{field}[{indices[0]}]',
                        f'p->{field}[{indices[0]}]']
        else:
            patterns += [f'params->{field}', f'p->{field}']
        if any(p in text for p in patterns):
            used.append((field, indices))
    return used


def generate_function(func_name: str, level: str, spin: str,
                      compute_lines: list, output_writes: list,
                      all_params: list, is_vxc_only: bool) -> str:
    """Generate a single Rust #[cube(launch_unchecked)] function."""
    is_pol = (spin == 'pol')
    fn_name = f'{func_name}_{level}_{spin}'

    if is_vxc_only:
        out_bufs = [b for b in LEVEL_OUTPUTS[level] if b != 'zk']
    else:
        out_bufs = LEVEL_OUTPUTS[level]

    used_params = find_used_params(compute_lines, all_params)

    # Build output var → (field, component) map
    out_map = {}
    for field, comp, var in output_writes:
        out_map[var] = (field, comp)

    # Check for special function calls
    raw_text = ' '.join(compute_lines)
    has_xbspline = 'xbspline(' in raw_text
    has_xc_integrate = 'xc_integrate(' in raw_text

    # xc_integrate calls inject param_beta — ensure it's in the parameter list
    if has_xc_integrate and ('beta', ()) not in used_params:
        used_params.append(('beta', ()))
    has_cbspline = 'cbspline(' in raw_text

    lines = []
    lines.append(f'#[allow(unused_variables, non_snake_case)]')
    lines.append(f'#[cube(launch_unchecked)]')
    lines.append(f'pub fn {fn_name}(')
    lines.append(f'    rho: &Array<f64>,')
    lines.append(f'    sigma: &Array<f64>,')
    for buf in out_bufs:
        lines.append(f'    {buf}: &mut Array<f64>,')
    for field, indices in used_params:
        lines.append(f'    {param_rust_name(field, indices)}: f64,')
    if has_xbspline:
        for i in range(10):
            lines.append(f'    param_cx_{i}: f64,')
    if has_cbspline:
        for i in range(10):
            lines.append(f'    param_cc_{i}: f64,')
    lines.append(f'    dens_threshold: f64,')
    lines.append(f'    zeta_threshold: f64,')
    lines.append(f') {{')

    # Bounds check
    bounds_arr = out_bufs[0] if out_bufs else 'vrho'
    lines.append(f'    let ip = ABSOLUTE_POS;')
    lines.append(f'    if ip < {bounds_arr}.len() {{')

    # Load polarized inputs
    if is_pol:
        lines.append(f'        let rho0 = rho[ip * 2];')
        lines.append(f'        let rho1 = rho[ip * 2 + 1];')
        lines.append(f'        let sigma0 = sigma[ip * 3];')
        lines.append(f'        let sigma1 = sigma[ip * 3 + 1];')
        lines.append(f'        let sigma2 = sigma[ip * 3 + 2];')

    # Translate computation lines and insert output writes
    for cline in compute_lines:
        stripped = cline.rstrip(';').strip()
        m = re.match(r'(\w+)\s*=\s*(.*)', stripped)
        if not m:
            continue

        var_name = m.group(1)
        expr = m.group(2)
        translated = translate_line(expr, is_pol)
        lines.append(f'        let {var_name} = {translated};')

        # If this variable is an output write, emit the += line
        if var_name in out_map:
            out_field, component = out_map[var_name]
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


# ---------------------------------------------------------------------------
# File generation
# ---------------------------------------------------------------------------

def translate_one_function(c_src: str, func_name: str, level: str, spin: str,
                           all_params: list, imports_str: str,
                           is_vxc_only: bool) -> str | None:
    """Translate a single (level, spin) function to a complete .rs file."""
    bodies = extract_function_bodies(c_src)
    key = (level, spin)
    if key not in bodies:
        return None

    max_order = detect_max_order(c_src)
    if LEVEL_ORD[level] > max_order:
        return None

    compute_lines, output_writes = parse_body(bodies[key], level, spin, is_vxc_only)
    fn_code = generate_function(func_name, level, spin, compute_lines,
                                output_writes, all_params, is_vxc_only)

    src_dir = 'gga_vxc' if is_vxc_only else 'gga_exc'
    return (
        f'//! {func_name.upper()} {level} {spin} kernel.\n'
        f'//!\n'
        f'//! Auto-translated from `libxc-master/src/maple2c/{src_dir}/{func_name}.c`.\n'
        f'//! Preserves exact maple2c variable names and FP operation order.\n'
        f'\n'
        f'{FILE_HEADER}\n'
        f'\n'
        f'{imports_str}\n'
        f'\n'
        f'{fn_code}\n'
    )


# Math functions that don't have Rust implementations yet
UNIMPLEMENTED_MATH = {
    # All previously-deferred functions are now implemented
}


def check_unimplemented_math(c_src: str) -> str | None:
    """Return reason string if the C source needs unimplemented math, else None."""
    for func, reason in UNIMPLEMENTED_MATH.items():
        if re.search(r'\b' + func + r'\(', c_src):
            return f'needs {func} ({reason})'
    return None


def translate_functional(c_file: str, func_name: str, out_dir: str,
                         is_vxc_only: bool = False) -> list:
    """Translate a full functional to split files under out_dir/func_name/."""
    c_src = read_c_source(c_file)

    # Check for unimplemented math before doing any work
    unimp = check_unimplemented_math(c_src)
    if unimp:
        raise RuntimeError(unimp)

    max_order = detect_max_order(c_src)
    all_params = scan_params(c_src)
    imports = detect_imports(c_src)
    imports_str = format_imports(imports)

    func_dir = os.path.join(out_dir, func_name)
    os.makedirs(func_dir, exist_ok=True)

    levels = (['vxc', 'fxc', 'kxc', 'lxc'] if is_vxc_only
              else ['exc', 'vxc', 'fxc', 'kxc', 'lxc'])

    mod_entries = []
    written = []

    for spin in ('unpol', 'pol'):
        for level in levels:
            rs_code = translate_one_function(c_src, func_name, level, spin,
                                            all_params, imports_str, is_vxc_only)
            if rs_code is None:
                continue

            sub_name = f'{level}_{spin}'
            path = os.path.join(func_dir, f'{sub_name}.rs')
            with open(path, 'w') as f:
                f.write(rs_code)
            written.append(path)
            mod_entries.append(f'pub mod {sub_name};')

    # Write mod.rs
    mod_rs = f'//! {func_name.upper()} kernel — split into per-function files.\n\n'
    mod_rs += '\n'.join(mod_entries) + '\n'
    mod_path = os.path.join(func_dir, 'mod.rs')
    with open(mod_path, 'w') as f:
        f.write(mod_rs)
    written.append(mod_path)

    return written


# ---------------------------------------------------------------------------
# Batch mode
# ---------------------------------------------------------------------------

def batch_translate(out_dir: str, c_dir: str = 'libxc-master/src/maple2c'):
    """Translate all GGA functionals."""
    gga_exc_dir = os.path.join(c_dir, 'gga_exc')
    gga_vxc_dir = os.path.join(c_dir, 'gga_vxc')

    all_funcs = []

    # Standard gga_exc files
    for fname in sorted(os.listdir(gga_exc_dir)):
        if not fname.endswith('.c') or 'Zone' in fname or fname == 'Makefile.am':
            continue
        func_name = fname[:-2]  # strip .c
        c_path = os.path.join(gga_exc_dir, fname)
        all_funcs.append((c_path, func_name, False))

    # Special gga_vxc files
    if os.path.isdir(gga_vxc_dir):
        for fname in sorted(os.listdir(gga_vxc_dir)):
            if not fname.endswith('.c') or 'Zone' in fname or fname == 'Makefile.am':
                continue
            func_name = fname[:-2]
            c_path = os.path.join(gga_vxc_dir, fname)
            all_funcs.append((c_path, func_name, True))

    mod_entries = []
    deferred = []

    for c_path, func_name, is_vxc in all_funcs:
        try:
            written = translate_functional(c_path, func_name, out_dir, is_vxc)
            mod_entries.append(f'pub mod {func_name};')
            print(f'  OK: {func_name} ({len(written)} files)')
        except Exception as e:
            deferred.append((func_name, str(e)))
            print(f'  SKIP: {func_name}: {e}')

    # Write top-level mod.rs
    mod_rs_lines = [
        '//! GGA kernel translations from maple2c.',
        f'//!',
        f'//! Auto-generated: {len(mod_entries)} GGA functionals.',
        '',
    ]
    for entry in mod_entries:
        mod_rs_lines.append(entry)

    if deferred:
        mod_rs_lines.append('')
        for name, reason in deferred:
            mod_rs_lines.append(f'// pub mod {name};  // deferred: {reason}')

    mod_rs_lines.append('')
    mod_path = os.path.join(out_dir, 'mod.rs')
    with open(mod_path, 'w') as f:
        f.write('\n'.join(mod_rs_lines))
    print(f'\nWrote {mod_path} ({len(mod_entries)} enabled, {len(deferred)} deferred)')


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def main():
    if '--batch' in sys.argv:
        idx = sys.argv.index('--write-to')
        out_dir = sys.argv[idx + 1]
        batch_translate(out_dir)
        return

    if len(sys.argv) < 3:
        print("Usage: translate_gga_v2.py <c_file> <func_name> --write-to <dir> [--vxc-only]")
        print("       translate_gga_v2.py --batch --write-to <dir>")
        sys.exit(1)

    c_file = sys.argv[1]
    func_name = sys.argv[2]
    is_vxc_only = '--vxc-only' in sys.argv

    idx = sys.argv.index('--write-to')
    out_dir = sys.argv[idx + 1]

    written = translate_functional(c_file, func_name, out_dir, is_vxc_only)
    for p in written:
        print(f'Wrote {p}')


if __name__ == '__main__':
    main()
