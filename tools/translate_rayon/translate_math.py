#!/usr/bin/env python3
"""Port `#[cube]` math-helper modules to the plain-Rust math crate.

Attempts every module the emitted kernel tree still needs and reports which
convert mechanically and which require hand work, rather than silently emitting
something half-translated.

Usage: python3 tools/translate_rayon/translate_math.py [--write]
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from xform import UnsupportedKernel, transform_math  # noqa: E402

REPO = Path(__file__).resolve().parents[2]
SRC = REPO / "crates" / "kernels" / "math" / "src"
DST = REPO / "crates" / "kernels-rayon" / "math" / "src"

# Already ported by hand (powers carries the cbrt fix and is not a transliteration).
ALREADY = {"constants", "piecewise", "powers", "lib"}


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true")
    args = ap.parse_args()

    ok, failed = [], []
    for path in sorted(SRC.glob("*.rs")):
        if path.stem in ALREADY:
            continue
        try:
            out = transform_math(path.read_text())
        except UnsupportedKernel as exc:
            failed.append((path.stem, str(exc)))
            if args.write:
                (DST / path.name).unlink(missing_ok=True)
            continue
        ok.append(path.stem)
        if args.write:
            (DST / path.name).write_text(out)

    if args.write and ok:
        lib = (DST / "lib.rs").read_text()
        existing = {
            l.split()[-1].rstrip(";") for l in lib.splitlines() if l.startswith("pub mod ")
        }
        add = [m for m in ok if m not in existing]
        if add:
            lib = lib.rstrip() + "\n" + "\n".join(f"pub mod {m};" for m in sorted(add)) + "\n"
            (DST / "lib.rs").write_text(lib)

    print(f"converted ({len(ok)}): {', '.join(ok) or '-'}")
    print(f"needs hand work ({len(failed)}):")
    for name, why in failed:
        print(f"  {name}: {why}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
