#!/usr/bin/env python3
"""
Batch translate all LDA kernel files from C to Rust.

This generates:
1. src/kernel/lda/{name}.rs  - kernel file with #[cube] functions
2. src/kernel/lda/launch_{name}.rs - launch wrapper file
3. Updates src/kernel/lda/mod.rs with module declarations
"""

import os
import sys
import subprocess

# Project root
PROJECT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
C_DIR = os.path.join(PROJECT, 'libxc-master', 'src', 'maple2c', 'lda_exc')
C_VXC_DIR = os.path.join(PROJECT, 'libxc-master', 'src', 'maple2c', 'lda_vxc')
KERNEL_DIR = os.path.join(PROJECT, 'src', 'kernel', 'lda')

# Import the translator
sys.path.insert(0, os.path.join(PROJECT, 'tools'))
import translate_lda as tl

# All LDA functionals to translate (lda_x already done)
LDA_FUNCS = [
    'lda_c_1d_csc', 'lda_c_1d_loos', 'lda_c_2d_amgb', 'lda_c_2d_prm',
    'lda_c_chachiyo', 'lda_c_chachiyo_mod', 'lda_c_gk72', 'lda_c_gombas',
    'lda_c_hl', 'lda_c_lp96', 'lda_c_ml1', 'lda_c_pk09', 'lda_c_pmgb06',
    'lda_c_pw', 'lda_c_pw_erf', 'lda_c_pz', 'lda_c_rc04', 'lda_c_rpa',
    'lda_c_vwn', 'lda_c_vwn_1', 'lda_c_vwn_2', 'lda_c_vwn_3', 'lda_c_vwn_4',
    'lda_c_vwn_rpa', 'lda_c_w20', 'lda_c_wigner',
    'lda_k_gds08_worker', 'lda_k_tf', 'lda_k_zlp',
    'lda_x_1d_exponential', 'lda_x_1d_soft', 'lda_x_2d', 'lda_x_erf',
    'lda_x_rel', 'lda_x_sloc', 'lda_x_yukawa',
    'lda_xc_1d_ehwlrg', 'lda_xc_ksdt', 'lda_xc_teter93', 'lda_xc_zlp',
    'hyb_lda_xc_bn05',
]

# Special vxc-only functional
VXC_ONLY = ['lda_xc_tih']


def translate_functional(func_name, is_vxc_only=False):
    """Translate a single functional."""
    if is_vxc_only:
        c_file = os.path.join(C_VXC_DIR, f'{func_name}.c')
    else:
        c_file = os.path.join(C_DIR, f'{func_name}.c')

    if not os.path.exists(c_file):
        print(f"  SKIP: {c_file} not found")
        return False

    try:
        # Generate kernel file
        kernel_rs = tl.translate_c_to_rust(c_file, func_name, is_vxc_only)
        kernel_path = os.path.join(KERNEL_DIR, f'{func_name}.rs')
        with open(kernel_path, 'w') as f:
            f.write(kernel_rs)

        # Generate launch wrapper
        params = tl.FUNC_PARAMS.get(func_name, [])
        launch_rs = tl.generate_launch_wrapper(func_name, c_file, params, is_vxc_only)
        launch_path = os.path.join(KERNEL_DIR, f'launch_{func_name}.rs')
        with open(launch_path, 'w') as f:
            f.write(launch_rs)

        print(f"  OK: {func_name}")
        return True
    except Exception as e:
        print(f"  ERROR: {func_name}: {e}")
        import traceback
        traceback.print_exc()
        return False


def generate_mod_rs():
    """Generate the mod.rs with all module declarations."""
    all_funcs = ['lda_x'] + sorted(LDA_FUNCS) + sorted(VXC_ONLY)

    lines = ['// LDA kernel implementations.']
    for name in all_funcs:
        lines.append(f'pub mod {name};')
        lines.append(f'pub mod launch_{name};')

    mod_path = os.path.join(KERNEL_DIR, 'mod.rs')
    with open(mod_path, 'w') as f:
        f.write('\n'.join(lines) + '\n')

    print(f"  Updated mod.rs with {len(all_funcs)} functionals")


def main():
    print("=== Batch LDA Translation ===")
    print(f"Kernel dir: {KERNEL_DIR}")
    print()

    success = 0
    failed = 0

    # Translate regular LDA functionals
    for name in LDA_FUNCS:
        ok = translate_functional(name)
        if ok:
            success += 1
        else:
            failed += 1

    # Translate vxc-only functionals
    for name in VXC_ONLY:
        ok = translate_functional(name, is_vxc_only=True)
        if ok:
            success += 1
        else:
            failed += 1

    print()
    print(f"=== Results: {success} OK, {failed} FAILED ===")

    # Generate mod.rs
    print()
    generate_mod_rs()

    return 0 if failed == 0 else 1


if __name__ == '__main__':
    sys.exit(main())
