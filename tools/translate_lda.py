#!/usr/bin/env python3
"""
Translate maple2c C kernel files to Rust #[cube] functions.

This script reads a C source file from libxc's maple2c directory and produces:
1. A Rust kernel file with #[cube(launch_unchecked)] functions
2. A Rust launch wrapper file with safe wrappers

Translation rules:
- Exact maple2c variable names preserved (t2, t3, ..., tzk0, tvrho0)
- Floating-point operation order preserved
- Numeric literals: 0.XeN -> proper f64 literal
- POW_1_3(x) -> pow_1_3(x), etc.
- my_piecewise3/5 -> piecewise3/5
- M_CBRT3 etc. -> crate::math::constants::*
- Output accumulation via +=
"""

import re
import sys
import os
from pathlib import Path

# Functionals that have params and their field names
FUNC_PARAMS = {
    'lda_c_1d_csc': ['ferro', 'para'],
    'lda_c_2d_prm': ['N', 'c'],
    'lda_c_chachiyo': ['af', 'ap', 'bf', 'bp', 'cf', 'cp'],
    'lda_c_chachiyo_mod': ['af', 'ap', 'bf', 'bp', 'cf', 'cp'],
    'lda_c_hl': ['hl_c', 'hl_r'],
    'lda_c_lp96': ['C1', 'C2', 'C3'],
    'lda_c_ml1': ['fc', 'q'],
    'lda_c_pw': ['a', 'alpha1', 'beta1', 'beta2', 'beta3', 'beta4', 'fz20', 'pp'],
    'lda_c_pz': ['a', 'b', 'beta1', 'beta2', 'c', 'd', 'gamma'],
    'lda_c_wigner': ['a', 'b'],
    'lda_k_gds08_worker': ['A', 'B', 'C'],
    'lda_k_tf': ['ax'],
    'lda_x': ['alpha'],
    'lda_x_1d_exponential': ['beta'],
    'lda_x_1d_soft': ['beta'],
    'lda_x_sloc': ['a', 'b'],
    'lda_xc_1d_ehwlrg': ['a1', 'a2', 'a3', 'alpha'],
    'lda_xc_ksdt': ['T', 'b', 'c', 'd', 'e', 'thetaParam'],
}

# Rust keywords that cannot be used as parameter names
RUST_KEYWORDS = {'type', 'struct', 'fn', 'let', 'mut', 'ref', 'self', 'super',
                 'use', 'mod', 'pub', 'return', 'match', 'if', 'else', 'for',
                 'while', 'loop', 'break', 'continue', 'as', 'in', 'move',
                 'where', 'async', 'await', 'dyn', 'trait', 'impl', 'enum',
                 'const', 'static', 'extern', 'crate', 'true', 'false'}


def sanitize_param_name(name):
    """Make a param name safe for Rust."""
    # Lowercase for Rust convention
    rust_name = 'param_' + name if name in RUST_KEYWORDS else name
    return rust_name


def translate_numeric_literal(match_str):
    """Convert maple2c numeric literals to Rust f64."""
    # Handle patterns like 0.XYZeN
    # 0.1e1 -> 1.0, 0.2e1 -> 2.0, etc.
    try:
        val = float(match_str)
        # Format with enough precision
        if val == int(val) and abs(val) < 1e15:
            return f"{int(val)}.0"
        else:
            # Use repr for full precision
            s = repr(val)
            if '.' not in s and 'e' not in s and 'E' not in s:
                s = s + '.0'
            return s
    except ValueError:
        return match_str


