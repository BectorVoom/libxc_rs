#!/usr/bin/env python3
"""Extract libxc `ext_params` defaults for each functional.

The kernels take their per-functional constants as ordinary `f64` arguments
(`param_kappa`, `param_mu`, ...). libxc keeps the defaults in C:

    #define PBE_N_PAR 2
    static const char  *pbe_names[PBE_N_PAR]  = {"_kappa", "_mu"};
    static const double pbe_values[PBE_N_PAR] = {0.8040, 0.2195149727645171};
    const xc_func_info_type xc_func_info_gga_x_pbe = {
      ...
      {PBE_N_PAR, pbe_names, pbe_desc, pbe_values, set_ext_params_cpy},
    };

This reads those out and pairs them with the kernel's parameter list.

Safety rules -- a functional is emitted only if all hold, otherwise it is
reported as unresolved rather than guessed at:

  1. Its `xc_func_info_` block names both a `_names` and a `_values` array.
  2. The setter is `set_ext_params_cpy`. Other setters transform the values on
     the way into the params struct, so a straight copy would be wrong.
  3. Every value parses as a plain float literal (no C expressions).
  4. `{"param" + n for n in names}` matches the kernel's parameter set exactly.
     This is the real check: it catches any case where maple2c named things
     differently from the ext_params table.

Usage: python3 tools/translate_rayon/extract_params.py [--json OUT]
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
LIBXC_SRC = REPO / "libxc-master" / "src"
KERNELS = REPO / "crates" / "kernels-rayon"

_NAMES_RE = re.compile(
    r"static\s+const\s+char\s*\*\s*(\w+)\s*\[[^\]]*\]\s*=\s*\{(.*?)\}\s*;", re.S
)
_VALUES_RE = re.compile(
    r"static\s+const\s+double\s+(\w+)\s*\[[^\]]*\]\s*=\s*\{(.*?)\}\s*;", re.S
)
_INFO_RE = re.compile(
    r"const\s+xc_func_info_type\s+xc_func_info_(\w+)\s*=\s*\{(.*?)\n\};", re.S
)
_EXT_RE = re.compile(r"\{\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*\}")
_FLOAT_RE = re.compile(r"^[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?$")


def strip_comments(src: str) -> str:
    src = re.sub(r"/\*.*?\*/", "", src, flags=re.S)
    return re.sub(r"//[^\n]*", "", src)


def load_libxc() -> tuple[dict, dict, dict]:
    """Return (names, values, infos), all keyed per source file.

    Array identifiers are file-local in libxc -- a great many translation units
    declare plain `names[N_PAR]` and `desc[N_PAR]` -- so a single global map
    lets the last file parsed win and silently pairs one functional's names
    with another's values. Everything is therefore scoped by file, and lookups
    resolve only within the file that declared the `xc_func_info_` block.
    """
    names, values, infos = {}, {}, {}
    for c in sorted(LIBXC_SRC.glob("*.c")):
        key = c.name
        src = strip_comments(c.read_text(errors="replace"))
        names[key] = {
            m.group(1): re.findall(r'"([^"]*)"', m.group(2))
            for m in _NAMES_RE.finditer(src)
        }
        values[key] = {
            m.group(1): [x.strip() for x in m.group(2).split(",") if x.strip()]
            for m in _VALUES_RE.finditer(src)
        }
        for m in _INFO_RE.finditer(src):
            infos[m.group(1)] = (key, m.group(2))
    return names, values, infos


def kernel_params(fam: str, func: str) -> list[str] | None:
    d = KERNELS / fam / func / "src"
    for order in ("exc_unpol", "vxc_unpol", "fxc_unpol", "exc_pol", "vxc_pol"):
        f = d / f"{order}.rs"
        if not f.is_file():
            continue
        m = re.search(r"pub fn \w+\(\n(.*?)\n\) \{", f.read_text(), re.S)
        if not m:
            continue
        out = []
        for line in m.group(1).split("\n"):
            line = line.strip().rstrip(",")
            if not line or "&" in line:
                continue
            nm, _, ty = line.partition(":")
            nm, ty = nm.strip(), ty.strip()
            if ty == "f64" and nm not in ("dens_threshold", "zeta_threshold"):
                out.append(nm)
        return out
    return None


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--json", default=None)
    args = ap.parse_args()

    names, values, infos = load_libxc()
    resolved, unresolved = {}, {}

    for fam in ("lda", "gga", "mgga"):
        fam_dir = KERNELS / fam
        if not fam_dir.is_dir():
            continue
        for d in sorted(fam_dir.iterdir()):
            if not d.is_dir():
                continue
            func = d.name
            kp = kernel_params(fam, func)
            if kp is None:
                continue
            if not kp:
                resolved[func] = {"family": fam, "params": [], "values": []}
                continue

            entry = infos.get(func)
            if entry is None:
                unresolved[func] = "no xc_func_info_ block in libxc source"
                continue
            srcfile, info = entry
            m = _EXT_RE.search(info)
            if not m:
                unresolved[func] = "no ext_params tuple in xc_func_info_"
                continue
            _npar, nm_arr, _desc, val_arr, setter = m.groups()
            if setter != "set_ext_params_cpy":
                unresolved[func] = f"setter is {setter}, not a plain copy"
                continue
            file_names = names.get(srcfile, {})
            file_values = values.get(srcfile, {})
            if nm_arr not in file_names or val_arr not in file_values:
                unresolved[func] = (
                    f"names/values array not found in {srcfile} ({nm_arr}, {val_arr})"
                )
                continue

            libnames, libvals = file_names[nm_arr], file_values[val_arr]
            if len(libnames) != len(libvals):
                unresolved[func] = f"names/values length mismatch ({len(libnames)} vs {len(libvals)})"
                continue
            if not all(_FLOAT_RE.match(v) for v in libvals):
                bad = [v for v in libvals if not _FLOAT_RE.match(v)]
                unresolved[func] = f"non-literal value(s): {bad[:3]}"
                continue

            mapping = {f"param{n}": v for n, v in zip(libnames, libvals)}
            if set(mapping) != set(kp):
                only_k = sorted(set(kp) - set(mapping))
                only_l = sorted(set(mapping) - set(kp))
                unresolved[func] = (
                    f"param set mismatch; kernel-only={only_k[:4]} libxc-only={only_l[:4]}"
                )
                continue

            resolved[func] = {
                "family": fam,
                "params": kp,
                "values": [mapping[p] for p in kp],
            }

    n_paramless = sum(1 for v in resolved.values() if not v["params"])
    n_wired = len(resolved) - n_paramless
    print(f"resolved   : {len(resolved)}  ({n_paramless} paramless, {n_wired} with defaults)")
    print(f"unresolved : {len(unresolved)}")
    if unresolved:
        reasons = {}
        for f, why in unresolved.items():
            key = why.split(";")[0].split("(")[0].strip()
            reasons.setdefault(key, []).append(f)
        print("\nunresolved by reason:")
        for key, fs in sorted(reasons.items(), key=lambda kv: -len(kv[1])):
            print(f"  {len(fs):3d}  {key}")
            print(f"       e.g. {', '.join(fs[:4])}")

    if args.json:
        Path(args.json).write_text(json.dumps(
            {"resolved": resolved, "unresolved": unresolved}, indent=1))
        print(f"\nwrote {args.json}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
