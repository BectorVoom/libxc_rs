#!/usr/bin/env python3
"""
Batch translate all GGA maple2c C kernel files to Rust.

Uses translate_gga.py for each functional. All GGA kernels use split mode
(one file per derivative order x spin) to avoid LLVM stack overflow in debug builds.
"""

import os
import sys
import subprocess
import glob

GGA_EXC_DIR = "libxc-master/src/maple2c/gga_exc"
GGA_VXC_DIR = "libxc-master/src/maple2c/gga_vxc"
OUTPUT_DIR = "src/kernel/gga"
TOOL = "tools/translate_gga.py"


def get_func_name(c_file):
    """Extract functional name from C file path."""
    return os.path.splitext(os.path.basename(c_file))[0]


def translate_all():
    # Collect all C files
    exc_files = sorted(glob.glob(os.path.join(GGA_EXC_DIR, "*.c")))
    vxc_files = sorted(glob.glob(os.path.join(GGA_VXC_DIR, "*.c")))

    all_funcs = []

    # Translate exc files
    for c_file in exc_files:
        func_name = get_func_name(c_file)
        print(f"Translating {func_name}...", end=" ", flush=True)

        result = subprocess.run(
            [sys.executable, TOOL, c_file, func_name, "--write-to", OUTPUT_DIR, "--split"],
            capture_output=True, text=True
        )
        if result.returncode != 0:
            print(f"FAILED: {result.stderr.strip()}")
            continue

        n_files = len(result.stdout.strip().split("\n"))
        print(f"ok ({n_files} files)")
        all_funcs.append(func_name)

    # Translate vxc files (special: --vxc-only)
    for c_file in vxc_files:
        func_name = get_func_name(c_file)
        print(f"Translating {func_name} (vxc-only)...", end=" ", flush=True)

        result = subprocess.run(
            [sys.executable, TOOL, c_file, func_name, "--write-to", OUTPUT_DIR, "--split", "--vxc-only"],
            capture_output=True, text=True
        )
        if result.returncode != 0:
            print(f"FAILED: {result.stderr.strip()}")
            continue

        n_files = len(result.stdout.strip().split("\n"))
        print(f"ok ({n_files} files)")
        all_funcs.append(func_name)

    # Generate mod.rs
    mod_path = os.path.join(OUTPUT_DIR, "mod.rs")
    with open(mod_path, "w") as f:
        f.write("//! GGA kernel translations from maple2c.\n")
        f.write("//!\n")
        f.write(f"//! Auto-generated: {len(all_funcs)} GGA functionals.\n")
        f.write("\n")
        for func_name in sorted(all_funcs):
            f.write(f"pub mod {func_name};\n")
        f.write("")

    print(f"\nDone: {len(all_funcs)} functionals translated")
    print(f"Module file: {mod_path}")


if __name__ == "__main__":
    translate_all()