def translate_expr(line, func_name, is_pol=False):
    """Translate a C expression line to Rust."""
    result = line

    # Strip trailing semicolons (we'll add them in let statements)
    result = result.rstrip(';').strip()

    # Replace params->field with param_field (scalar argument)
    result = re.sub(r'params->(\w+)', lambda m: f'param_{m.group(1)}', result)

    # Replace p->dens_threshold with dens_threshold
    result = result.replace('p->dens_threshold', 'dens_threshold')
    result = result.replace('p->zeta_threshold', 'zeta_threshold')

    # Replace C functions/macros
    result = result.replace('POW_1_3(', 'pow_1_3(')
    result = result.replace('POW_2_3(', 'pow_2_3(')
    result = result.replace('POW_4_3(', 'pow_4_3(')
    result = result.replace('POW_5_3(', 'pow_5_3(')
    result = result.replace('POW_3_2(', 'pow_3_2(')
    result = result.replace('POW_1_4(', 'pow_1_4(')
    result = result.replace('POW_7_3(', 'pow_7_3(')
    result = result.replace('POW_2(', 'pow_2(')
    result = result.replace('POW_3(', 'pow_3(')
    result = result.replace('my_piecewise5(', 'piecewise5(')
    result = result.replace('my_piecewise3(', 'piecewise3(')

    # Replace math functions with f64:: versions
    # Be careful with log - it should be f64::ln
    result = re.sub(r'\blog\(', 'f64::ln(', result)
    result = re.sub(r'\bsqrt\(', 'f64::sqrt(', result)
    result = re.sub(r'\bexp\(', 'f64::exp(', result)
    result = re.sub(r'\batan\(', 'f64::atan(', result)
    result = re.sub(r'\basin\(', 'f64::asin(', result)
    result = re.sub(r'\bacos\(', 'f64::acos(', result)
    result = re.sub(r'\btanh\(', 'f64::tanh(', result)
    result = re.sub(r'\bsinh\(', 'f64::sinh(', result)
    result = re.sub(r'\bcosh\(', 'f64::cosh(', result)
    result = re.sub(r'\bfabs\(', 'f64::abs(', result)
    result = re.sub(r'\berf\(', 'erf_cube(', result)
    # pow(x, y) -> f64::powf(x, y) -- but only standalone pow, not POW_*
    result = re.sub(r'\bpow\(', 'f64::powf(', result)

    # Replace M_PI with the constant from our math module (usable in #[cube] kernels)
    # Must replace M_PI before other M_ constants to avoid M_PI matching inside M_PIECEWISE etc.
    # Use word boundary to avoid replacing M_CBRTPI
    result = re.sub(r'\bM_PI\b', 'M_PI', result)

    # Replace integer literal 0 and 1 in piecewise contexts with f64
    # This is tricky - need to handle cases like piecewise3(cond, 0, expr) -> piecewise3(cond, 0.0, expr)
    # and piecewise3(cond, expr, 1) -> piecewise3(cond, expr, 1.0)

    # Translate numeric literals: 0.XeN format
    # Match patterns like 0.123e4, 0.1e1, etc. but NOT things like t0.1
    result = re.sub(r'(?<![a-zA-Z_])(\d+\.\d+e[+-]?\d+)', lambda m: translate_numeric_literal(m.group(1)), result)

    # Also handle standalone integers that should be f64
    # Replace lone integer 0 in expressions (not as array index) with 0.0
    # Be careful not to replace rho[0] indices
    # Only replace standalone 0 or 1 that appear as function arguments to piecewise

    # Replace rho[0] with rho[ip] for unpolarized
    if not is_pol:
        result = result.replace('rho[0]', 'rho[ip]')
    else:
        # For polarized: rho[0] -> rho[ip * 2], rho[1] -> rho[ip * 2 + 1]
        result = result.replace('rho[0]', 'rho[ip * 2]')
        result = result.replace('rho[1]', 'rho[ip * 2 + 1]')

    # Fix standalone integer literals that should be floats
    # In piecewise calls: piecewise3(..., 0, ...) -> piecewise3(..., 0.0, ...)
    # Also: arithmetic like * 1 -> * 1.0
    # But be careful not to touch array indices

    return result


def parse_c_function(text, func_type, spin):
    """Parse a single C function body and extract computation lines."""
    lines = []
    var_decls = []
    compute_lines = []
    output_lines = []

    in_body = False
    brace_depth = 0

    for line in text.split('\n'):
        stripped = line.strip()

        if not in_body:
            if '{' in stripped:
                brace_depth += stripped.count('{') - stripped.count('}')
                in_body = True
            continue

        brace_depth += stripped.count('{') - stripped.count('}')
        if brace_depth <= 0:
            break

        # Skip empty lines, comments, preprocessor
        if not stripped or stripped.startswith('//') or stripped.startswith('#') or stripped.startswith('/*'):
            continue

        # Skip variable declarations
        if stripped.startswith('double '):
            var_decls.append(stripped)
            continue

        # Skip params struct lines
        if '_params *params' in stripped or 'params = (' in stripped or 'assert(' in stripped:
            continue

        # Skip output write lines (we handle them separately)
        if 'out->' in stripped:
            output_lines.append(stripped)
            continue

        # Skip conditional output lines
        if stripped.startswith('if(out->'):
            continue

        # This is a computation line
        compute_lines.append(stripped)

    return var_decls, compute_lines, output_lines


