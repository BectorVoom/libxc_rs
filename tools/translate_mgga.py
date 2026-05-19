#!/usr/bin/env python3
"""
MGGA maple2c-to-Rust translator — built from scratch modeled on translate_gga.py.

Translates C kernel files from libxc maple2c to Rust #[cube(launch_unchecked)]
functions. Generates one .rs file per (derivative_level, spin_mode) pair inside
a per-functional directory.

Usage:
    translate_mgga.py <c_file> <func_name> --write-to <dir> [--vxc-only]
    translate_mgga.py --batch --write-to <dir>
"""

import re
import sys
import os
from pathlib import Path

# Routing helper: emit `#[cube]` instead of `#[cube(launch_unchecked)]` for
# functionals that `MggaFunctional::from_id` does not route, so regen does
# not re-introduce dead-code launch wrappers (cubecl_macro_fanout_manual §4.3).
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from kernel_routing import cached_routed_funcnames
# Phase 11 splitter v2: CSE pass + per-functional subcrate emitter (D-01/D-04/D-10).
from translate_v2.cse import partition_compute_lines, Chunk
from translate_v2 import emit
from translate_v2 import per_functional
KERNEL_FAMILY = "mgga"


# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------

LEVELS = ['exc', 'vxc', 'fxc', 'kxc', 'lxc']
LEVEL_ORD = {l: i for i, l in enumerate(LEVELS)}

# Cumulative output buffers per derivative level (70 MGGA fields total at lxc)
LEVEL_OUTPUTS = {
    'exc': ['zk'],
    'vxc': ['zk', 'vrho', 'vsigma', 'vlapl', 'vtau'],
    'fxc': ['zk', 'vrho', 'vsigma', 'vlapl', 'vtau',
            'v2rho2', 'v2rhosigma', 'v2rholapl', 'v2rhotau',
            'v2sigma2', 'v2sigmalapl', 'v2sigmatau',
            'v2lapl2', 'v2lapltau', 'v2tau2'],
    'kxc': ['zk', 'vrho', 'vsigma', 'vlapl', 'vtau',
            'v2rho2', 'v2rhosigma', 'v2rholapl', 'v2rhotau',
            'v2sigma2', 'v2sigmalapl', 'v2sigmatau',
            'v2lapl2', 'v2lapltau', 'v2tau2',
            'v3rho3', 'v3rho2sigma', 'v3rho2lapl', 'v3rho2tau',
            'v3rhosigma2', 'v3rhosigmalapl', 'v3rhosigmatau',
            'v3rholapl2', 'v3rholapltau', 'v3rhotau2',
            'v3sigma3', 'v3sigma2lapl', 'v3sigma2tau',
            'v3sigmalapl2', 'v3sigmalapltau', 'v3sigmatau2',
            'v3lapl3', 'v3lapl2tau', 'v3lapltau2', 'v3tau3'],
    'lxc': ['zk', 'vrho', 'vsigma', 'vlapl', 'vtau',
            'v2rho2', 'v2rhosigma', 'v2rholapl', 'v2rhotau',
            'v2sigma2', 'v2sigmalapl', 'v2sigmatau',
            'v2lapl2', 'v2lapltau', 'v2tau2',
            'v3rho3', 'v3rho2sigma', 'v3rho2lapl', 'v3rho2tau',
            'v3rhosigma2', 'v3rhosigmalapl', 'v3rhosigmatau',
            'v3rholapl2', 'v3rholapltau', 'v3rhotau2',
            'v3sigma3', 'v3sigma2lapl', 'v3sigma2tau',
            'v3sigmalapl2', 'v3sigmalapltau', 'v3sigmatau2',
            'v3lapl3', 'v3lapl2tau', 'v3lapltau2', 'v3tau3',
            'v4rho4', 'v4rho3sigma', 'v4rho3lapl', 'v4rho3tau',
            'v4rho2sigma2', 'v4rho2sigmalapl', 'v4rho2sigmatau',
            'v4rho2lapl2', 'v4rho2lapltau', 'v4rho2tau2',
            'v4rhosigma3', 'v4rhosigma2lapl', 'v4rhosigma2tau',
            'v4rhosigmalapl2', 'v4rhosigmalapltau', 'v4rhosigmatau2',
            'v4rholapl3', 'v4rholapl2tau', 'v4rholapltau2', 'v4rhotau3',
            'v4sigma4', 'v4sigma3lapl', 'v4sigma3tau',
            'v4sigma2lapl2', 'v4sigma2lapltau', 'v4sigma2tau2',
            'v4sigmalapl3', 'v4sigmalapl2tau', 'v4sigmalapltau2', 'v4sigmatau3',
            'v4lapl4', 'v4lapl3tau', 'v4lapl2tau2', 'v4lapltau3', 'v4tau4'],
}

