#!/usr/bin/env python3
"""
Phase 9 Plan 09-04 regeneration driver.

For each functional currently present in EXACTLY ONE sub-crate under
`crates/kernel-{lda,gga-*,mgga-*}/src/<functional>/`, re-run the matching
translator (translate_lda_v2.py / translate_gga.py / translate_mgga.py) with
the new 18K SPLIT_THRESHOLD into a temporary staging directory, then atomically
replace the existing per-functional directory contents (preserving sub-crate
boundary per CONTEXT D-09).

Functionals that are currently distributed across multiple sub-crates (per the
post-Plan-09-03 first-fit-decreasing bin-packing) are SKIPPED to honor D-09's
"sub-crate boundaries are NOT re-bin-packed" rule. They are logged to
`log/09-04-skipped-multi-subcrate.log` and noted as a deferred architectural
decision in 09-04-SUMMARY.md.

Operation order is preserved (D-08): the translators emit the same `let`
sequence and output-write order as before; only the split boundaries between
files change.

Usage:
    python3 tools/regen_phase09.py              # regen all single-sub-crate functionals
    python3 tools/regen_phase09.py --dry-run    # report only, no writes
    python3 tools/regen_phase09.py --family lda # regen one family only
"""

import argparse
import collections
import os
import shutil
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES_DIR = os.path.join(ROOT, "crates")
MAPLE2C = os.path.join(ROOT, "libxc-master", "src", "maple2c")
LOG_DIR = os.path.join(ROOT, "log")

TRANSLATOR = {
    "lda": os.path.join(ROOT, "tools", "translate_lda_v2.py"),
    "gga": os.path.join(ROOT, "tools", "translate_gga.py"),
    "mgga": os.path.join(ROOT, "tools", "translate_mgga.py"),
}


def family_of(func_name: str) -> str:
    """Map functional name to its family (lda/gga/mgga)."""
    # hyb_*_xc_*: family is the second token
    if func_name.startswith("hyb_lda"):
        return "lda"
    if func_name.startswith("hyb_gga"):
        return "gga"
    if func_name.startswith("hyb_mgga"):
        return "mgga"
    if func_name.startswith("lda"):
        return "lda"
    if func_name.startswith("gga"):
        return "gga"
    if func_name.startswith("mgga"):
        return "mgga"
    raise ValueError(f"Unknown family for functional: {func_name}")


def find_maple2c_source(func_name: str, family: str):
    """Resolve the maple2c .c source for a functional. Returns (path, is_vxc_only)."""
    # Prefer the *_exc directory (most functionals); fall back to *_vxc with --vxc-only.
    exc_path = os.path.join(MAPLE2C, f"{family}_exc", f"{func_name}.c")
    if os.path.exists(exc_path):
        return exc_path, False
    vxc_path = os.path.join(MAPLE2C, f"{family}_vxc", f"{func_name}.c")
    if os.path.exists(vxc_path):
        return vxc_path, True
    return None, None


def scan_functional_distribution():
    """Map every functional → list of sub-crates that contain it."""
    out = collections.defaultdict(list)
    for entry in sorted(os.listdir(CRATES_DIR)):
        if not (entry.startswith("kernel-lda") or
                entry.startswith("kernel-gga") or
                entry.startswith("kernel-mgga")):
            continue
        if entry in ("kernel-gga", "kernel-mgga"):
            continue  # facade aggregators (no functionals of their own)
        # kernel-lda IS where LDA functionals live (no aggregator/sub-crate split for LDA)
        src_dir = os.path.join(CRATES_DIR, entry, "src")
        if not os.path.isdir(src_dir):
            continue
        for item in sorted(os.listdir(src_dir)):
            path = os.path.join(src_dir, item)
            if not os.path.isdir(path):
                continue
            # Skip non-functional support modules (e.g., 'deferred')
            if item == "deferred":
                continue
            out[item].append(entry)
    return dict(out)


def list_existing_files(func_dir: str):
    """Return set of .rs filenames (excluding mod.rs) in a functional directory."""
    if not os.path.isdir(func_dir):
        return set()
    return {f for f in os.listdir(func_dir)
            if f.endswith(".rs") and f != "mod.rs"}


def run_translator(family: str, c_path: str, func_name: str,
                   write_to: str, vxc_only: bool, threshold: int = 18000):
    """Run the translator subprocess and return (returncode, stdout, stderr)."""
    cmd = [sys.executable, TRANSLATOR[family], c_path, func_name,
           "--write-to", write_to, "--split", "--incremental"]
    if vxc_only:
        cmd.append("--vxc-only")
    if family == "gga":
        cmd.extend(["--split-threshold", str(threshold)])
    # LDA + MGGA translators do not have --split-threshold CLI flag; they use
    # the SPLIT_THRESHOLD module-level constant (already raised to 18000).
    proc = subprocess.run(cmd, capture_output=True, text=True)
    return proc.returncode, proc.stdout, proc.stderr