def extract_functions(c_source):
    """Extract all functions from C source, grouped by (level, spin)."""
    functions = {}
    levels = ['exc', 'vxc', 'fxc', 'kxc', 'lxc']
    spins = ['unpol', 'pol']

    for level in levels:
        for spin in spins:
            func_name = f'func_{level}_{spin}'
            # Find the function in the source
            pattern = rf'func_{level}_{spin}\s*\([^)]+\)\s*\{{'
            match = re.search(pattern, c_source)
            if match:
                start = match.start()
                # Find the matching closing brace
                brace_count = 0
                pos = match.end() - 1  # at the opening brace
                for i in range(pos, len(c_source)):
                    if c_source[i] == '{':
                        brace_count += 1
                    elif c_source[i] == '}':
                        brace_count -= 1
                        if brace_count == 0:
                            func_text = c_source[start:i+1]
                            functions[(level, spin)] = func_text
                            break

    return functions


def get_output_fields(level):
    """Get the output fields for each derivative level."""
    if level == 'exc':
        return [('zk', 'tzk0')]
    elif level == 'vxc':
        return [('zk', 'tzk0'), ('vrho', 'tvrho0')]
    elif level == 'fxc':
        return [('zk', 'tzk0'), ('vrho', 'tvrho0'), ('v2rho2', 'tv2rho20')]
    elif level == 'kxc':
        return [('zk', 'tzk0'), ('vrho', 'tvrho0'), ('v2rho2', 'tv2rho20'), ('v3rho3', 'tv3rho30')]
    elif level == 'lxc':
        return [('zk', 'tzk0'), ('vrho', 'tvrho0'), ('v2rho2', 'tv2rho20'), ('v3rho3', 'tv3rho30'), ('v4rho4', 'tv4rho40')]
    return []


def get_output_fields_vxc_only(level):
    """For _vxc-only functionals (no exc output), e.g. lda_xc_tih."""
    if level == 'vxc':
        return [('vrho', 'tvrho0')]
    elif level == 'fxc':
        return [('vrho', 'tvrho0'), ('v2rho2', 'tv2rho20')]
    elif level == 'kxc':
        return [('vrho', 'tvrho0'), ('v2rho2', 'tv2rho20'), ('v3rho3', 'tv3rho30')]
    elif level == 'lxc':
        return [('vrho', 'tvrho0'), ('v2rho2', 'tv2rho20'), ('v3rho3', 'tv3rho30'), ('v4rho4', 'tv4rho40')]
    return []


def detect_output_vars(compute_lines, output_lines):
    """Detect which output variables are actually computed."""
    # Look at assignments: var = expr;
    assigned_vars = set()
    for line in compute_lines:
        m = re.match(r'\s*(\w+)\s*=', line)
        if m:
            assigned_vars.add(m.group(1))

    outputs = []
    # Check output_lines for which outputs are written
    for oline in output_lines:
        for field in ['zk', 'vrho', 'v2rho2', 'v3rho3', 'v4rho4']:
            if f'out->{field}' in oline:
                # Find the += variable
                m = re.search(r'\+=\s*(\w+)', oline)
                if m:
                    outputs.append((field, m.group(1)))

    return outputs


def translate_compute_line(line, is_pol):
    """Translate a single computation line from C to Rust."""
    stripped = line.strip().rstrip(';')

    # Check if it's an assignment
    m = re.match(r'(\w+)\s*=\s*(.*)', stripped)
    if m:
        var_name = m.group(1)
        expr = m.group(2)
        translated = translate_expr(expr, '', is_pol)
        return f'        let {var_name} = {translated};'

    # Otherwise just translate the expression
    return f'        {translate_expr(stripped, "", is_pol)};'