# Polarized component count per output field (unpolarized always 1)
# Verified against verify/src/lib.rs lines 440-491
POL_DIMS = {
    'zk': 1,
    'vrho': 2, 'vsigma': 3, 'vlapl': 2, 'vtau': 2,
    'v2rho2': 3, 'v2rhosigma': 6, 'v2rholapl': 4, 'v2rhotau': 4,
    'v2sigma2': 6, 'v2sigmalapl': 6, 'v2sigmatau': 6,
    'v2lapl2': 3, 'v2lapltau': 4, 'v2tau2': 3,
    'v3rho3': 4, 'v3rho2sigma': 9, 'v3rho2lapl': 6, 'v3rho2tau': 6,
    'v3rhosigma2': 12, 'v3rhosigmalapl': 12, 'v3rhosigmatau': 12,
    'v3rholapl2': 6, 'v3rholapltau': 8, 'v3rhotau2': 6,
    'v3sigma3': 10, 'v3sigma2lapl': 12, 'v3sigma2tau': 12,
    'v3sigmalapl2': 9, 'v3sigmalapltau': 12, 'v3sigmatau2': 9,
    'v3lapl3': 4, 'v3lapl2tau': 6, 'v3lapltau2': 6, 'v3tau3': 4,
    'v4rho4': 5, 'v4rho3sigma': 12, 'v4rho3lapl': 8, 'v4rho3tau': 8,
    'v4rho2sigma2': 18, 'v4rho2sigmalapl': 18, 'v4rho2sigmatau': 18,
    'v4rho2lapl2': 9, 'v4rho2lapltau': 12, 'v4rho2tau2': 9,
    'v4rhosigma3': 20, 'v4rhosigma2lapl': 36, 'v4rhosigma2tau': 36,
    'v4rhosigmalapl2': 18, 'v4rhosigmalapltau': 24, 'v4rhosigmatau2': 36,
    'v4rholapl3': 8, 'v4rholapl2tau': 12, 'v4rholapltau2': 12, 'v4rhotau3': 8,
    'v4sigma4': 15, 'v4sigma3lapl': 20, 'v4sigma3tau': 30,
    'v4sigma2lapl2': 18, 'v4sigma2lapltau': 24, 'v4sigma2tau2': 18,
    'v4sigmalapl3': 12, 'v4sigmalapl2tau': 18, 'v4sigmalapltau2': 18, 'v4sigmatau3': 12,
    'v4lapl4': 5, 'v4lapl3tau': 8, 'v4lapl2tau2': 9, 'v4lapltau3': 8, 'v4tau4': 5,
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
        for m in re.finditer(rf'{pfx}->(\w+)(?!\[|\w)', c_src):
            fld = m.group(1)
            if pfx == 'p' and fld in skip: continue
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
            imports.append((fn.lower(), 'libxc_kernel_math::powers'))

    if re.search(r'\bcbrt\(', c_src):
        imports.append(('safe_cbrt', 'libxc_kernel_math::powers'))

    if 'my_piecewise3(' in c_src:
        imports.append(('piecewise3', 'libxc_kernel_math::piecewise'))
    if 'my_piecewise5(' in c_src:
        imports.append(('piecewise5', 'libxc_kernel_math::piecewise'))
    if re.search(r'\bHeaviside\(', c_src):
        imports.append(('Heaviside', 'libxc_kernel_math::piecewise'))

    if re.search(r'\berf\(', c_src):
        imports.append(('erf_approx', 'libxc_kernel_math::erf'))
    if re.search(r'\berfc\(', c_src):
        imports.append(('erfc_approx', 'libxc_kernel_math::erf'))

    consts = ['M_PI', 'M_CBRT2', 'M_CBRT3', 'M_CBRT4', 'M_CBRT5', 'M_CBRT6',
              'M_CBRT7', 'M_CBRT9', 'M_CBRTPI', 'M_SQRTPI', 'M_SQRT2',
              'M_SQRT3', 'RS_FACTOR', 'X_FACTOR_C', 'K_FACTOR_C',
              'FZETAFACTOR', 'KF_CONST', 'M_C']
    for c in consts:
        if re.search(r'\b' + c + r'\b', c_src):
            imports.append((c, 'libxc_kernel_math::constants'))

    if re.search(r'\bLambertW\(', c_src):
        imports.append(('lambert_w', 'libxc_kernel_math::lambert_w'))
    if re.search(r'\bxc_E1_scaled\(', c_src):
        imports.append(('xc_e1_scaled', 'libxc_kernel_math::expint_e1'))
    if re.search(r'\bxc_erfcx\(', c_src):
        imports.append(('xc_erfcx', 'libxc_kernel_math::special'))
    if re.search(r'\bxc_dilogarithm\(', c_src):
        imports.append(('xc_dilogarithm', 'libxc_kernel_math::special'))
    if re.search(r'\bxc_integrate_func0\(', c_src):
        imports.append(('xc_integrate_func0', 'libxc_kernel_math::integrate'))
    if re.search(r'\bxc_integrate_func1\(', c_src):
        imports.append(('xc_integrate_func1', 'libxc_kernel_math::integrate'))
    if re.search(r'\bxbspline\(', c_src):
        imports.append(('case21_xbspline', 'libxc_kernel_math::bspline'))
    if re.search(r'\bcbspline\(', c_src):
        imports.append(('case21_cbspline', 'libxc_kernel_math::bspline'))
    # Phase 11.1 (2026-05-19): detect Bessel imports — mgga_x_2d_prhg07 uses
    # xc_bessel_I0/I1, and future MGGA functionals may pull K0/K1 too.
    if re.search(r'\bxc_bessel_I0\b', c_src):
        imports.append(('xc_bessel_I0', 'libxc_kernel_math::bessel'))
    if re.search(r'\bxc_bessel_I0_scaled\b', c_src):
        imports.append(('xc_bessel_I0_scaled', 'libxc_kernel_math::bessel'))
    if re.search(r'\bxc_bessel_I1\b', c_src):
        imports.append(('xc_bessel_I1', 'libxc_kernel_math::bessel'))
    if re.search(r'\bxc_bessel_I1_scaled\b', c_src):
        imports.append(('xc_bessel_I1_scaled', 'libxc_kernel_math::bessel'))
    if re.search(r'\bxc_bessel_K0\b', c_src):
        imports.append(('xc_bessel_K0', 'libxc_kernel_math::bessel'))
    if re.search(r'\bxc_bessel_K0_scaled\b', c_src):
        imports.append(('xc_bessel_K0_scaled', 'libxc_kernel_math::bessel'))
    if re.search(r'\bxc_bessel_K1\b', c_src):
        imports.append(('xc_bessel_K1', 'libxc_kernel_math::bessel'))
    if re.search(r'\bxc_bessel_K1_scaled\b', c_src):
        imports.append(('xc_bessel_K1_scaled', 'libxc_kernel_math::bessel'))

    # Cross-functional MGGA helper calls (root-finders defined in other crates).
    # Without these imports the consumer kernel files fail to compile.
    if re.search(r'\bxc_mgga_x_br89_get_x\(', c_src):
        imports.append(('xc_mgga_x_br89_get_x', 'libxc_kernel_math::br89'))
    if re.search(r'\bxc_mgga_x_mbrxc_get_x\(', c_src):
        imports.append(('xc_mgga_x_mbrxc_get_x', 'libxc_kernel_math::mbrxc'))

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

    # --- Parameter accesses: params->field[i][j] -> param_field_i_j ---
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

    # --- Power macros --- (lowercase + `::<f64>` turbofish for flat path)
    # Phase 11.1-01-fix1: emit `::<f64>` turbofish here so the FLAT emission
    # path (small functionals not CSE-subdivided) compiles. The CHUNKED
    # emission path retargets `::<f64>` -> `::<F>` via
    # tools/translate_v2/per_functional.py::_wrap_f64_literals_v2's
    # _FN_F64_TURBOFISH_RE retargeting pass (D-08 native chunked emit).
    # See PATTERN.md Rule 10 § "SUPERSEDED 2026-05-19" annotation.
    # Idempotent: re-running on `pow_1_3::<f64>(` does not match `pow_1_3(`.
    for macro in ['POW_1_3', 'POW_2_3', 'POW_4_3', 'POW_5_3', 'POW_3_2',
                  'POW_1_4', 'POW_7_3', 'POW_2', 'POW_3']:
        s = s.replace(f'{macro}(', f'{macro.lower()}::<f64>(')

    # --- Piecewise macros (NAME REMAP + ::<f64> turbofish; chunked retargets) ---
    s = s.replace('my_piecewise5(', 'piecewise5::<f64>(')
    s = s.replace('my_piecewise3(', 'piecewise3::<f64>(')

    # --- C math functions -> Rust ---
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
        (r'\bxc_integrate_func0\(', 'xc_integrate_func0('),
        (r'\bxc_integrate_func1\(', 'xc_integrate_func1('),
    ]
    for c_pat, rust_fn in math_map:
        s = re.sub(c_pat, rust_fn, s)

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
        s = s.replace('lapl[0]', 'lapl0')
        s = s.replace('lapl[1]', 'lapl1')
        s = s.replace('tau[0]', 'tau0')
        s = s.replace('tau[1]', 'tau1')
    else:
        s = s.replace('rho[0]', 'rho[ip]')
        s = s.replace('sigma[0]', 'sigma[ip]')
        s = s.replace('lapl[0]', 'lapl[ip]')
        s = s.replace('tau[0]', 'tau[ip]')

    # --- Integer literal arguments in function calls ---
    s = re.sub(r',\s*(\d+)\s*\)', lambda m: f', {m.group(1)}.0)', s)
    s = re.sub(r',\s*(\d+)\s*,', lambda m: f', {m.group(1)}.0,', s)

    # --- Restore bspline ider placeholders to integer literals ---
    s = re.sub(r'__IDER_(\d+)__', r'\1', s)

    # Phase-2 helper turbofish (`::<f64>` form for flat path)
    # Phase 11.1-01-fix1: same rationale as Block A above — flat emission
    # path requires `::<f64>`; chunked path retargets to `::<F>` via
    # _wrap_f64_literals_v2's _FN_F64_TURBOFISH_RE.
    # Idempotent: `\bname\(` does not match `name::<f64>(` on subsequent passes.
    for fn in [
        'xc_mgga_x_br89_get_x',                          # br89.rs
        'xc_mgga_x_mbrxc_get_x',                         # mbrxc.rs
        'erf_approx', 'erfc_approx',                     # erf.rs (math_map output)
        'xc_dilogarithm', 'xc_erfcx',                    # special.rs (math_map output)
        'xc_e1_scaled',                                  # expint_e1.rs (math_map output)
        'xc_integrate_func0', 'xc_integrate_func1',      # integrate.rs (gga_x_fd_lb94)
        'xc_bessel_I0_scaled', 'xc_bessel_I0',
        'xc_bessel_I1_scaled', 'xc_bessel_I1',           # bessel.rs (I)
        'xc_bessel_K0_scaled', 'xc_bessel_K0',
        'xc_bessel_K1_scaled', 'xc_bessel_K1',           # bessel.rs (K, added 2026-05-19 Phase 11.1)
        'lambert_w',                                     # lambert_w.rs (added 2026-05-19 Phase 11.1; used by mgga_x_2d_prhg07)
        'case21_xbspline', 'case21_cbspline',            # bspline.rs
    ]:
        s = re.sub(rf'\b{fn}\(', f'{fn}::<f64>(', s)

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
        # Handles all 4 MGGA guard patterns:
        # Pattern A: if(out->zk != NULL && ...)
        # Pattern B: if(out->vrho != NULL && ... NEEDS_LAPLACIAN ...)
        # Pattern C: if(out->vrho != NULL && ... NEEDS_TAU ...)
        # Pattern D: if(out->v2rho2 != NULL && ... NEEDS_LAPLACIAN ... NEEDS_TAU ...)
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
# Shared preamble and incremental delta detection
# ---------------------------------------------------------------------------