def regen_functional(func_name: str, sub_crate: str, log_file, dry_run: bool):
    """Regenerate a single functional in place. Returns dict with status info."""
    family = family_of(func_name)
    c_path, vxc_only = find_maple2c_source(func_name, family)
    if c_path is None:
        log_file.write(f"NO_C_SOURCE: {func_name} (family={family})\n")
        return {"status": "no_c_source", "func": func_name}

    # Some hyb_*-prefixed functionals have a different family for the c source
    # (e.g., hyb_lda_xc_bn05.c lives under lda_exc/). Normalize:
    if func_name.startswith("hyb_lda"):
        family = "lda"
    elif func_name.startswith("hyb_gga"):
        family = "gga"
    elif func_name.startswith("hyb_mgga"):
        family = "mgga"

    target_func_dir = os.path.join(CRATES_DIR, sub_crate, "src", func_name)
    pre_files = list_existing_files(target_func_dir)

    with tempfile.TemporaryDirectory(prefix=f"regen-{func_name}-") as staging:
        rc, stdout, stderr = run_translator(
            family, c_path, func_name, staging, vxc_only)
        if rc != 0:
            log_file.write(f"TRANSLATE_FAIL: {func_name} rc={rc}\n"
                           f"  stderr: {stderr.strip()}\n")
            return {"status": "translate_fail", "func": func_name,
                    "stderr": stderr.strip()}

        staged_func = os.path.join(staging, func_name)
        if not os.path.isdir(staged_func):
            log_file.write(f"NO_STAGING_OUTPUT: {func_name}\n")
            return {"status": "no_staging_output", "func": func_name}

        post_files = list_existing_files(staged_func)
        added = post_files - pre_files
        removed = pre_files - post_files
        unchanged = pre_files & post_files

        if dry_run:
            log_file.write(
                f"DRY_RUN: {func_name} → {sub_crate} "
                f"pre={len(pre_files)} post={len(post_files)} "
                f"added={len(added)} removed={len(removed)} unchanged={len(unchanged)}\n")
            return {"status": "dry_run", "func": func_name,
                    "pre": len(pre_files), "post": len(post_files),
                    "added": sorted(added), "removed": sorted(removed)}

        # Atomic-ish replace: remove old .rs files (except mod.rs which we'll
        # overwrite), then copy new ones.
        for fname in pre_files:
            old = os.path.join(target_func_dir, fname)
            if os.path.exists(old):
                os.remove(old)
        # Ensure target dir exists
        os.makedirs(target_func_dir, exist_ok=True)
        # Copy new .rs files (and mod.rs)
        for entry in os.listdir(staged_func):
            src = os.path.join(staged_func, entry)
            dst = os.path.join(target_func_dir, entry)
            if os.path.isfile(src):
                shutil.copy2(src, dst)

        log_file.write(
            f"OK: {func_name} → {sub_crate} "
            f"pre={len(pre_files)} post={len(post_files)} "
            f"added={len(added)} removed={len(removed)}\n")
        return {"status": "ok", "func": func_name,
                "pre": len(pre_files), "post": len(post_files),
                "added": sorted(added), "removed": sorted(removed)}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--family", choices=["lda", "gga", "mgga", "all"],
                        default="all")
    args = parser.parse_args()

    os.makedirs(LOG_DIR, exist_ok=True)
    distribution = scan_functional_distribution()

    single_subcrate = {f: cs[0] for f, cs in distribution.items() if len(cs) == 1}
    multi_subcrate = {f: cs for f, cs in distribution.items() if len(cs) > 1}

    # Filter by family if requested
    if args.family != "all":
        single_subcrate = {
            f: c for f, c in single_subcrate.items() if family_of(f) == args.family
        }

    # Skip log
    skip_log_path = os.path.join(LOG_DIR, "09-04-skipped-multi-subcrate.log")
    with open(skip_log_path, "w") as skip_log:
        skip_log.write(f"# Multi-sub-crate functionals SKIPPED in regen (D-09 preservation)\n")
        skip_log.write(f"# Total: {len(multi_subcrate)} functionals\n")
        skip_log.write(f"# These were distributed across multiple sub-crates by\n")
        skip_log.write(f"# tools/split_oversized_kernel.py for compile-OOM mitigation.\n")
        skip_log.write(f"# Regenerating them at the new 18K threshold would produce a\n")
        skip_log.write(f"# different (smaller) file set that cannot fit the existing\n")
        skip_log.write(f"# multi-sub-crate distribution without violating D-09.\n#\n")
        for f, cs in sorted(multi_subcrate.items()):
            skip_log.write(f"{f}: {len(cs)} sub-crates ({','.join(cs)})\n")
    print(f"Skip log written: {skip_log_path}")
    print(f"Skipped {len(multi_subcrate)} multi-sub-crate functionals.")

    # Per-family regen log
    by_family = collections.defaultdict(list)
    for f, c in single_subcrate.items():
        by_family[family_of(f)].append((f, c))

    summary = {"ok": 0, "fail": 0, "no_c_source": 0,
               "translate_fail": 0, "dry_run": 0}

    for family in ["lda", "gga", "mgga"]:
        if args.family != "all" and args.family != family:
            continue
        family_log = os.path.join(LOG_DIR, f"09-04-regen-{family}.log")
        with open(family_log, "w") as flog:
            flog.write(f"# Phase 9 Plan 09-04: regen {family.upper()} (single-sub-crate functionals)\n")
            flog.write(f"# Threshold: 18000 lines per #[cube] function\n")
            flog.write(f"# Mode: {'dry-run' if args.dry_run else 'live'}\n#\n")
            for func_name, sub_crate in sorted(by_family.get(family, [])):
                print(f"  [{family}] {func_name} → {sub_crate}", flush=True)
                result = regen_functional(func_name, sub_crate, flog, args.dry_run)
                if result["status"] == "ok":
                    summary["ok"] += 1
                elif result["status"] == "dry_run":
                    summary["dry_run"] += 1
                else:
                    summary["fail"] += 1
                    summary[result["status"]] = summary.get(result["status"], 0) + 1
        print(f"  → {family_log}")

    print()
    print("Summary:")
    for k, v in sorted(summary.items()):
        print(f"  {k}: {v}")


if __name__ == "__main__":
    main()