def detect_used_imports(compute_lines_text):
    """Detect which imports are needed based on compute lines."""
    imports = {
        'pow_1_3': False, 'pow_2_3': False, 'pow_4_3': False,
        'pow_5_3': False, 'pow_3_2': False, 'pow_1_4': False,
        'pow_7_3': False, 'pow_2': False, 'pow_3': False,
        'piecewise3': False, 'piecewise5': False,
        'erf_cube': False,
        'M_PI': False, 'M_CBRT2': False, 'M_CBRT3': False, 'M_CBRT4': False,
        'M_CBRT5': False, 'M_CBRT6': False, 'M_CBRT7': False,
        'M_CBRT9': False, 'M_CBRTPI': False, 'M_SQRTPI': False,
        'M_SQRT2': False, 'M_SQRT3': False, 'M_C': False,
        'RS_FACTOR': False, 'X_FACTOR_C': False,
        'K_FACTOR_C': False, 'FZETAFACTOR': False,
    }

    text = compute_lines_text

    # Check power functions
    if 'POW_1_3(' in text: imports['pow_1_3'] = True
    if 'POW_2_3(' in text: imports['pow_2_3'] = True
    if 'POW_4_3(' in text: imports['pow_4_3'] = True
    if 'POW_5_3(' in text: imports['pow_5_3'] = True
    if 'POW_3_2(' in text: imports['pow_3_2'] = True
    if 'POW_1_4(' in text: imports['pow_1_4'] = True
    if 'POW_7_3(' in text: imports['pow_7_3'] = True
    if 'POW_2(' in text: imports['pow_2'] = True
    if 'POW_3(' in text: imports['pow_3'] = True

    # Check piecewise
    if 'my_piecewise3(' in text: imports['piecewise3'] = True
    if 'my_piecewise5(' in text: imports['piecewise5'] = True

    # Check erf
    if re.search(r'\berf\(', text): imports['erf_cube'] = True

    # Check constants - use word boundary to avoid M_CBRT2 matching M_C
    for const in ['M_PI', 'M_CBRT2', 'M_CBRT3', 'M_CBRT4', 'M_CBRT5', 'M_CBRT6',
                  'M_CBRT7', 'M_CBRT9', 'M_CBRTPI', 'M_SQRTPI', 'M_SQRT2',
                  'M_SQRT3', 'RS_FACTOR', 'X_FACTOR_C', 'K_FACTOR_C',
                  'FZETAFACTOR']:
        if re.search(r'\b' + const + r'\b', text):
            imports[const] = True
    # M_C needs special care to not match M_CBRT etc.
    if re.search(r'\bM_C\b', text):
        imports['M_C'] = True

    return imports


def generate_imports(imports):
    """Generate Rust import statements."""
    lines = ['use cubecl::prelude::*;']

    # Constants
    consts_needed = [k for k in ['M_PI', 'M_CBRT2', 'M_CBRT3', 'M_CBRT4', 'M_CBRT5',
                                  'M_CBRT6', 'M_CBRT7', 'M_CBRT9', 'M_CBRTPI',
                                  'M_SQRTPI', 'M_SQRT2', 'M_SQRT3', 'M_C',
                                  'RS_FACTOR', 'X_FACTOR_C', 'K_FACTOR_C',
                                  'FZETAFACTOR']
                     if imports.get(k)]
    if consts_needed:
        lines.append(f'use crate::math::constants::{{{", ".join(consts_needed)}}};')

    # Power functions
    pows_needed = [k for k in ['pow_1_3', 'pow_2_3', 'pow_4_3', 'pow_5_3',
                                'pow_3_2', 'pow_1_4', 'pow_7_3', 'pow_2', 'pow_3']
                   if imports.get(k)]
    if pows_needed:
        lines.append(f'use crate::math::powers::{{{", ".join(pows_needed)}}};')

    # Piecewise functions
    pws_needed = [k for k in ['piecewise3', 'piecewise5'] if imports.get(k)]
    if pws_needed:
        lines.append(f'use crate::math::piecewise::{{{", ".join(pws_needed)}}};')

    # Erf
    if imports.get('erf_cube'):
        lines.append('use crate::math::erf::erf_cube;')

    return '\n'.join(lines)