def detect_shared_preamble(bodies: dict, spin: str) -> tuple:
    """Detect the shared preamble (common prefix) across all derivative orders.

    Args:
        bodies: dict of (level, spin) -> function body text
        spin: 'unpol' or 'pol'

    Returns:
        (preamble_lines, per_level_remaining) where:
        - preamble_lines: list of C compute lines common to ALL derivative orders
        - per_level_remaining: dict of level -> list of lines AFTER the preamble
    """
    levels_present = []
    level_compute = {}
    for level in LEVELS:
        key = (level, spin)
        if key in bodies:
            compute, _ = parse_body(bodies[key], level, spin, False)
            level_compute[level] = compute
            levels_present.append(level)

    if not levels_present:
        return [], {}

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


def detect_incremental_deltas(bodies: dict, spin: str) -> dict:
    """Compute incremental deltas between consecutive derivative orders.

    Returns:
        dict of level -> (shared_line_count, delta_lines, output_writes)
    """
    levels_present = []
    level_data = {}
    for level in LEVELS:
        key = (level, spin)
        if key in bodies:
            compute, outputs = parse_body(bodies[key], level, spin, False)
            level_data[level] = (compute, outputs)
            levels_present.append(level)

    if not levels_present:
        return {}

    result = {}
    first = levels_present[0]
    first_compute, first_outputs = level_data[first]
    result[first] = (0, first_compute, first_outputs)

    for idx in range(1, len(levels_present)):
        curr_level = levels_present[idx]
        prev_level = levels_present[idx - 1]
        curr_compute, curr_outputs = level_data[curr_level]
        prev_compute, _ = level_data[prev_level]

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