def generate_kernel_function(func_name, level, spin, compute_lines, outputs,
                              params, is_vxc_only=False):
    """Generate a single Rust #[cube(launch_unchecked)] function."""
    is_pol = spin == 'pol'
    spin_label = 'polarized' if is_pol else 'unpolarized'

    # Build function signature
    fn_name = f'{func_name}_{level}_{spin}'

    # Output buffer parameters
    out_params = []
    if not is_vxc_only:
        out_params.append('    zk: &mut Array<f64>,')
    if level in ('vxc', 'fxc', 'kxc', 'lxc'):
        if is_vxc_only or level != 'exc':
            out_params.append('    vrho: &mut Array<f64>,')
    if level in ('fxc', 'kxc', 'lxc'):
        out_params.append('    v2rho2: &mut Array<f64>,')
    if level in ('kxc', 'lxc'):
        out_params.append('    v3rho3: &mut Array<f64>,')
    if level == 'lxc':
        out_params.append('    v4rho4: &mut Array<f64>,')

    # Parameter arguments
    param_args = []
    for p in params:
        rname = sanitize_param_name(p)
        param_args.append(f'    param_{p}: f64,')

    lines = []
    lines.append(f'/// {func_name.upper()} {level} -- {spin_label}.')
    lines.append(f'#[cube(launch_unchecked)]')
    lines.append(f'pub fn {fn_name}(')
    lines.append(f'    rho: &Array<f64>,')
    for op in out_params:
        lines.append(op)
    for pa in param_args:
        lines.append(pa)
    lines.append(f'    dens_threshold: f64,')
    lines.append(f'    #[allow(unused_variables)] zeta_threshold: f64,')
    lines.append(f') {{')

    # Determine the bounds check array
    bounds_arr = 'zk' if not is_vxc_only else 'vrho'
    lines.append(f'    let ip = ABSOLUTE_POS;')
    lines.append(f'    if ip < {bounds_arr}.len() {{')

    # For polarized functions, add rho extraction
    if is_pol:
        lines.append(f'        let rho0 = rho[ip * 2];')
        lines.append(f'        let rho1 = rho[ip * 2 + 1];')

    # Translate compute lines
    for cline in compute_lines:
        translated = translate_compute_line(cline, is_pol)
        lines.append(translated)

    # Add output accumulation
    for (field, var) in outputs:
        if is_pol and field in ('vrho', 'v2rho2', 'v3rho3', 'v4rho4'):
            # Polarized outputs may have multiple components
            # Check if there are multiple output vars like tvrho0, tvrho1
            pass
        lines.append(f'')
        lines.append(f'        {field}[ip] += {var};')

    lines.append(f'    }}')
    lines.append(f'}}')

    return '\n'.join(lines)


def process_compute_lines_for_outputs(compute_lines, output_lines, is_pol):
    """Process compute lines and organize by output sections."""
    # We need to figure out which compute lines correspond to which output
    # The pattern is: compute tzk0, output zk; compute tvrho0, output vrho; etc.
    sections = []
    current_section = []

    all_lines = []
    for line in compute_lines:
        all_lines.append(('compute', line))
    # Interleave output lines at correct positions
    # Actually, we should preserve the original order

    return compute_lines


def translate_c_to_rust(c_file_path, func_name, is_vxc_only=False):
    """Main translation function."""
    with open(c_file_path, 'r') as f:
        c_source = f.read()

    # Detect maple2c_order
    m = re.search(r'#define maple2c_order (\d+)', c_source)
    max_order = int(m.group(1)) if m else 4

    # Detect params
    params = FUNC_PARAMS.get(func_name, [])

    # Extract all functions
    functions = extract_functions(c_source)

    # Detect imports from full source
    imports = detect_used_imports(c_source)

    # Generate Rust source
    rust_lines = []
    rust_lines.append(f'//! {func_name.upper()} kernel functions translated from libxc maple2c.')
    rust_lines.append(f'//!')
    rust_lines.append(f'//! Auto-translated from `libxc-master/src/maple2c/{"lda_vxc" if is_vxc_only else "lda_exc"}/{func_name}.c`.')
    rust_lines.append(f'//! Translation preserves exact maple2c variable names and operation order.')
    rust_lines.append(f'')
    rust_lines.append(f'#[allow(unused_variables, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]')
    rust_lines.append(f'')
    rust_lines.append(generate_imports(imports))
    rust_lines.append(f'')

    levels = ['exc', 'vxc', 'fxc', 'kxc', 'lxc']
    if is_vxc_only:
        levels = ['vxc', 'fxc', 'kxc', 'lxc']

    for spin in ['unpol', 'pol']:
        is_pol = spin == 'pol'
        spin_label = 'UNPOLARIZED' if not is_pol else 'POLARIZED'
        rust_lines.append(f'// ============================================================================')
        rust_lines.append(f'// {spin_label} FUNCTIONS')
        rust_lines.append(f'// ============================================================================')
        rust_lines.append(f'')

        for level in levels:
            if max_order < {'exc': 0, 'vxc': 1, 'fxc': 2, 'kxc': 3, 'lxc': 4}[level]:
                continue

            key = (level, spin)
            if key not in functions:
                continue

            func_text = functions[key]
            var_decls, compute_lines, output_lines = parse_c_function(func_text, level, spin)
            outputs = detect_output_vars(compute_lines, output_lines)

            if not outputs:
                # Fallback: use standard output field mapping
                if is_vxc_only:
                    outputs = get_output_fields_vxc_only(level)
                else:
                    outputs = get_output_fields(level)

            # Handle polarized output indexing
            # In polarized mode, outputs like vrho have multiple components
            # tvrho0 -> vrho[ip * 2], tvrho1 -> vrho[ip * 2 + 1]
            pol_outputs = []
            if is_pol:
                for (field, var) in outputs:
                    pol_outputs.append((field, var))
                # Check for additional pol outputs (tvrho1, tv2rho21, etc.)
                for line in output_lines:
                    for extra_var in re.findall(r'\+=\s*(t\w+)', line):
                        found = False
                        for (f, v) in pol_outputs:
                            if v == extra_var:
                                found = True
                                break
                        if not found:
                            # Determine which field
                            for field in ['v4rho4', 'v3rho3', 'v2rho2', 'vrho', 'zk']:
                                if f'out->{field}' in line:
                                    pol_outputs.append((field, extra_var))
                                    break
                outputs_final = pol_outputs if pol_outputs else outputs
            else:
                outputs_final = outputs

            # Now generate the Rust function
            fn_lines = generate_rust_function(
                func_name, level, spin, compute_lines, output_lines,
                outputs_final, params, is_vxc_only, is_pol
            )
            rust_lines.append(fn_lines)
            rust_lines.append(f'')

    return '\n'.join(rust_lines)


def generate_rust_function(func_name, level, spin, compute_lines, output_lines,
                           outputs, params, is_vxc_only, is_pol):
    """Generate a complete Rust kernel function."""
    fn_name = f'{func_name}_{level}_{spin}'
    spin_label = 'polarized' if is_pol else 'unpolarized'

    # Build output buffer parameters
    out_params = []
    if not is_vxc_only or level == 'exc':
        out_params.append(('zk', True))
    else:
        # For vxc-only, no zk at any level
        pass

    # Always include zk for non-vxc-only functionals
    actual_out_params = []
    if not is_vxc_only:
        actual_out_params.append('zk')

    if level in ('vxc', 'fxc', 'kxc', 'lxc'):
        actual_out_params.append('vrho')
    if level in ('fxc', 'kxc', 'lxc'):
        actual_out_params.append('v2rho2')
    if level in ('kxc', 'lxc'):
        actual_out_params.append('v3rho3')
    if level == 'lxc':
        actual_out_params.append('v4rho4')

    lines = []
    lines.append(f'/// {func_name.upper()} {level} -- {spin_label}.')
    lines.append(f'#[cube(launch_unchecked)]')
    lines.append(f'pub fn {fn_name}(')
    lines.append(f'    rho: &Array<f64>,')
    for op in actual_out_params:
        lines.append(f'    {op}: &mut Array<f64>,')
    for p in params:
        lines.append(f'    param_{p}: f64,')
    lines.append(f'    dens_threshold: f64,')
    lines.append(f'    #[allow(unused_variables)] zeta_threshold: f64,')
    lines.append(f') {{')

    # Bounds check
    bounds_arr = actual_out_params[0] if actual_out_params else 'vrho'
    lines.append(f'    let ip = ABSOLUTE_POS;')
    lines.append(f'    if ip < {bounds_arr}.len() {{')

    # For polarized functions, extract rho components
    if is_pol:
        lines.append(f'        let rho0 = rho[ip * 2];')
        lines.append(f'        let rho1 = rho[ip * 2 + 1];')
        lines.append(f'')

    # Process compute lines in order, inserting output += at the right places
    # Parse the original interleaved structure
    all_translated = translate_body_with_outputs(compute_lines, output_lines, is_pol, is_vxc_only)
    for tl in all_translated:
        lines.append(tl)

    lines.append(f'    }}')
    lines.append(f'}}')

    return '\n'.join(lines)