# Default threshold: if a generated kernel function exceeds this many lines,
# split it into per-output-array sub-kernels.
# History: 5000 → 18000 (09-04 / CONTEXT D-06) → 50000 (q02) → 100000 (q07).
# Lowered 100000 → 6000 (260512) after lda-2 OOM'd cargo check on 13–17K-line
# `#[cube]` functions. CubeCL macro fan-out scales super-linearly with body
# size (cubecl_macro_fanout_manual.md §10). 6000 sits comfortably above
# lda-1's 4,229-line max which compiles fast.
# --no-split flag disables splitting by setting threshold to max.
# Phase 11 D-LOCK-B: 4500 leaves headroom vs the 5000 hard cap. Memory
# project_split_threshold_history: do not go below 4500 without recalibrating.
SPLIT_THRESHOLD = 4500
UNSPLITTABLE = 999999


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
    """Compute transitive closure of dependencies."""
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
    for field, comp, var_name in output_writes:
        groups.setdefault(field, []).append((field, comp, var_name))
    all_out_arrays = LEVEL_OUTPUTS.get('lxc', [])
    group_order = [f for f in all_out_arrays if f in groups]
    for f in groups:
        if f not in group_order:
            group_order.append(f)
    result = []
    for field in group_order:
        sub_outputs = groups[field]
        output_vars = set()
        for _, _, var_name in sub_outputs:
            output_vars.add(var_name)
            if var_name in var_deps:
                output_vars |= var_deps[var_name]
        needed = transitive_deps(output_vars, var_deps)
        sub_compute = [cl for cl in compute_lines
                       if re.match(r'(\w+)\s*=', cl.rstrip(';').strip())
                       and re.match(r'(\w+)\s*=', cl.rstrip(';').strip()).group(1) in needed]
        result.append((field, sub_compute, sub_outputs, [field]))
    return result


# Cap descriptive part of a merged-split suffix so the final filename
# stays under the Linux 255-byte per-component limit. lxc-level kernels
# can have 40+ output components; naive concatenation `f'{prev}_{new}'`
# produces 400+ char filenames (e.g. lxc_unpol_part0_zk_vrho_vsigma_..._v4rho2lapl2.rs).
# Uniqueness is provided by the `_partN` index in the caller; this is
# purely the descriptive portion.
_SUFFIX_MAX_CHARS = 60


def _capped_merge_suffix(prev_suffix: str, new_suffix: str) -> str:
    combined = f'{prev_suffix}_{new_suffix}'
    if len(combined) <= _SUFFIX_MAX_CHARS:
        return combined
    return f'{combined[:_SUFFIX_MAX_CHARS]}_etc'


def merge_small_splits(splits, threshold):
    """Merge consecutive small splits to reduce sub-kernel count."""
    if not splits:
        return splits
    merged = [splits[0]]
    for suffix, compute, outputs, bufs in splits[1:]:
        prev_suffix, prev_compute, prev_outputs, prev_bufs = merged[-1]
        if len(prev_compute) + len(compute) < threshold:
            seen = set()
            merged_compute = []
            for cline in prev_compute + compute:
                stripped = cline.rstrip(';').strip()
                m = re.match(r'(\w+)\s*=', stripped)
                key = m.group(1) if m else cline
                if key not in seen:
                    seen.add(key)
                    merged_compute.append(cline)
            merged[-1] = (_capped_merge_suffix(prev_suffix, suffix), merged_compute,
                         prev_outputs + outputs, prev_bufs + bufs)
        else:
            merged.append((suffix, compute, outputs, bufs))
    return merged


def estimate_function_lines(compute_lines, output_writes):
    return len(compute_lines) + len(output_writes) + 30


def generate_function(func_name: str, level: str, spin: str,
                      compute_lines: list, output_writes: list,
                      all_params: list, is_vxc_only: bool,
                      fn_suffix: str = '', out_bufs_override: list = None) -> str:
    """Generate a single Rust #[cube(...)] function.

    When `fn_suffix` is empty the function is the per-(level, spin) entry
    kernel and is emitted as `#[cube(launch_unchecked)]`. When `fn_suffix`
    starts with `_part` the function is a split-helper that the dispatch
    layer never invokes directly; emit it as plain `#[cube]` per
    `docs/manual/Cubecl/cubecl_macro_fanout_manual.md` §4.3 / §23 to skip
    the launch-wrapper boilerplate (Anti-pattern 1).
    """
    is_pol = (spin == 'pol')
    fn_name = f'{func_name}_{level}_{spin}{fn_suffix}'
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
        out_bufs = [b for b in LEVEL_OUTPUTS[level] if b != 'zk']
    else:
        out_bufs = LEVEL_OUTPUTS[level]

    used_params = find_used_params(compute_lines, all_params)

    # Build output var -> (field, component) map
    out_map = {}
    for field, comp, var in output_writes:
        out_map[var] = (field, comp)

    # Check for special bspline calls
    raw_text = ' '.join(compute_lines)
    has_xbspline = 'xbspline(' in raw_text
    has_cbspline = 'cbspline(' in raw_text

    lines = []
    lines.append(f'#[allow(unused_variables, non_snake_case)]')
    lines.append(cube_attr)
    lines.append(f'pub fn {fn_name}(')
    lines.append(f'    rho: &Array<f64>,')
    lines.append(f'    sigma: &Array<f64>,')
    lines.append(f'    lapl: &Array<f64>,')
    lines.append(f'    tau: &Array<f64>,')
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
        lines.append(f'        let lapl0 = lapl[ip * 2];')
        lines.append(f'        let lapl1 = lapl[ip * 2 + 1];')
        lines.append(f'        let tau0 = tau[ip * 2];')
        lines.append(f'        let tau1 = tau[ip * 2 + 1];')

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
                           is_vxc_only: bool) -> list:
    """Translate a single (level, spin) function to one or more .rs file contents.

    Returns a list of (sub_name, rs_code) tuples. Normally one entry, but
    oversized kernels are split into per-output-array sub-kernels.
    """
    bodies = extract_function_bodies(c_src)
    key = (level, spin)
    if key not in bodies:
        return []

    max_order = detect_max_order(c_src)
    if LEVEL_ORD[level] > max_order:
        return []

    compute_lines, output_writes = parse_body(bodies[key], level, spin, is_vxc_only)
    est = estimate_function_lines(compute_lines, output_writes)
    src_dir = 'mgga_vxc' if is_vxc_only else 'mgga_exc'

    if est <= SPLIT_THRESHOLD:
        fn_code = generate_function(func_name, level, spin, compute_lines,
                                    output_writes, all_params, is_vxc_only)
        rs_code = (
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
        return [(f'{level}_{spin}', rs_code)]

    # --- Oversized: split by output-array group ---
    is_pol = (spin == 'pol')
    splits = split_by_output_array(compute_lines, output_writes, is_pol)
    splits = merge_small_splits(splits, SPLIT_THRESHOLD)

    # Further split if individual chunks are still too large
    final_splits = []
    for suffix, sub_compute, sub_outputs, sub_bufs in splits:
        sub_est = estimate_function_lines(sub_compute, sub_outputs)
        if sub_est > SPLIT_THRESHOLD and len(sub_outputs) > 1:
            for field, comp, var_name in sub_outputs:
                var_order, var_deps = build_dependency_graph(sub_compute)
                output_vars = {var_name}
                if var_name in var_deps:
                    output_vars |= var_deps[var_name]
                needed = transitive_deps(output_vars, var_deps)
                comp_compute = [cl for cl in sub_compute
                                if re.match(r'(\w+)\s*=', cl.rstrip(';').strip())
                                and re.match(r'(\w+)\s*=', cl.rstrip(';').strip()).group(1) in needed]
                final_splits.append((f'{field}_{comp}', comp_compute,
                                    [(field, comp, var_name)], [field]))
            final_splits = merge_small_splits(final_splits, SPLIT_THRESHOLD)
        else:
            final_splits.append((suffix, sub_compute, sub_outputs, sub_bufs))

    results = []
    total_parts = len(final_splits)
    for idx, (suffix, sub_compute, sub_outputs, sub_bufs) in enumerate(final_splits):
        fn_suffix = f'_part{idx}'
        fn_code = generate_function(func_name, level, spin, sub_compute,
                                    sub_outputs, all_params, is_vxc_only,
                                    fn_suffix=fn_suffix,
                                    out_bufs_override=sub_bufs)
        sub_name = f'{level}_{spin}_part{idx}'
        sub_lines = estimate_function_lines(sub_compute, sub_outputs)
        rs_code = (
            f'//! {func_name.upper()} {level} {spin} kernel — split part {idx}/{total_parts} ({suffix}).\n'
            f'//!\n'
            f'//! Auto-translated from `libxc-master/src/maple2c/{src_dir}/{func_name}.c`.\n'
            f'//! Split sub-kernel: outputs [{", ".join(sub_bufs)}] ({sub_lines} lines).\n'
            f'\n'
            f'{FILE_HEADER}\n'
            f'\n'
            f'{imports_str}\n'
            f'\n'
            f'{fn_code}\n'
        )
        results.append((sub_name, rs_code))

    return results


# Math functions that don't have Rust implementations yet
UNIMPLEMENTED_MATH = {
    # xc_dilogarithm -> libxc_kernel_math::special::xc_dilogarithm (implemented)
    # xc_erfcx -> libxc_kernel_math::special::xc_erfcx (implemented)
    # xc_integrate_func0/func1 -> libxc_kernel_math::integrate (implemented)
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
            file_list = translate_one_function(c_src, func_name, level, spin,
                                              all_params, imports_str, is_vxc_only)
            for sub_name, rs_code in file_list:
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
# Incremental generation
# ---------------------------------------------------------------------------

def generate_incremental_function(func_name: str, level: str, spin: str,
                                  full_compute_lines: list, output_writes: list,
                                  all_params: list, is_vxc_only: bool,
                                  preamble_lines: list,
                                  prior_levels_delta: dict) -> str:
    """Generate a #[cube(launch_unchecked)] function with incremental structure annotations."""
    is_pol = (spin == 'pol')
    fn_name = f'{func_name}_{level}_{spin}'

    if is_vxc_only:
        out_bufs = [b for b in LEVEL_OUTPUTS[level] if b != 'zk']
    else:
        out_bufs = LEVEL_OUTPUTS[level]

    used_params = find_used_params(full_compute_lines, all_params)

    out_map = {}
    for field, comp, var in output_writes:
        out_map[var] = (field, comp)

    raw_text = ' '.join(full_compute_lines)
    has_xbspline = 'xbspline(' in raw_text
    has_cbspline = 'cbspline(' in raw_text

    is_unrouted = func_name not in cached_routed_funcnames(KERNEL_FAMILY)
    cube_attr = '#[cube]' if is_unrouted else '#[cube(launch_unchecked)]'

    lines = []
    lines.append(f'#[allow(unused_variables, non_snake_case)]')
    lines.append(cube_attr)
    lines.append(f'pub fn {fn_name}(')
    lines.append(f'    rho: &Array<f64>,')
    lines.append(f'    sigma: &Array<f64>,')
    lines.append(f'    lapl: &Array<f64>,')
    lines.append(f'    tau: &Array<f64>,')
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

    bounds_arr = out_bufs[0] if out_bufs else 'vrho'
    lines.append(f'    let ip = ABSOLUTE_POS;')
    lines.append(f'    if ip < {bounds_arr}.len() {{')

    if is_pol:
        lines.append(f'        let rho0 = rho[ip * 2];')
        lines.append(f'        let rho1 = rho[ip * 2 + 1];')
        lines.append(f'        let sigma0 = sigma[ip * 3];')
        lines.append(f'        let sigma1 = sigma[ip * 3 + 1];')
        lines.append(f'        let sigma2 = sigma[ip * 3 + 2];')
        lines.append(f'        let lapl0 = lapl[ip * 2];')
        lines.append(f'        let lapl1 = lapl[ip * 2 + 1];')
        lines.append(f'        let tau0 = tau[ip * 2];')
        lines.append(f'        let tau1 = tau[ip * 2 + 1];')

    # Build section boundaries
    preamble_count = len(preamble_lines)
    current_idx = LEVELS.index(level)

    section_boundaries = []
    if preamble_count > 0:
        section_boundaries.append((0, preamble_count, 'shared preamble'))

    prev_end = preamble_count
    for lvl_idx in range(current_idx + 1):
        lvl = LEVELS[lvl_idx]
        if lvl not in prior_levels_delta:
            continue
        shared_count, delta, _ = prior_levels_delta[lvl]
        delta_start = shared_count
        delta_end = shared_count + len(delta)
        actual_start = max(delta_start, prev_end)
        if actual_start < delta_end:
            label = f'{lvl} delta (this level)' if lvl == level else f'{lvl} delta'
            section_boundaries.append((actual_start, delta_end, label))
            prev_end = delta_end

    line_idx = 0
    section_idx = 0
    for cline in full_compute_lines:
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
        translated = translate_line(expr, is_pol)
        lines.append(f'        let {var_name} = {translated};')

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

        line_idx += 1

    lines.append(f'    }}')
    lines.append(f'}}')
    return '\n'.join(lines)


# ---------------------------------------------------------------------------
# Phase 11 D-10: per-functional subcrate emission (splitter v2)
# ---------------------------------------------------------------------------

def emit_per_functional(c_file: str, func_name: str, family: str = 'mgga',
                        is_vxc_only: bool = False,
                        split_threshold: int = None) -> list:
    """Emit `func_name` as a per-functional subcrate under
    crates/kernels/{family}/<func>/ in the q02 nested-by-output layout, via
    translate_v2.emit. Single outputs still over the cap after the per-output
    cut are CSE-subdivided into D-02 tuple-return chunks.

    Reuses this translator's own primitives (parse_body, generate_function,
    split_by_output_array, ...) — only the destination + layout change.
    `maple_to_kernels.py translate` calls this directly, replacing the stale
    regen_phase09.py in-place-replacement pipeline.
    """
    if split_threshold is None:
        split_threshold = SPLIT_THRESHOLD
    c_src = read_c_source(c_file)
    unimp = check_unimplemented_math(c_src)
    if unimp:
        raise RuntimeError(unimp)
    imports = detect_imports(c_src)
    imports_str = format_imports(imports)
    needs_libm = any('libm' in str(m).lower() for _n, m in imports)
    adapter = per_functional.FamilyAdapter(
        family=family,
        file_header=FILE_HEADER,
        src_dir_label='mgga_vxc' if is_vxc_only else 'mgga_exc',
        is_routed=(func_name in cached_routed_funcnames(KERNEL_FAMILY)),
        needs_libm=needs_libm,
        imports_str=imports_str,
        all_params=scan_params(c_src),
        bodies=extract_function_bodies(c_src),
        max_order=detect_max_order(c_src),
        level_ord=LEVEL_ORD,
        level_outputs=LEVEL_OUTPUTS,
        levels=(['vxc', 'fxc', 'kxc', 'lxc'] if is_vxc_only
                else ['exc', 'vxc', 'fxc', 'kxc', 'lxc']),
        translate_line=translate_line,
        parse=parse_body,
        estimate=estimate_function_lines,
        gen_fn=generate_function,
        split_by_output=split_by_output_array,
        merge_small=merge_small_splits,
        build_dep_graph=lambda c, is_pol: build_dependency_graph(c),
        transitive_deps=transitive_deps,
        ow_var=lambda ow: ow[2],
        ow_field=lambda ow: ow[0],
        ow_component=lambda ow: ow[1],
        pol_dims=POL_DIMS,
    )
    return per_functional.emit_functional(adapter, func_name, is_vxc_only,
                                          split_threshold)


def translate_functional_incremental(c_file: str, func_name: str, out_dir: str,
                                     is_vxc_only: bool = False) -> list:
    """Translate with incremental derivative structure annotations."""
    c_src = read_c_source(c_file)

    unimp = check_unimplemented_math(c_src)
    if unimp:
        raise RuntimeError(unimp)

    max_order = detect_max_order(c_src)
    all_params = scan_params(c_src)
    imports = detect_imports(c_src)
    imports_str = format_imports(imports)
    bodies = extract_function_bodies(c_src)

    levels = (['vxc', 'fxc', 'kxc', 'lxc'] if is_vxc_only
              else ['exc', 'vxc', 'fxc', 'kxc', 'lxc'])

    func_dir = os.path.join(out_dir, func_name)
    os.makedirs(func_dir, exist_ok=True)

    mod_entries = []
    written = []

    for spin in ('unpol', 'pol'):
        preamble_lines, _ = detect_shared_preamble(bodies, spin)
        deltas = detect_incremental_deltas(bodies, spin)

        for level in levels:
            if LEVEL_ORD[level] > max_order:
                continue
            key = (level, spin)
            if key not in bodies:
                continue

            compute, output_writes = parse_body(bodies[key], level, spin, is_vxc_only)
            src_dir = 'mgga_vxc' if is_vxc_only else 'mgga_exc'
            est = estimate_function_lines(compute, output_writes)

            if est <= SPLIT_THRESHOLD:
                fn_code = generate_incremental_function(
                    func_name, level, spin, compute, output_writes,
                    all_params, is_vxc_only, preamble_lines, deltas)

                sub_name = f'{level}_{spin}'
                file_lines = [
                    f'//! {func_name.upper()} {level} {spin} kernel (incremental).',
                    f'//!',
                    f'//! Auto-translated with incremental derivative structure.',
                    f'//! Preamble: {len(preamble_lines)} shared lines across all orders.',
                ]
                if level in deltas:
                    _, delta, _ = deltas[level]
                    file_lines.append(f'//! Delta: {len(delta)} lines unique to {level}.')
                file_lines.extend([
                    f'',
                    FILE_HEADER,
                    f'',
                    imports_str,
                    f'',
                    fn_code,
                    f'',
                ])

                path = os.path.join(func_dir, f'{sub_name}.rs')
                with open(path, 'w') as f:
                    f.write('\n'.join(file_lines))
                written.append(path)
                mod_entries.append(f'pub mod {sub_name};')
            else:
                # Oversized — split by output-array group, same as the
                # non-incremental split path. Mirrors translate_gga.py's
                # threshold-aware incremental flow.
                is_pol = (spin == 'pol')
                splits = split_by_output_array(compute, output_writes, is_pol)
                splits = merge_small_splits(splits, SPLIT_THRESHOLD)

                final_splits = []
                for suffix, sub_compute, sub_outputs, sub_bufs in splits:
                    sub_est = estimate_function_lines(sub_compute, sub_outputs)
                    if sub_est > SPLIT_THRESHOLD and len(sub_outputs) > 1:
                        for field, comp, var_name in sub_outputs:
                            _, var_deps = build_dependency_graph(sub_compute)
                            output_vars = {var_name}
                            if var_name in var_deps:
                                output_vars |= var_deps[var_name]
                            needed = transitive_deps(output_vars, var_deps)
                            comp_compute = [cl for cl in sub_compute
                                            if re.match(r'(\w+)\s*=', cl.rstrip(';').strip())
                                            and re.match(r'(\w+)\s*=', cl.rstrip(';').strip()).group(1) in needed]
                            final_splits.append((f'{field}_{comp}', comp_compute,
                                                [(field, comp, var_name)], [field]))
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
                        f'//!',
                        f'//! Auto-translated from `libxc-master/src/maple2c/{src_dir}/{func_name}.c`.',
                        f'//! Split sub-kernel: outputs [{", ".join(sub_bufs)}] ({sub_lines} lines).',
                        f'//! Each sub-kernel recomputes its dependency chain from inputs.',
                        f'',
                        FILE_HEADER,
                        f'',
                        imports_str,
                        f'',
                        fn_code,
                        f'',
                    ]
                    path = os.path.join(func_dir, f'{sub_name}.rs')
                    with open(path, 'w') as f:
                        f.write('\n'.join(file_lines))
                    written.append(path)
                    mod_entries.append(f'pub mod {sub_name};')

    # Write mod.rs with incremental stats
    mod_lines = [f'//! {func_name.upper()} kernel -- incremental derivative structure.', '']
    for spin in ('unpol', 'pol'):
        preamble_lines, _ = detect_shared_preamble(bodies, spin)
        deltas = detect_incremental_deltas(bodies, spin)
        mod_lines.append(f'//! {spin}: preamble={len(preamble_lines)} lines')
        for level in levels:
            if level in deltas:
                shared, delta, outs = deltas[level]
                mod_lines.append(f'//!   {level}: shared={shared}, delta={len(delta)}, outputs={len(outs)}')
    mod_lines.append('')
    mod_lines.extend(mod_entries)
    mod_lines.append('')

    mod_path = os.path.join(func_dir, 'mod.rs')
    with open(mod_path, 'w') as f:
        f.write('\n'.join(mod_lines))
    written.append(mod_path)

    return written


# ---------------------------------------------------------------------------
# Batch mode
# ---------------------------------------------------------------------------

def batch_translate(out_dir: str, c_dir: str = 'libxc-master/src/maple2c'):
    """Translate all MGGA functionals."""
    mgga_exc_dir = os.path.join(c_dir, 'mgga_exc')
    mgga_vxc_dir = os.path.join(c_dir, 'mgga_vxc')

    all_funcs = []

    # Standard mgga_exc files
    for fname in sorted(os.listdir(mgga_exc_dir)):
        if not fname.endswith('.c') or 'Zone' in fname or fname == 'Makefile.am':
            continue
        func_name = fname[:-2]  # strip .c
        c_path = os.path.join(mgga_exc_dir, fname)
        all_funcs.append((c_path, func_name, False))

    # Special mgga_vxc files
    if os.path.isdir(mgga_vxc_dir):
        for fname in sorted(os.listdir(mgga_vxc_dir)):
            if not fname.endswith('.c') or 'Zone' in fname or fname == 'Makefile.am':
                continue
            func_name = fname[:-2]
            c_path = os.path.join(mgga_vxc_dir, fname)
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
        '//! MGGA kernel translations from maple2c.',
        f'//!',
        f'//! Auto-generated: {len(mod_entries)} MGGA functionals.',
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
    global SPLIT_THRESHOLD

    if '--batch' in sys.argv:
        idx = sys.argv.index('--write-to')
        out_dir = sys.argv[idx + 1]
        batch_translate(out_dir)
        return

    if len(sys.argv) < 3:
        print("Usage: translate_mgga.py <c_file> <func_name> --write-to <dir> [--vxc-only] [--incremental] [--split-threshold N] [--no-split]")
        print("       translate_mgga.py --batch --write-to <dir>")
        sys.exit(1)

    c_file = sys.argv[1]
    func_name = sys.argv[2]
    is_vxc_only = '--vxc-only' in sys.argv
    incremental = '--incremental' in sys.argv
    no_split_mode = '--no-split' in sys.argv

    # Apply --no-split flag to disable splitting
    if no_split_mode:
        SPLIT_THRESHOLD = UNSPLITTABLE

    # Parse --split-threshold (overrides module-level SPLIT_THRESHOLD).
    if '--split-threshold' in sys.argv:
        idx_st = sys.argv.index('--split-threshold')
        threshold = int(sys.argv[idx_st + 1])
        SPLIT_THRESHOLD = threshold

    idx = sys.argv.index('--write-to')
    out_dir = sys.argv[idx + 1]

    if incremental:
        written = translate_functional_incremental(c_file, func_name, out_dir, is_vxc_only)
    else:
        written = translate_functional(c_file, func_name, out_dir, is_vxc_only)
    for p in written:
        print(f'Wrote {p}')


if __name__ == '__main__':
    main()