def translate_body_with_outputs(compute_lines, output_lines, is_pol, is_vxc_only):
    """Translate compute lines and insert output accumulation."""
    result = []

    # Track which output vars have been seen
    output_map = {}  # var_name -> (field, index_expr)
    for oline in output_lines:
        # Parse: out->field[ip*p->dim.field + N] += var;
        m = re.search(r'out->(\w+)\[ip\*p->dim\.\w+\s*\+\s*(\d+)\]\s*\+=\s*(\w+)', oline)
        if m:
            field = m.group(1)
            idx = int(m.group(2))
            var = m.group(3)
            output_map[var] = (field, idx)

    # Process compute lines
    output_vars_seen = set()
    for cline in compute_lines:
        stripped = cline.strip().rstrip(';')
        m = re.match(r'(\w+)\s*=\s*(.*)', stripped)
        if m:
            var_name = m.group(1)
            expr = m.group(2)
            translated_expr = translate_expr(expr, '', is_pol)

            # Fix integer literals that need to be floats
            translated_expr = fix_integer_literals(translated_expr)

            result.append(f'        let {var_name} = {translated_expr};')

            # Check if this var needs output accumulation
            if var_name in output_map:
                field, idx = output_map[var_name]
                if is_pol and idx > 0:
                    dim = get_pol_dim(field)
                    result.append(f'        {field}[ip * {dim} + {idx}] += {var_name};')
                elif is_pol and idx == 0:
                    dim = get_pol_dim(field)
                    if dim > 1:
                        result.append(f'        {field}[ip * {dim}] += {var_name};')
                    else:
                        result.append(f'        {field}[ip] += {var_name};')
                else:
                    result.append(f'        {field}[ip] += {var_name};')
                result.append(f'')
                output_vars_seen.add(var_name)

    return result


def get_pol_dim(field):
    """Get the polarized dimension for an output field."""
    dims = {
        'zk': 1,
        'vrho': 2,
        'v2rho2': 3,
        'v3rho3': 4,
        'v4rho4': 5,
    }
    return dims.get(field, 1)


def fix_integer_literals(expr):
    """Fix standalone integer literals that should be f64 in CubeCL context."""
    # Replace standalone integer arguments in function calls
    # piecewise3(cond, 0, expr) -> piecewise3(cond, 0.0, expr)
    # But don't touch array indices like rho[ip * 2]

    # Handle piecewise integer args: , 0) or , 1) or , 0,
    expr = re.sub(r',\s*(\d+)\s*\)', lambda m: f', {m.group(1)}.0)', expr)
    expr = re.sub(r',\s*(\d+)\s*,', lambda m: f', {m.group(1)}.0,', expr)

    return expr


def generate_launch_wrapper(func_name, c_file_path, params, is_vxc_only=False):
    """Generate the launch wrapper file."""
    with open(c_file_path, 'r') as f:
        c_source = f.read()

    m = re.search(r'#define maple2c_order (\d+)', c_source)
    max_order = int(m.group(1)) if m else 4

    functions = extract_functions(c_source)

    lines = []
    lines.append(f'//! Safe wrapper functions for {func_name.upper()} CubeCL kernel launches.')
    lines.append(f'//!')
    lines.append(f'//! Auto-generated launch wrappers. All unsafe confined to this module.')
    lines.append(f'')
    lines.append(f'use cubecl::cpu::CpuRuntime;')
    lines.append(f'use cubecl::client::ComputeClient;')
    lines.append(f'use cubecl::prelude::*;')
    lines.append(f'')
    lines.append(f'use super::{func_name};')
    lines.append(f'use super::launch_lda_x::BufArg;')
    lines.append(f'')

    levels_start = ['exc', 'vxc', 'fxc', 'kxc', 'lxc']
    if is_vxc_only:
        levels_start = ['vxc', 'fxc', 'kxc', 'lxc']

    for spin in ['unpol', 'pol']:
        is_pol = spin == 'pol'
        spin_label = 'UNPOLARIZED' if not is_pol else 'POLARIZED'
        lines.append(f'// ============================================================================')
        lines.append(f'// {spin_label} WRAPPERS')
        lines.append(f'// ============================================================================')
        lines.append(f'')

        for level in levels_start:
            if max_order < {'exc': 0, 'vxc': 1, 'fxc': 2, 'kxc': 3, 'lxc': 4}[level]:
                continue

            key = (level, spin)
            if key not in functions:
                continue

            fn_name = f'{func_name}_{level}_{spin}'
            launch_fn = f'launch_{fn_name}'

            # Build parameter list
            buf_params = []
            if not is_vxc_only:
                buf_params.append('zk')
            if level in ('vxc', 'fxc', 'kxc', 'lxc'):
                buf_params.append('vrho')
            if level in ('fxc', 'kxc', 'lxc'):
                buf_params.append('v2rho2')
            if level in ('kxc', 'lxc'):
                buf_params.append('v3rho3')
            if level == 'lxc':
                buf_params.append('v4rho4')

            # Function signature
            lines.append(f'#[allow(clippy::too_many_arguments)]')
            lines.append(f'pub fn {launch_fn}(')
            lines.append(f'    client: &ComputeClient<CpuRuntime>,')
            lines.append(f'    cube_count: CubeCount,')
            lines.append(f'    cube_dim: CubeDim,')
            lines.append(f'    rho: &BufArg<\'_>,')
            for bp in buf_params:
                lines.append(f'    {bp}: &BufArg<\'_>,')
            for p in params:
                lines.append(f'    param_{p}: f64,')
            lines.append(f'    dens_threshold: f64,')
            lines.append(f'    zeta_threshold: f64,')
            lines.append(f') -> Result<(), Box<dyn std::error::Error>> {{')
            lines.append(f'    unsafe {{')
            lines.append(f'        {func_name}::{fn_name}::launch_unchecked::<CpuRuntime>(')
            lines.append(f'            client, cube_count, cube_dim,')
            lines.append(f'            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),')
            for bp in buf_params:
                lines.append(f'            ArrayArg::from_raw_parts::<f64>({bp}.handle, {bp}.len, 1),')
            for p in params:
                lines.append(f'            ScalarArg::new(param_{p}),')
            lines.append(f'            ScalarArg::new(dens_threshold),')
            lines.append(f'            ScalarArg::new(zeta_threshold),')
            lines.append(f'        )?;')
            lines.append(f'    }}')
            lines.append(f'    Ok(())')
            lines.append(f'}}')
            lines.append(f'')

    return '\n'.join(lines)


def main():
    if len(sys.argv) < 3:
        print("Usage: translate_lda.py <c_file> <func_name> [--vxc-only]")
        sys.exit(1)

    c_file = sys.argv[1]
    func_name = sys.argv[2]
    is_vxc_only = '--vxc-only' in sys.argv

    # Generate kernel file
    kernel_rs = translate_c_to_rust(c_file, func_name, is_vxc_only)
    print("=== KERNEL ===")
    print(kernel_rs)

    # Generate launch wrapper
    params = FUNC_PARAMS.get(func_name, [])
    launch_rs = generate_launch_wrapper(func_name, c_file, params, is_vxc_only)
    print("\n=== LAUNCH ===")
    print(launch_rs)


if __name__ == '__main__':
    main()
