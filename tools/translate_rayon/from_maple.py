#!/usr/bin/env python3
"""Emit rayon kernels directly from libxc's maple2c C sources.

Replaces the old three-stage pipeline
(`archive/kernels-cubecl` -> `xform.py` -> `flatten.py` -> `vnmerge.py`), which
transformed an archived CubeCL tree that no longer exists.

Why generating from C is *better*, not merely equivalent
--------------------------------------------------------

The CubeCL tree was never the real source; it was itself generated from these
same maple2c files. Everything the old pipeline had to undo was damage the
CubeCL emitter had done to get under `cubecl-macros`' memory ceiling:

* it split each function into `partN` pieces, re-deriving every shared
  intermediate in each piece (2-16x redundant arithmetic) -- `vnmerge.py`
  existed to value-number that back together;
* it fanned the pieces out across 231,749 `chunkN.rs` helper files --
  `flatten.py` existed to inline those back;
* it spilled five MGGA functionals across 39 `_pN` companion crates.

maple2c emits **one fully common-subexpression-eliminated function per
(order, spin)** with no duplication at all. Reading it directly means none of
those passes are needed and the 39 shard crates disappear: 305 crates -> 266.

What this does not change
-------------------------

The transform is still purely structural. Every expression keeps maple2c's
exact operand order and grouping, so floating-point results are unchanged --
which is what `AGENTS.md` requires of a maple2c translation. Nothing here
reassociates, factors, or reorders arithmetic.

Correctness posture
-------------------

The C vocabulary is small and closed, so this refuses to guess. After
translating an expression, every identifier left in it must be a known local,
parameter, input, constant or math-crate helper; anything else raises
`Untranslatable` and the functional is reported rather than emitted with a
silent mistranslation.

Usage:
    python3 tools/translate_rayon/from_maple.py --all
    python3 tools/translate_rayon/from_maple.py --func gga_x_pbe --dry-run
"""
from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import sys

import simd as simd_mod
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
MAPLE = REPO / "libxc-master" / "src" / "maple2c"
OUT = REPO / "crates" / "kernels-rayon"

ORDERS = ["exc", "vxc", "fxc", "kxc", "lxc"]

# (functional, order, spin) emitted as explicit `wide::f64x8` SIMD instead of
# scalar. This is an allowlist, not a policy, because a functional qualifies
# only if BOTH its speedup and its accuracy have been measured -- see the table
# in simd.py. Adding one without measuring risks a silent slowdown (the kernels
# already auto-vectorise 8-wide) or a silent accuracy loss (the derivative
# expressions amplify wide's ~1 ulp transcendentals by orders of magnitude, and
# by how much is a property of the formula).
SIMD_EXACT_FUNCS = {
    # The most libm-heavy routed kernels:
    # All transcendentals are mapped to bit-exact libxc_rkernel_math::simd forms.
    ("lda_c_vwn", "vxc", "unpol"),
    ("lda_c_vwn", "exc", "unpol"),
    ("lda_c_vwn", "vxc", "pol"),
    ("lda_c_vwn", "exc", "pol"),
    ("lda_c_vwn_1", "vxc", "unpol"),
    ("lda_c_vwn_1", "exc", "unpol"),
    ("lda_c_vwn_2", "vxc", "unpol"),
    ("lda_c_vwn_2", "exc", "unpol"),
    ("lda_c_vwn_3", "vxc", "unpol"),
    ("lda_c_vwn_3", "exc", "unpol"),
    ("lda_c_vwn_4", "vxc", "unpol"),
    ("lda_c_vwn_4", "exc", "unpol"),
    ("lda_c_vwn_rpa", "vxc", "unpol"),
    ("lda_c_vwn_rpa", "exc", "unpol"),
    ("lda_c_w20", "vxc", "unpol"),
    ("lda_c_w20", "exc", "unpol"),
    ("gga_c_lyp", "vxc", "unpol"),
    ("gga_c_lyp", "exc", "unpol"),
    ("gga_c_zvpbeloc", "vxc", "unpol"),
    ("gga_c_zvpbeloc", "exc", "unpol"),
    ("gga_c_gaploc", "vxc", "unpol"),
    ("gga_c_gaploc", "exc", "unpol"),
    ("gga_xc_th2", "vxc", "unpol"),
    ("gga_xc_th2", "exc", "unpol"),
    ("mgga_c_tpssloc", "vxc", "unpol"),
    ("mgga_c_tpssloc", "exc", "unpol"),
    ("mgga_c_scan", "vxc", "unpol"),
    ("mgga_c_scan", "exc", "unpol"),
    ("mgga_c_rregtm", "vxc", "unpol"),
    ("mgga_c_rregtm", "exc", "unpol"),
    ("mgga_c_r2scan", "vxc", "unpol"),
    ("mgga_c_r2scan", "exc", "unpol"),
    ("mgga_c_revscan", "vxc", "unpol"),
    ("mgga_c_revscan", "exc", "unpol"),
    ("mgga_c_kcis", "vxc", "unpol"),
    ("mgga_c_kcis", "exc", "unpol"),
    ("mgga_c_kcisk", "vxc", "unpol"),
    ("mgga_c_kcisk", "exc", "unpol"),
    ("mgga_k_pc07", "vxc", "unpol"),
    ("mgga_k_pc07", "exc", "unpol"),
    ("mgga_x_scan", "vxc", "unpol"),
    ("mgga_x_scan", "exc", "unpol"),
    ("mgga_x_scan", "vxc", "pol"),
    ("mgga_x_scan", "exc", "pol"),
    ("mgga_x_rscan", "vxc", "unpol"),
    ("mgga_x_rscan", "exc", "unpol"),
    # Added by tools/translate_rayon/simd_qualify.py; each line's
    # ratio is sweep ns/pt before -> after, fingerprint unchanged.
    ("mgga_c_rscan", "exc", "unpol"),  # 2.06x  (20.16 -> 9.81 ns/pt)
    ("mgga_c_rscan", "vxc", "unpol"),  # 2.31x  (27.52 -> 11.89 ns/pt)
    ("mgga_x_r2scan", "exc", "unpol"),  # 1.50x  (14.14 -> 9.40 ns/pt)
    ("mgga_x_r2scan", "vxc", "unpol"),  # 2.04x  (21.69 -> 10.64 ns/pt)
    # Added by tools/translate_rayon/simd_qualify.py; each line's
    # ratio is sweep ns/pt before -> after, fingerprint unchanged.
    ("mgga_x_tpss", "exc", "unpol"),  # 1.59x  (11.90 -> 7.49 ns/pt)
    ("mgga_x_tpss", "vxc", "unpol"),  # 1.86x  (18.26 -> 9.82 ns/pt)
    # --- PBE (2026-09-03) --------------------------------------------------
    # These OVERTURN a standing rejection, and the reason matters more than the
    # numbers. `gga_x_pbe` was rejected at 0.55x and `gga_x_b88` at 0.96x on the
    # grounds that "LLVM already vectorises them, so forcing SIMD is a
    # regression". That was true *of the tree it was measured on*: back then
    # `pow_1_3` was `powers.rs::cbrt_f64`, a branch-free inline polynomial +
    # Halley + Newton sequence with no call in it, which LLVM happily packed
    # 8-wide along with the rest of the loop.
    #
    # Commit 31fd1ff47f ("replace math with rmath across kernels") repointed
    # `safe_cbrt` at `rmath::cbrt`, and 4395787e90 pinned it to `BitExact`.
    # That is the correct thing numerically -- measured here over 2M physical
    # inputs, `rmath::cbrt` is bit-identical to `f64::cbrt`/glibc on 100% of
    # them, which the old inline version was not -- but it is an opaque
    # ~9.6 ns/elem call, and a call in the grid loop stops the loop
    # vectorising. Every kernel that had been carried by the inline cbrt lost
    # its vectorisation silently: `gga_x_b88` sweep went 2.18 -> 9.45 ns/pt
    # against an unchanged libxc, and the rejection numbers above stopped
    # describing this tree.
    #
    # Explicit SIMD restores it, because `simd::cbrt` is a real vector kernel
    # rather than 8 scalar calls. Fingerprints are unchanged on every triple
    # (bit-exact, as the whole `simd::` surface is), so these are pure wins.
    # `gga_x_pbe` has no transcendental at all besides cbrt, which is why it
    # shows the mechanism most cleanly.
    #
    # ratio is sweep ns/pt before -> after; libxc-Nt on the same run in ().
    ("gga_x_pbe", "exc", "unpol"),
    ("gga_x_pbe", "vxc", "unpol"),  # 1.81x  (7.32 -> 4.04)   libxc 4.84
    ("gga_x_pbe", "exc", "pol"),
    ("gga_x_pbe", "vxc", "pol"),  # 2.04x  (15.33 -> 7.53)  libxc 21.26
    ("gga_x_pbe", "fxc", "unpol"),  # 1.93x  (9.29 -> 4.81)   libxc 7.82
    ("gga_c_pbe", "exc", "unpol"),
    ("gga_c_pbe", "vxc", "unpol"),  # 2.56x  (15.30 -> 5.97)  libxc 15.49
    ("gga_c_pbe", "exc", "pol"),
    ("gga_c_pbe", "vxc", "pol"),  # 2.55x  (25.67 -> 10.05) libxc 30.19
    ("gga_c_pbe", "fxc", "unpol"),  # 1.76x  (22.34 -> 12.71) libxc 34.65
    # Added by tools/translate_rayon/simd_qualify.py; each line's
    # ratio is sweep ns/pt before -> after, fingerprint unchanged.
    ("gga_c_hcth_a", "exc", "unpol"),  # 1.44x  (14.61 -> 10.13 ns/pt)
    ("gga_c_hcth_a", "vxc", "unpol"),  # 1.78x  (22.32 -> 12.54 ns/pt)
    ("gga_c_lm", "exc", "unpol"),  # 1.72x  (11.36 -> 6.62 ns/pt)
    ("gga_c_lm", "vxc", "unpol"),  # 1.90x  (13.04 -> 6.86 ns/pt)
    ("gga_c_lypr", "vxc", "unpol"),  # 1.70x  (14.78 -> 8.72 ns/pt)
    ("gga_c_optc", "exc", "unpol"),  # 2.22x  (22.35 -> 10.08 ns/pt)
    ("gga_c_optc", "vxc", "unpol"),  # 2.17x  (29.16 -> 13.40 ns/pt)
    ("gga_c_p86vwn", "exc", "unpol"),  # 1.57x  (13.52 -> 8.59 ns/pt)
    ("gga_c_p86vwn", "vxc", "unpol"),  # 1.75x  (17.27 -> 9.87 ns/pt)
    ("gga_c_pbe_erf_gws", "exc", "unpol"),  # 2.15x  (18.33 -> 8.53 ns/pt)
    ("gga_c_pbe_erf_gws", "vxc", "unpol"),  # 2.52x  (26.26 -> 10.43 ns/pt)
    ("gga_c_pbe_vwn", "exc", "unpol"),  # 1.57x  (17.13 -> 10.93 ns/pt)
    ("gga_c_pw91", "exc", "unpol"),  # 1.89x  (14.00 -> 7.40 ns/pt)
    ("gga_c_pw91", "vxc", "unpol"),  # 2.00x  (17.24 -> 8.64 ns/pt)
    ("gga_c_q2d", "vxc", "unpol"),  # 2.52x  (28.96 -> 11.47 ns/pt)
    ("gga_c_revtca", "exc", "unpol"),  # 1.44x  (7.98 -> 5.53 ns/pt)
    ("gga_c_revtca", "vxc", "unpol"),  # 1.52x  (10.02 -> 6.60 ns/pt)
    ("gga_c_sg4", "exc", "unpol"),  # 1.86x  (13.58 -> 7.30 ns/pt)
    ("gga_c_sg4", "vxc", "unpol"),  # 2.19x  (18.40 -> 8.39 ns/pt)
    ("gga_c_tca", "exc", "unpol"),  # 1.43x  (7.98 -> 5.60 ns/pt)
    ("gga_c_tca", "vxc", "unpol"),  # 1.70x  (11.15 -> 6.54 ns/pt)
    ("gga_c_zpbeint", "exc", "unpol"),  # 1.85x  (12.38 -> 6.68 ns/pt)
    ("gga_c_zpbeint", "vxc", "unpol"),  # 2.09x  (16.37 -> 7.83 ns/pt)
    ("gga_c_zvpbeint", "exc", "unpol"),  # 1.93x  (12.47 -> 6.46 ns/pt)
    ("gga_c_zvpbeint", "vxc", "unpol"),  # 2.18x  (15.86 -> 7.28 ns/pt)
    ("gga_k_lc94", "exc", "unpol"),  # 1.69x  (10.13 -> 5.99 ns/pt)
    ("gga_k_lc94", "vxc", "unpol"),  # 1.89x  (12.39 -> 6.54 ns/pt)
    ("gga_k_lkt", "vxc", "unpol"),  # 1.92x  (12.75 -> 6.65 ns/pt)
    ("gga_x_airy", "exc", "unpol"),  # 1.79x  (13.36 -> 7.45 ns/pt)
    ("gga_x_airy", "vxc", "unpol"),  # 1.76x  (26.49 -> 15.06 ns/pt)
    ("gga_x_b88", "exc", "unpol"),  # 1.44x  (8.54 -> 5.91 ns/pt)
    ("gga_x_b88", "vxc", "unpol"),  # 1.65x  (10.34 -> 6.26 ns/pt)
    ("gga_x_ityh", "exc", "unpol"),  # 1.73x  (12.95 -> 7.49 ns/pt)
    ("gga_x_ityh", "vxc", "unpol"),  # 2.03x  (17.47 -> 8.61 ns/pt)
    ("gga_x_ityh_optx", "exc", "unpol"),  # 1.76x  (12.21 -> 6.95 ns/pt)
    ("gga_x_ityh_optx", "vxc", "unpol"),  # 1.88x  (15.71 -> 8.35 ns/pt)
    ("gga_x_ityh_pbe", "exc", "unpol"),  # 1.57x  (14.92 -> 9.49 ns/pt)
    ("gga_x_ityh_pbe", "vxc", "unpol"),  # 1.74x  (19.69 -> 11.29 ns/pt)
    ("gga_x_lag", "exc", "unpol"),  # 1.84x  (9.88 -> 5.38 ns/pt)
    ("gga_x_lag", "vxc", "unpol"),  # 1.68x  (14.39 -> 8.55 ns/pt)
    ("gga_x_lg93", "vxc", "unpol"),  # 1.83x  (10.62 -> 5.82 ns/pt)
    ("gga_x_ncap", "exc", "unpol"),  # 2.01x  (10.55 -> 5.26 ns/pt)
    ("gga_x_ncap", "vxc", "unpol"),  # 2.27x  (13.53 -> 5.96 ns/pt)
    ("gga_x_pbe_erf_gws", "vxc", "unpol"),  # 2.19x  (24.36 -> 11.12 ns/pt)
    ("gga_x_pbepow", "vxc", "unpol"),  # 1.82x  (9.97 -> 5.47 ns/pt)
    ("gga_x_pw91", "exc", "unpol"),  # 1.57x  (11.56 -> 7.38 ns/pt)
    ("gga_x_pw91", "vxc", "unpol"),  # 1.79x  (13.58 -> 7.59 ns/pt)
    ("gga_x_q2d", "vxc", "unpol"),  # 1.90x  (10.81 -> 5.69 ns/pt)
    ("gga_x_sfat", "exc", "unpol"),  # 1.83x  (20.57 -> 11.21 ns/pt)
    ("gga_x_sfat", "vxc", "unpol"),  # 2.17x  (27.43 -> 12.62 ns/pt)
    ("gga_x_sfat_pbe", "exc", "unpol"),  # 1.75x  (16.32 -> 9.31 ns/pt)
    ("gga_x_sfat_pbe", "vxc", "unpol"),  # 1.99x  (21.90 -> 10.98 ns/pt)
    ("gga_xc_th3", "exc", "unpol"),  # 1.83x  (11.03 -> 6.04 ns/pt)
    ("gga_xc_th3", "vxc", "unpol"),  # 1.84x  (13.72 -> 7.46 ns/pt)
    ("hyb_gga_xc_wb97", "exc", "unpol"),  # 1.75x  (21.66 -> 12.36 ns/pt)
    ("hyb_gga_xc_wb97", "vxc", "unpol"),  # 2.43x  (36.66 -> 15.07 ns/pt)
    ("hyb_lda_xc_bn05", "exc", "unpol"),  # 2.14x  (13.06 -> 6.11 ns/pt)
    ("hyb_lda_xc_bn05", "vxc", "unpol"),  # 2.24x  (15.54 -> 6.93 ns/pt)
    ("hyb_mgga_x_js18", "exc", "unpol"),  # 2.13x  (46.87 -> 22.00 ns/pt)
    ("hyb_mgga_x_js18", "vxc", "unpol"),  # 2.22x  (62.07 -> 27.98 ns/pt)
    ("hyb_mgga_x_pjs18", "exc", "unpol"),  # 1.99x  (28.93 -> 14.56 ns/pt)
    ("hyb_mgga_x_pjs18", "vxc", "unpol"),  # 2.07x  (44.39 -> 21.45 ns/pt)
    ("hyb_mgga_xc_gas22", "exc", "unpol"),  # 1.61x  (17.74 -> 11.04 ns/pt)
    ("hyb_mgga_xc_gas22", "vxc", "unpol"),  # 2.21x  (30.40 -> 13.76 ns/pt)
    ("lda_c_ml1", "exc", "unpol"),  # 1.38x  (12.69 -> 9.20 ns/pt)
    ("lda_c_ml1", "vxc", "unpol"),  # 1.41x  (13.22 -> 9.36 ns/pt)
    ("lda_c_pmgb06", "exc", "unpol"),  # 2.16x  (11.31 -> 5.24 ns/pt)
    ("lda_c_pmgb06", "vxc", "unpol"),  # 2.31x  (14.88 -> 6.44 ns/pt)
    ("lda_c_pw", "exc", "unpol"),  # 2.07x  (12.61 -> 6.10 ns/pt)
    ("lda_c_pw", "vxc", "unpol"),  # 2.01x  (14.21 -> 7.08 ns/pt)
    ("lda_c_pw_erf", "exc", "unpol"),  # 2.09x  (10.88 -> 5.22 ns/pt)
    ("lda_c_pw_erf", "vxc", "unpol"),  # 2.38x  (15.17 -> 6.38 ns/pt)
    ("lda_c_pz", "exc", "unpol"),  # 1.42x  (6.10 -> 4.30 ns/pt)
    ("lda_c_pz", "vxc", "unpol"),  # 1.64x  (7.98 -> 4.86 ns/pt)
    ("lda_xc_ksdt", "exc", "unpol"),  # 2.68x  (21.59 -> 8.05 ns/pt)
    ("lda_xc_ksdt", "vxc", "unpol"),  # 2.65x  (31.71 -> 11.97 ns/pt)
    ("mgga_c_b88", "exc", "unpol"),  # 1.89x  (11.29 -> 5.97 ns/pt)
    ("mgga_c_b88", "vxc", "unpol"),  # 2.12x  (14.07 -> 6.62 ns/pt)
    ("mgga_c_pkzb", "exc", "unpol"),  # 2.36x  (17.50 -> 7.42 ns/pt)
    ("mgga_c_pkzb", "vxc", "unpol"),  # 2.63x  (24.21 -> 9.21 ns/pt)
    ("mgga_c_revtpss", "exc", "unpol"),  # 2.16x  (36.28 -> 16.77 ns/pt)
    ("mgga_c_revtpss", "vxc", "unpol"),  # 2.21x  (51.35 -> 23.19 ns/pt)
    ("mgga_c_rmggac", "exc", "unpol"),  # 2.07x  (14.43 -> 6.98 ns/pt)
    ("mgga_c_rmggac", "vxc", "unpol"),  # 2.50x  (19.67 -> 7.86 ns/pt)
    ("mgga_c_rppscan", "exc", "unpol"),  # 2.06x  (14.59 -> 7.07 ns/pt)
    ("mgga_c_rppscan", "vxc", "unpol"),  # 2.34x  (19.41 -> 8.31 ns/pt)
    ("mgga_c_tpss", "exc", "unpol"),  # 2.21x  (37.53 -> 16.99 ns/pt)
    ("mgga_c_tpss", "vxc", "unpol"),  # 2.29x  (50.13 -> 21.88 ns/pt)
    ("mgga_x_br89_explicit", "exc", "unpol"),  # 1.66x  (11.40 -> 6.86 ns/pt)
    ("mgga_x_br89_explicit", "vxc", "unpol"),  # 1.91x  (16.37 -> 8.55 ns/pt)
    ("mgga_x_m11_l", "exc", "unpol"),  # 1.46x  (14.84 -> 10.19 ns/pt)
    ("mgga_x_m11_l", "vxc", "unpol"),  # 2.17x  (32.62 -> 15.04 ns/pt)
    ("mgga_x_r4scan", "vxc", "unpol"),  # 2.33x  (26.47 -> 11.37 ns/pt)
    ("mgga_x_regtm", "exc", "unpol"),  # 1.79x  (14.73 -> 8.23 ns/pt)
    ("mgga_x_regtm", "vxc", "unpol"),  # 2.20x  (22.44 -> 10.20 ns/pt)
    ("mgga_x_revtm", "exc", "unpol"),  # 1.91x  (11.47 -> 6.02 ns/pt)
    ("mgga_x_revtm", "vxc", "unpol"),  # 2.40x  (16.98 -> 7.07 ns/pt)
    ("mgga_x_rppscan", "vxc", "unpol"),  # 2.16x  (15.45 -> 7.16 ns/pt)
    ("mgga_x_task", "exc", "unpol"),  # 1.89x  (11.63 -> 6.17 ns/pt)
    ("mgga_x_task", "vxc", "unpol"),  # 2.37x  (16.57 -> 6.99 ns/pt)
    ("mgga_x_tm", "exc", "unpol"),  # 1.87x  (11.04 -> 5.89 ns/pt)
    ("mgga_x_tm", "vxc", "unpol"),  # 2.45x  (16.01 -> 6.54 ns/pt)
    # Added by tools/translate_rayon/simd_qualify.py; each line's
    # ratio is sweep ns/pt before -> after, fingerprint unchanged.
    ("gga_c_acgga", "exc", "unpol"),  # 2.06x  (11.94 -> 5.79 ns/pt)
    ("gga_c_acgga", "vxc", "unpol"),  # 2.13x  (15.02 -> 7.06 ns/pt)
    ("gga_c_acggap", "exc", "unpol"),  # 1.77x  (11.35 -> 6.42 ns/pt)
    ("gga_c_acggap", "vxc", "unpol"),  # 2.07x  (16.68 -> 8.07 ns/pt)
    ("gga_c_am05", "exc", "unpol"),  # 1.48x  (7.72 -> 5.22 ns/pt)
    ("gga_c_am05", "vxc", "unpol"),  # 1.63x  (9.28 -> 5.69 ns/pt)
    ("gga_c_bmk", "exc", "unpol"),  # 1.98x  (11.96 -> 6.04 ns/pt)
    ("gga_c_bmk", "vxc", "unpol"),  # 2.08x  (16.64 -> 8.00 ns/pt)
    ("gga_c_chachiyo", "exc", "unpol"),  # 1.65x  (8.58 -> 5.21 ns/pt)
    ("gga_c_chachiyo", "vxc", "unpol"),  # 1.75x  (9.77 -> 5.58 ns/pt)
    ("gga_c_gapc", "exc", "unpol"),  # 2.31x  (15.15 -> 6.57 ns/pt)
    ("gga_c_gapc", "vxc", "unpol"),  # 2.28x  (28.53 -> 12.53 ns/pt)
    ("gga_c_lypr", "exc", "unpol"),  # 1.48x  (13.39 -> 9.03 ns/pt)
    ("gga_c_op_b88", "exc", "unpol"),  # 1.82x  (9.61 -> 5.28 ns/pt)
    ("gga_c_op_b88", "vxc", "unpol"),  # 2.06x  (12.76 -> 6.20 ns/pt)
    ("gga_c_op_g96", "exc", "unpol"),  # 1.64x  (8.32 -> 5.06 ns/pt)
    ("gga_c_op_g96", "vxc", "unpol"),  # 1.67x  (9.87 -> 5.93 ns/pt)
    ("gga_c_op_pbe", "exc", "unpol"),  # 1.60x  (8.26 -> 5.16 ns/pt)
    ("gga_c_op_pbe", "vxc", "unpol"),  # 1.76x  (10.63 -> 6.05 ns/pt)
    ("gga_c_op_pw91", "exc", "unpol"),  # 1.51x  (10.87 -> 7.18 ns/pt)
    ("gga_c_op_pw91", "vxc", "unpol"),  # 1.86x  (14.85 -> 7.97 ns/pt)
    ("gga_c_p86", "exc", "unpol"),  # 1.61x  (8.69 -> 5.39 ns/pt)
    ("gga_c_p86", "vxc", "unpol"),  # 1.75x  (11.20 -> 6.39 ns/pt)
    ("gga_c_pbe_vwn", "vxc", "unpol"),  # 1.85x  (16.90 -> 9.15 ns/pt)
    ("gga_c_pbeloc", "exc", "unpol"),  # 1.78x  (11.16 -> 6.27 ns/pt)
    ("gga_c_pbeloc", "vxc", "unpol"),  # 2.00x  (15.12 -> 7.54 ns/pt)
    ("gga_c_q2d", "exc", "unpol"),  # 1.97x  (13.86 -> 7.05 ns/pt)
    ("gga_c_regtpss", "exc", "unpol"),  # 1.88x  (10.73 -> 5.71 ns/pt)
    ("gga_c_regtpss", "vxc", "unpol"),  # 2.10x  (14.36 -> 6.83 ns/pt)
    ("gga_c_scan_e0", "exc", "unpol"),  # 2.02x  (11.60 -> 5.75 ns/pt)
    ("gga_c_scan_e0", "vxc", "unpol"),  # 2.10x  (14.68 -> 6.98 ns/pt)
    ("gga_c_sogga11", "exc", "unpol"),  # 1.88x  (9.71 -> 5.16 ns/pt)
    ("gga_c_sogga11", "vxc", "unpol"),  # 2.23x  (13.05 -> 5.85 ns/pt)
    ("gga_k_apbe", "exc", "unpol"),  # 1.23x  (6.18 -> 5.01 ns/pt)
    ("gga_k_apbe", "vxc", "unpol"),  # 1.42x  (6.57 -> 4.64 ns/pt)
    ("gga_k_apbeint", "exc", "unpol"),  # 1.42x  (7.27 -> 5.11 ns/pt)
    ("gga_k_apbeint", "vxc", "unpol"),  # 1.57x  (8.56 -> 5.44 ns/pt)
    ("gga_k_exp4", "exc", "unpol"),  # 1.43x  (8.83 -> 6.17 ns/pt)
    ("gga_k_exp4", "vxc", "unpol"),  # 1.64x  (9.42 -> 5.76 ns/pt)
    ("gga_k_lgap", "exc", "unpol"),  # 1.71x  (7.78 -> 4.55 ns/pt)
    ("gga_k_lgap", "vxc", "unpol"),  # 1.56x  (8.07 -> 5.16 ns/pt)
    ("gga_k_lgap_ge", "exc", "unpol"),  # 1.37x  (6.58 -> 4.80 ns/pt)
    ("gga_k_lgap_ge", "vxc", "unpol"),  # 1.43x  (7.04 -> 4.94 ns/pt)
    ("gga_k_lkt", "exc", "unpol"),  # 1.63x  (8.40 -> 5.14 ns/pt)
    ("gga_k_llp", "exc", "unpol"),  # 1.35x  (7.53 -> 5.56 ns/pt)
    ("gga_k_llp", "vxc", "unpol"),  # 1.60x  (8.74 -> 5.46 ns/pt)
    ("gga_k_meyer", "exc", "unpol"),  # 1.66x  (8.74 -> 5.26 ns/pt)
    ("gga_k_meyer", "vxc", "unpol"),  # 1.70x  (9.39 -> 5.54 ns/pt)
    ("gga_k_ol1", "exc", "unpol"),  # 1.41x  (6.32 -> 4.47 ns/pt)
    ("gga_k_ol1", "vxc", "unpol"),  # 1.51x  (6.87 -> 4.53 ns/pt)
    ("gga_k_pearson", "exc", "unpol"),  # 1.33x  (5.94 -> 4.46 ns/pt)
    ("gga_k_pearson", "vxc", "unpol"),  # 1.45x  (6.70 -> 4.61 ns/pt)
    ("gga_k_rational_p", "exc", "unpol"),  # 1.60x  (8.42 -> 5.27 ns/pt)
    ("gga_k_rational_p", "vxc", "unpol"),  # 1.59x  (8.84 -> 5.57 ns/pt)
    ("gga_k_vt84f", "exc", "unpol"),  # 1.42x  (8.27 -> 5.84 ns/pt)
    ("gga_k_vt84f", "vxc", "unpol"),  # 1.83x  (10.96 -> 5.99 ns/pt)
    ("gga_x_ak13", "exc", "unpol"),  # 1.52x  (8.15 -> 5.37 ns/pt)
    ("gga_x_ak13", "vxc", "unpol"),  # 1.69x  (9.75 -> 5.76 ns/pt)
    ("gga_x_b86", "exc", "unpol"),  # 1.77x  (8.32 -> 4.69 ns/pt)
    ("gga_x_b86", "vxc", "unpol"),  # 1.81x  (9.21 -> 5.10 ns/pt)
    ("gga_x_bayesian", "exc", "unpol"),  # 1.43x  (6.48 -> 4.53 ns/pt)
    ("gga_x_bayesian", "vxc", "unpol"),  # 1.59x  (8.44 -> 5.30 ns/pt)
    ("gga_x_beefvdw", "exc", "unpol"),  # 1.74x  (9.53 -> 5.47 ns/pt)
    ("gga_x_beefvdw", "vxc", "unpol"),  # 2.24x  (13.26 -> 5.93 ns/pt)
    ("gga_x_bpccac", "exc", "unpol"),  # 1.62x  (9.64 -> 5.94 ns/pt)
    ("gga_x_bpccac", "vxc", "unpol"),  # 1.70x  (11.29 -> 6.66 ns/pt)
    ("gga_x_c09x", "exc", "unpol"),  # 1.47x  (7.35 -> 5.00 ns/pt)
    ("gga_x_c09x", "vxc", "unpol"),  # 1.58x  (8.46 -> 5.34 ns/pt)
    ("gga_x_cap", "exc", "unpol"),  # 1.56x  (7.83 -> 5.03 ns/pt)
    ("gga_x_cap", "vxc", "unpol"),  # 1.72x  (9.98 -> 5.81 ns/pt)
    ("gga_x_ev93", "exc", "unpol"),  # 1.44x  (6.76 -> 4.71 ns/pt)
    ("gga_x_ev93", "vxc", "unpol"),  # 1.61x  (8.49 -> 5.26 ns/pt)
    ("gga_x_g96", "exc", "unpol"),  # 1.41x  (6.70 -> 4.76 ns/pt)
    ("gga_x_g96", "vxc", "unpol"),  # 1.29x  (6.65 -> 5.16 ns/pt)
    ("gga_x_hcth_a", "exc", "unpol"),  # 1.49x  (7.68 -> 5.14 ns/pt)
    ("gga_x_hcth_a", "vxc", "unpol"),  # 1.59x  (9.57 -> 6.03 ns/pt)
    ("gga_x_hjs_b88_v2", "exc", "unpol"),  # 2.35x  (15.92 -> 6.77 ns/pt)
    ("gga_x_hjs_b88_v2", "vxc", "unpol"),  # 2.69x  (25.68 -> 9.55 ns/pt)
    ("gga_x_htbs", "exc", "unpol"),  # 1.72x  (9.46 -> 5.50 ns/pt)
    ("gga_x_htbs", "vxc", "unpol"),  # 1.93x  (12.18 -> 6.30 ns/pt)
    ("gga_x_lg93", "exc", "unpol"),  # 1.69x  (8.77 -> 5.20 ns/pt)
    ("gga_x_lspbe", "exc", "unpol"),  # 1.49x  (6.78 -> 4.55 ns/pt)
    ("gga_x_lspbe", "vxc", "unpol"),  # 1.67x  (8.20 -> 4.92 ns/pt)
    ("gga_x_lsrpbe", "exc", "unpol"),  # 1.52x  (7.49 -> 4.93 ns/pt)
    ("gga_x_lsrpbe", "vxc", "unpol"),  # 1.49x  (7.89 -> 5.31 ns/pt)
    ("gga_x_lv_rpw86", "exc", "unpol"),  # 1.64x  (8.89 -> 5.43 ns/pt)
    ("gga_x_lv_rpw86", "vxc", "unpol"),  # 1.73x  (11.20 -> 6.48 ns/pt)
    ("gga_x_mpbe", "exc", "unpol"),  # 1.46x  (7.05 -> 4.84 ns/pt)
    ("gga_x_mpbe", "vxc", "unpol"),  # 1.75x  (9.34 -> 5.33 ns/pt)
    ("gga_x_pbe_erf_gws", "exc", "unpol"),  # 1.92x  (13.75 -> 7.16 ns/pt)
    ("gga_x_pbea", "exc", "unpol"),  # 1.69x  (8.15 -> 4.83 ns/pt)
    ("gga_x_pbea", "vxc", "unpol"),  # 1.53x  (9.25 -> 6.06 ns/pt)
    ("gga_x_pbepow", "exc", "unpol"),  # 1.61x  (8.86 -> 5.51 ns/pt)
    ("gga_x_pbetrans", "exc", "unpol"),  # 1.54x  (7.42 -> 4.80 ns/pt)
    ("gga_x_pbetrans", "vxc", "unpol"),  # 1.76x  (8.96 -> 5.10 ns/pt)
    ("gga_x_pw86", "exc", "unpol"),  # 1.61x  (8.64 -> 5.38 ns/pt)
    ("gga_x_pw86", "vxc", "unpol"),  # 1.74x  (9.90 -> 5.69 ns/pt)
    ("gga_x_q2d", "exc", "unpol"),  # 1.54x  (8.64 -> 5.62 ns/pt)
    ("gga_x_rpbe", "exc", "unpol"),  # 1.53x  (6.75 -> 4.42 ns/pt)
    ("gga_x_rpbe", "vxc", "unpol"),  # 1.38x  (7.16 -> 5.17 ns/pt)
    ("gga_x_sogga11", "exc", "unpol"),  # 1.51x  (8.00 -> 5.32 ns/pt)
    ("gga_x_sogga11", "vxc", "unpol"),  # 1.92x  (9.51 -> 4.96 ns/pt)
    ("gga_x_wc", "exc", "unpol"),  # 1.60x  (8.13 -> 5.08 ns/pt)
    ("gga_x_wc", "vxc", "unpol"),  # 1.78x  (9.72 -> 5.46 ns/pt)
    ("gga_xc_th1", "exc", "unpol"),  # 1.76x  (8.29 -> 4.71 ns/pt)
    ("gga_xc_th1", "vxc", "unpol"),  # 2.08x  (10.56 -> 5.07 ns/pt)
    ("lda_c_rc04", "exc", "unpol"),  # 1.24x  (4.77 -> 3.85 ns/pt)
    ("lda_c_rc04", "vxc", "unpol"),  # 1.27x  (4.72 -> 3.72 ns/pt)
    ("lda_x_erf", "exc", "unpol"),  # 1.51x  (7.38 -> 4.89 ns/pt)
    ("lda_x_erf", "vxc", "unpol"),  # 1.71x  (9.64 -> 5.63 ns/pt)
    ("lda_x_rel", "exc", "unpol"),  # 1.33x  (4.98 -> 3.75 ns/pt)
    ("lda_x_rel", "vxc", "unpol"),  # 1.68x  (5.96 -> 3.54 ns/pt)
    ("lda_x_yukawa", "exc", "unpol"),  # 2.06x  (12.26 -> 5.96 ns/pt)
    ("lda_x_yukawa", "vxc", "unpol"),  # 1.96x  (12.79 -> 6.51 ns/pt)
    ("mgga_c_bc95", "exc", "unpol"),  # 1.73x  (11.43 -> 6.61 ns/pt)
    ("mgga_c_bc95", "vxc", "unpol"),  # 1.95x  (15.03 -> 7.73 ns/pt)
    ("mgga_c_ccalda", "exc", "unpol"),  # 1.55x  (8.08 -> 5.21 ns/pt)
    ("mgga_c_ccalda", "vxc", "unpol"),  # 1.82x  (11.32 -> 6.23 ns/pt)
    ("mgga_c_m05", "exc", "unpol"),  # 1.86x  (13.01 -> 6.99 ns/pt)
    ("mgga_c_m05", "vxc", "unpol"),  # 2.22x  (20.18 -> 9.09 ns/pt)
    ("mgga_c_vsxc", "exc", "unpol"),  # 1.60x  (11.51 -> 7.17 ns/pt)
    ("mgga_c_vsxc", "vxc", "unpol"),  # 1.98x  (17.75 -> 8.95 ns/pt)
    ("mgga_x_edmgga", "exc", "unpol"),  # 1.78x  (9.74 -> 5.47 ns/pt)
    ("mgga_x_edmgga", "vxc", "unpol"),  # 1.95x  (12.44 -> 6.38 ns/pt)
    ("mgga_x_gvt4", "exc", "unpol"),  # 1.23x  (7.25 -> 5.92 ns/pt)
    ("mgga_x_gvt4", "vxc", "unpol"),  # 1.47x  (8.64 -> 5.88 ns/pt)
    ("mgga_x_jk", "exc", "unpol"),  # 1.58x  (8.25 -> 5.21 ns/pt)
    ("mgga_x_jk", "vxc", "unpol"),  # 1.58x  (9.65 -> 6.11 ns/pt)
    ("mgga_x_lta", "exc", "unpol"),  # 1.61x  (8.56 -> 5.32 ns/pt)
    ("mgga_x_lta", "vxc", "unpol"),  # 1.58x  (8.96 -> 5.68 ns/pt)
    ("mgga_x_mvs", "exc", "unpol"),  # 1.56x  (9.41 -> 6.04 ns/pt)
    ("mgga_x_mvs", "vxc", "unpol"),  # 1.74x  (11.01 -> 6.34 ns/pt)
    ("mgga_x_mvsb", "exc", "unpol"),  # 1.63x  (9.41 -> 5.77 ns/pt)
    ("mgga_x_mvsb", "vxc", "unpol"),  # 1.93x  (12.19 -> 6.33 ns/pt)
    ("mgga_x_r4scan", "exc", "unpol"),  # 1.73x  (12.73 -> 7.35 ns/pt)
    ("mgga_x_regtpss", "exc", "unpol"),  # 1.92x  (10.26 -> 5.35 ns/pt)
    ("mgga_x_regtpss", "vxc", "unpol"),  # 2.27x  (16.81 -> 7.42 ns/pt)
    ("mgga_x_rppscan", "exc", "unpol"),  # 1.73x  (10.15 -> 5.87 ns/pt)
    ("mgga_x_rtpss", "exc", "unpol"),  # 1.62x  (8.89 -> 5.48 ns/pt)
    ("mgga_x_rtpss", "vxc", "unpol"),  # 2.35x  (15.22 -> 6.48 ns/pt)
    ("mgga_x_sa_tpss", "exc", "unpol"),  # 1.77x  (10.16 -> 5.74 ns/pt)
    ("mgga_x_sa_tpss", "vxc", "unpol"),  # 2.40x  (17.42 -> 7.26 ns/pt)
    ("mgga_x_vt84", "exc", "unpol"),  # 1.59x  (10.12 -> 6.36 ns/pt)
    ("mgga_x_vt84", "vxc", "unpol"),  # 2.22x  (16.40 -> 7.39 ns/pt)
    # Added by tools/translate_rayon/simd_qualify.py; each line's
    # ratio is sweep ns/pt before -> after, fingerprint unchanged.
    ("gga_c_ccdf", "exc", "unpol"),  # 1.40x  (5.19 -> 3.71 ns/pt)
    ("gga_c_ccdf", "vxc", "unpol"),  # 1.51x  (5.92 -> 3.93 ns/pt)
    ("gga_c_op_xalpha", "vxc", "unpol"),  # 1.81x  (9.14 -> 5.05 ns/pt)
    ("gga_c_wi", "exc", "unpol"),  # 1.18x  (5.25 -> 4.44 ns/pt)
    ("gga_c_wi", "vxc", "unpol"),  # 1.57x  (6.12 -> 3.91 ns/pt)
    ("gga_c_wl", "exc", "unpol"),  # 1.30x  (3.95 -> 3.04 ns/pt)
    ("gga_c_wl", "vxc", "unpol"),  # 1.42x  (4.53 -> 3.19 ns/pt)
    ("gga_k_dk", "exc", "unpol"),  # 1.48x  (8.38 -> 5.67 ns/pt)
    ("gga_k_dk", "vxc", "unpol"),  # 1.83x  (10.64 -> 5.83 ns/pt)
    ("gga_k_ol2", "exc", "unpol"),  # 1.40x  (7.69 -> 5.48 ns/pt)
    ("gga_k_ol2", "vxc", "unpol"),  # 1.74x  (9.03 -> 5.20 ns/pt)
    ("gga_k_thakkar", "exc", "unpol"),  # 1.61x  (10.74 -> 6.65 ns/pt)
    ("gga_x_chachiyo", "exc", "unpol"),  # 1.61x  (9.70 -> 6.01 ns/pt)
    ("gga_x_chachiyo", "vxc", "unpol"),  # 1.72x  (11.51 -> 6.69 ns/pt)
    ("gga_x_n12", "exc", "unpol"),  # 1.97x  (10.32 -> 5.23 ns/pt)
    ("gga_x_n12", "vxc", "unpol"),  # 1.75x  (12.79 -> 7.30 ns/pt)
    ("gga_x_ol2", "exc", "unpol"),  # 1.59x  (8.43 -> 5.31 ns/pt)
    ("gga_x_ol2", "vxc", "unpol"),  # 1.59x  (9.27 -> 5.83 ns/pt)
    ("gga_x_optx", "exc", "unpol"),  # 1.52x  (8.03 -> 5.29 ns/pt)
    ("gga_x_optx", "vxc", "unpol"),  # 1.70x  (9.49 -> 5.57 ns/pt)
    ("gga_x_pbeint", "exc", "unpol"),  # 1.25x  (7.65 -> 6.11 ns/pt)
    ("gga_x_pbeint", "vxc", "unpol"),  # 1.66x  (9.75 -> 5.89 ns/pt)
    ("gga_x_q1d", "exc", "unpol"),  # 1.30x  (7.70 -> 5.91 ns/pt)
    ("gga_x_q1d", "vxc", "unpol"),  # 1.65x  (10.07 -> 6.09 ns/pt)
    ("gga_x_rge2", "exc", "unpol"),  # 1.26x  (6.71 -> 5.31 ns/pt)
    ("gga_x_rge2", "vxc", "unpol"),  # 1.35x  (7.46 -> 5.52 ns/pt)
    ("gga_x_sg4", "exc", "unpol"),  # 1.39x  (6.89 -> 4.97 ns/pt)
    ("gga_x_sg4", "vxc", "unpol"),  # 1.70x  (8.49 -> 5.01 ns/pt)
    ("gga_x_ssb_sw", "exc", "unpol"),  # 1.51x  (7.49 -> 4.95 ns/pt)
    ("gga_x_ssb_sw", "vxc", "unpol"),  # 1.77x  (9.98 -> 5.63 ns/pt)
    ("hyb_mgga_x_dldf", "exc", "unpol"),  # 1.78x  (8.93 -> 5.01 ns/pt)
    ("hyb_mgga_x_dldf", "vxc", "unpol"),  # 1.88x  (9.91 -> 5.28 ns/pt)
    ("hyb_mgga_x_m05", "exc", "unpol"),  # 1.62x  (10.94 -> 6.75 ns/pt)
    ("hyb_mgga_x_m05", "vxc", "unpol"),  # 1.90x  (13.41 -> 7.04 ns/pt)
    ("lda_c_2d_amgb", "exc", "unpol"),  # 2.34x  (3.67 -> 1.56 ns/pt)
    ("lda_c_2d_amgb", "vxc", "unpol"),  # 2.40x  (4.16 -> 1.73 ns/pt)
    ("lda_c_chachiyo", "exc", "unpol"),  # 1.73x  (6.57 -> 3.79 ns/pt)
    ("lda_c_chachiyo", "vxc", "unpol"),  # 1.62x  (7.20 -> 4.44 ns/pt)
    ("lda_c_chachiyo_mod", "exc", "unpol"),  # 1.82x  (6.80 -> 3.74 ns/pt)
    ("lda_c_chachiyo_mod", "vxc", "unpol"),  # 1.82x  (7.07 -> 3.88 ns/pt)
    ("lda_c_gk72", "exc", "unpol"),  # 1.62x  (4.03 -> 2.49 ns/pt)
    ("lda_c_gk72", "vxc", "unpol"),  # 1.79x  (7.27 -> 4.07 ns/pt)
    ("lda_c_gombas", "exc", "unpol"),  # 1.98x  (3.45 -> 1.74 ns/pt)
    ("lda_c_gombas", "vxc", "unpol"),  # 2.09x  (4.09 -> 1.96 ns/pt)
    ("lda_c_hl", "exc", "unpol"),  # 1.74x  (7.26 -> 4.17 ns/pt)
    ("lda_c_hl", "vxc", "unpol"),  # 1.90x  (7.86 -> 4.15 ns/pt)
    ("lda_c_rpa", "exc", "unpol"),  # 1.51x  (3.55 -> 2.35 ns/pt)
    ("lda_c_rpa", "vxc", "unpol"),  # 1.67x  (3.80 -> 2.28 ns/pt)
    ("lda_c_wigner", "exc", "unpol"),  # 1.06x  (2.17 -> 2.04 ns/pt)
    ("lda_c_wigner", "vxc", "unpol"),  # 1.24x  (2.52 -> 2.04 ns/pt)
    ("lda_k_tf", "exc", "unpol"),  # 1.07x  (3.64 -> 3.38 ns/pt)
    ("lda_k_tf", "vxc", "unpol"),  # 1.27x  (4.11 -> 3.23 ns/pt)
    ("lda_k_zlp", "exc", "unpol"),  # 1.56x  (5.70 -> 3.65 ns/pt)
    ("lda_k_zlp", "vxc", "unpol"),  # 1.59x  (7.02 -> 4.41 ns/pt)
    ("lda_x", "exc", "unpol"),  # 1.63x  (3.52 -> 2.16 ns/pt)
    ("lda_x", "vxc", "unpol"),  # 1.62x  (3.55 -> 2.19 ns/pt)
    ("lda_x_sloc", "exc", "unpol"),  # 1.51x  (3.02 -> 2.00 ns/pt)
    ("lda_x_sloc", "vxc", "unpol"),  # 1.41x  (3.18 -> 2.25 ns/pt)
    ("lda_xc_teter93", "exc", "unpol"),  # 1.40x  (5.02 -> 3.58 ns/pt)
    ("lda_xc_teter93", "vxc", "unpol"),  # 1.67x  (6.80 -> 4.08 ns/pt)
    ("lda_xc_zlp", "exc", "unpol"),  # 1.88x  (3.26 -> 1.73 ns/pt)
    ("lda_xc_zlp", "vxc", "unpol"),  # 1.99x  (3.58 -> 1.80 ns/pt)
    ("mgga_c_cc", "exc", "unpol"),  # 1.51x  (8.70 -> 5.75 ns/pt)
    ("mgga_c_cs", "exc", "unpol"),  # 1.55x  (5.96 -> 3.83 ns/pt)
    ("mgga_c_cs", "vxc", "unpol"),  # 1.68x  (6.51 -> 3.89 ns/pt)
    ("mgga_k_gea2", "exc", "unpol"),  # 1.37x  (7.51 -> 5.48 ns/pt)
    ("mgga_k_gea2", "vxc", "unpol"),  # 1.51x  (8.67 -> 5.74 ns/pt)
    ("mgga_k_gea4", "exc", "unpol"),  # 1.45x  (8.49 -> 5.86 ns/pt)
    ("mgga_k_gea4", "vxc", "unpol"),  # 1.80x  (10.61 -> 5.90 ns/pt)
    ("mgga_k_rda", "exc", "unpol"),  # 1.57x  (10.32 -> 6.56 ns/pt)
    ("mgga_k_rda", "vxc", "unpol"),  # 1.59x  (13.29 -> 8.38 ns/pt)
    ("mgga_x_2d_js17", "exc", "unpol"),  # 1.71x  (6.41 -> 3.75 ns/pt)
    ("mgga_x_2d_js17", "vxc", "unpol"),  # 1.59x  (7.11 -> 4.48 ns/pt)
    ("mgga_x_ft98", "exc", "unpol"),  # 1.78x  (11.59 -> 6.52 ns/pt)
    ("mgga_x_ft98", "vxc", "unpol"),  # 2.21x  (17.67 -> 7.99 ns/pt)
    ("mgga_x_gx", "exc", "unpol"),  # 1.49x  (8.50 -> 5.69 ns/pt)
    ("mgga_x_gx", "vxc", "unpol"),  # 1.37x  (9.66 -> 7.06 ns/pt)
    ("mgga_x_mbeef", "exc", "unpol"),  # 2.04x  (13.93 -> 6.84 ns/pt)
    ("mgga_x_mbeef", "vxc", "unpol"),  # 2.58x  (28.47 -> 11.03 ns/pt)
    ("mgga_x_mbeefvdw", "exc", "unpol"),  # 1.55x  (10.25 -> 6.60 ns/pt)
    ("mgga_x_mbeefvdw", "vxc", "unpol"),  # 2.02x  (18.17 -> 8.98 ns/pt)
    ("mgga_x_mcml", "exc", "unpol"),  # 1.77x  (12.34 -> 6.96 ns/pt)
    ("mgga_x_mcml", "vxc", "unpol"),  # 2.44x  (31.77 -> 12.99 ns/pt)
    ("mgga_x_pbe_gx", "exc", "unpol"),  # 1.37x  (7.79 -> 5.69 ns/pt)
    ("mgga_x_pbe_gx", "vxc", "unpol"),  # 1.63x  (10.66 -> 6.56 ns/pt)
    ("mgga_x_pkzb", "exc", "unpol"),  # 1.32x  (7.34 -> 5.57 ns/pt)
    ("mgga_x_pkzb", "vxc", "unpol"),  # 1.34x  (9.42 -> 7.05 ns/pt)
    ("mgga_x_rlda", "exc", "unpol"),  # 1.19x  (7.16 -> 6.04 ns/pt)
    ("mgga_x_rlda", "vxc", "unpol"),  # 1.24x  (7.45 -> 6.03 ns/pt)
    ("mgga_x_tau_hcth", "exc", "unpol"),  # 1.60x  (9.03 -> 5.65 ns/pt)
    ("mgga_x_tau_hcth", "vxc", "unpol"),  # 1.70x  (11.74 -> 6.90 ns/pt)
    ("mgga_x_th", "exc", "unpol"),  # 1.24x  (6.46 -> 5.20 ns/pt)
    ("mgga_x_th", "vxc", "unpol"),  # 1.05x  (5.26 -> 5.03 ns/pt)
    ("mgga_x_vcml", "exc", "unpol"),  # 1.91x  (14.62 -> 7.67 ns/pt)
    ("mgga_x_vcml", "vxc", "unpol"),  # 2.62x  (32.23 -> 12.29 ns/pt)
    ("mgga_xc_zlp", "exc", "unpol"),  # 1.56x  (5.43 -> 3.47 ns/pt)
    ("mgga_xc_zlp", "vxc", "unpol"),  # 1.54x  (5.46 -> 3.55 ns/pt)
}

# Sweep override, read by tools/translate_rayon/simd_qualify.py. It lets the
# qualification driver try a batch of candidate triples without rewriting the
# literal above, so a sweep that is interrupted leaves no half-edited allowlist
# behind; only triples that actually passed the gate are written into the set,
# and that is a separate deliberate step (`simd_qualify.py --apply`). Format is
# `func:order:spin,func:order:spin,...`. Unset -- the normal case, including
# every `--all` regeneration -- reproduces the committed allowlist exactly.
_simd_extra = os.environ.get("LIBXC_RS_SIMD_EXTRA", "").strip()
if _simd_extra:
    for _t in _simd_extra.split(","):
        _t = _t.strip()
        if not _t:
            continue
        _p = _t.split(":")
        if len(_p) != 3:
            raise SystemExit(
                f"LIBXC_RS_SIMD_EXTRA: expected func:order:spin, got {_t!r}")
        if _p[1] not in ORDERS or _p[2] not in ("unpol", "pol"):
            raise SystemExit(f"LIBXC_RS_SIMD_EXTRA: bad order/spin in {_t!r}")
        SIMD_EXACT_FUNCS.add((_p[0], _p[1], _p[2]))

# There is no second, approximate mode. This project uses rmath's bit-exact
# path only: `libxc_rkernel_math::rmath` is a `<BitExact, FullRange>` surface
# (`math/src/rmath_bitexact.rs`) and the upstream crate -- whose own free
# functions are deliberately its Fast path -- is not reachable from a kernel.
SIMD_FUNCS = SIMD_EXACT_FUNCS
SPINS = ["unpol", "pol"]


class Untranslatable(Exception):
    pass


# --------------------------------------------------------------------------
# Dimensions: elements per grid point, per family and spin.
# Mirrors libxc `util.c: internal_counters_set_{lda,gga,mgga}` and
# `crates/libxc-core/src/dims`. Used both for input indexing and for the
# output writes.
# --------------------------------------------------------------------------

def dims(fam: str, pol: bool) -> dict[str, int]:
    """Elements per grid point for every array, read out of
    `crates/libxc-core/src/dims/mod.rs`.

    These are NOT hand-derived here. They were once, and it was wrong:
    `v3sigma2lapl` polarized is `6*2 = 12` (libxc `util.c`
    `internal_counters_set_mgga`), not the 9 a plain count of index
    combinations suggests. Getting one of these wrong misaligns every
    subsequent grid point of that output and is invisible in a spot check, so
    the table is parsed from the single definition the eval layer also indexes
    with -- the two cannot drift apart.
    """
    key = (fam, pol)
    if key in _DIMS_CACHE:
        return _DIMS_CACHE[key]
    src = (REPO / "crates/libxc-core/src/dims/mod.rs").read_text()
    want = "Polarized" if pol else "Unpolarized"
    out: dict[str, int] = {}

    # `lda` -> `gga` -> `mgga` each start from the previous, so replay the
    # chain up to the family asked for.
    for f in ("lda", "gga", "mgga"):
        m = re.search(rf"pub fn {f}\(spin: Spin\) -> Self \{{(.*?)\n    \}}", src, re.S)
        if not m:
            raise Untranslatable(f"cannot find Dimensions::{f} in libxc-core")
        body = m.group(1)
        # assignments outside the spin match, e.g. `d.lapl = spin as u8;`
        def take(text: str) -> None:
            # libxc-core writes the polarized values as products that mirror
            # util.c (`d.v2sigmalapl = 3 * 2;`), so the right-hand side has to
            # be evaluated, not just parsed as an integer.
            for name, val in re.findall(r"d\.(\w+) = ([^;]+);", text):
                val = val.split("//")[0].strip()
                if val == "spin as u8":
                    out[name] = 2 if pol else 1
                elif re.fullmatch(r"[\d\s*+]+", val):
                    out[name] = eval(val)  # noqa: S307 - digits and * + only
                else:
                    raise Untranslatable(
                        f"cannot evaluate Dimensions::{f} {name} = {val!r}")

        take(body.split("match spin")[0])
        mm = re.search(rf"Spin::{want} => \{{(.*?)^            \}}", body, re.S | re.M)
        if mm:
            take(mm.group(1))
        if f == fam:
            break
    out.setdefault("zk", 1)
    _DIMS_CACHE[key] = out
    return out


_DIMS_CACHE: dict[tuple[str, bool], dict[str, int]] = {}


def dim_of(name: str, fam: str, pol: bool) -> int:
    d = dims(fam, pol)
    if name not in d:
        raise Untranslatable(f"no dimension recorded for {name!r} ({fam}, pol={pol})")
    return d[name]


# --------------------------------------------------------------------------
# Vocabulary
# --------------------------------------------------------------------------

# C name -> Rust path (imported from libxc_rkernel_math)
FUNCS = {
    "my_piecewise3": ("piecewise3", "piecewise"),
    "my_piecewise5": ("piecewise5", "piecewise"),
    "Heaviside": ("Heaviside", "piecewise"),
    "POW_1_3": ("pow_1_3", "powers"),
    "POW_2_3": ("pow_2_3", "powers"),
    "POW_4_3": ("pow_4_3", "powers"),
    "POW_5_3": ("pow_5_3", "powers"),
    "POW_7_3": ("pow_7_3", "powers"),
    "POW_3_2": ("pow_3_2", "powers"),
    "POW_1_4": ("pow_1_4", "powers"),
    "POW_2": ("pow_2", "powers"),
    "POW_3": ("pow_3", "powers"),
    "xc_E1_scaled": ("xc_e1_scaled", "expint_e1"),
    "xc_erfcx": ("xc_erfcx", "erf"),
    "xc_dilogarithm": ("xc_dilogarithm", "special"),
    "xc_mgga_x_br89_get_x": ("xc_mgga_x_br89_get_x", "br89"),
    "xc_mgga_x_mbrxc_get_x": ("xc_mgga_x_mbrxc_get_x", "mbrxc"),
    "xc_bessel_I0": ("xc_bessel_I0", "bessel"),
    "xc_bessel_I1": ("xc_bessel_I1", "bessel"),
    "xc_bessel_K0": ("xc_bessel_K0", "bessel"),
    "xc_bessel_K1": ("xc_bessel_K1", "bessel"),
    "xc_bessel_I0_scaled": ("xc_bessel_I0_scaled", "bessel"),
    "xc_bessel_I1_scaled": ("xc_bessel_I1_scaled", "bessel"),
    "xc_bessel_K0_scaled": ("xc_bessel_K0_scaled", "bessel"),
    "xc_bessel_K1_scaled": ("xc_bessel_K1_scaled", "bessel"),
    "lambert_w": ("lambert_w", "lambert_w"),
    "LambertW": ("lambert_w", "lambert_w"),
}

# C libm -> rmath functions.
LIBM = {
    "sqrt": "rmath::sqrt", "log": "rmath::ln", "exp": "rmath::exp",
    "atan": "rmath::atan", "atan2": "rmath::atan2", "fabs": "rmath::abs",
    "tanh": "rmath::tanh", "sinh": "rmath::sinh", "cosh": "rmath::cosh",
    "asinh": "rmath::asinh", "acosh": "rmath::acosh", "atanh": "rmath::atanh",
    "sin": "rmath::sin", "cos": "rmath::cos", "tan": "rmath::tan",
    "asin": "rmath::asin", "acos": "rmath::acos", "pow": "POWF",
    "erf": "rmath::erf", "erfc": "rmath::erfc", "cbrt": "rmath::cbrt",
    "expm1": "rmath::expm1", "log1p": "rmath::log1p",
}
ERF_MOD = {}

# Which pre-specialised integrand each functional's `xc_integrate(funcN, ...)`
# resolves to, and the extra arguments the C integrand reads off `params`.
INTEGRATE_NAME = {
    ("gga_x_fd_lb94", "func0"): "xc_integrate_func0",
    ("gga_x_fd_lb94", "func1"): "xc_integrate_func1",
    ("lda_x_1d_soft", "func1"): "xc_integrate_lda_soft_func1",
    ("lda_x_1d_soft", "func2"): "xc_integrate_lda_soft_func2",
    ("lda_x_1d_exponential", "func1"): "xc_integrate_lda_exponential_func1",
    ("lda_x_1d_exponential", "func2"): "xc_integrate_lda_exponential_func2",
}
INTEGRATE_ARGS = {"gga_x_fd_lb94": ["param_beta"]}

# case21 b-splines take a fixed 10-coefficient control vector.
BSPLINE_COEFFS = 10

LANES_NOTE = ("Eight grid points per step; every lane runs maple2c's "
              "expression\n//! sequence in its original order.")

CONSTS = {"M_PI", "M_SQRTPI", "M_CBRTPI", "M_SQRT3", "M_CBRT2", "M_CBRT3",
          "M_CBRT4", "M_CBRT5", "M_CBRT6", "M_CBRT7", "M_CBRT9", "M_SQRT2",
          "M_C"}

INPUTS = {"lda": ["rho"], "gga": ["rho", "sigma"],
          "mgga": ["rho", "sigma", "lapl", "tau"]}

# Outputs written at each order, cumulative. Matches
# crates/libxc-reval `required_fields`.
OUT_ORDER = {
    "lda": [["zk"], ["vrho"], ["v2rho2"], ["v3rho3"], ["v4rho4"]],
    "gga": [["zk"], ["vrho", "vsigma"],
            ["v2rho2", "v2rhosigma", "v2sigma2"],
            ["v3rho3", "v3rho2sigma", "v3rhosigma2", "v3sigma3"],
            ["v4rho4", "v4rho3sigma", "v4rho2sigma2", "v4rhosigma3", "v4sigma4"]],
}

NUM = re.compile(r"(?<![\w.])(\d+\.\d*(?:[eE][+-]?\d+)?|\.\d+(?:[eE][+-]?\d+)?|\d+(?:[eE][+-]?\d+)?)")
# An identifier can never be preceded by a word character or a dot. Without
# that guard this matches the `e` in a converted literal like `8.64e-07` and
# reports it as an unknown symbol.
IDENT = re.compile(r"(?<![\w.])[A-Za-z_]\w*")


def c_number_to_rust(tok: str) -> str:
    """maple2c writes every constant in scientific form (`0.2e1`, `0.310907e-1`).
    Reparse and re-emit; Python's repr round-trips a double exactly, so the
    bits are identical however the text differs."""
    v = float(tok)
    r = repr(v)
    if "." not in r and "e" not in r and "E" not in r and "inf" not in r and "nan" not in r:
        r += ".0"
    return r


# --------------------------------------------------------------------------
# Parsing
# --------------------------------------------------------------------------

def strip_comments(s: str) -> str:
    s = re.sub(r"/\*.*?\*/", "", s, flags=re.S)
    return re.sub(r"//[^\n]*", "", s)


def split_functions(src: str) -> dict[tuple[str, str], str]:
    """Return {(order, spin): body} for every `func_<order>_<spin>` present."""
    src = strip_comments(src)
    out: dict[tuple[str, str], str] = {}
    for m in re.finditer(r"^func_(\w+?)_(unpol|pol)\s*\(", src, re.M):
        order, spin = m.group(1), m.group(2)
        if order not in ORDERS:
            continue
        # body runs from the opening brace to its matching close
        b = src.index("{", m.end())
        depth, i = 0, b
        while i < len(src):
            if src[i] == "{":
                depth += 1
            elif src[i] == "}":
                depth -= 1
                if depth == 0:
                    break
            i += 1
        out[(order, spin)] = src[b + 1:i]
    return out


def statements(body: str) -> list[str]:
    """Split a function body into `;`-terminated statements, dropping the
    declarations and the params boilerplate."""
    body = re.sub(r"#\s*\w+[^\n]*", "", body)
    # Output writes are guarded:
    #     if(out->zk != NULL && (p->info->flags & XC_FLAGS_HAVE_EXC))
    #       out->zk[ip*p->dim.zk + 0] += tzk0;
    # The guard is libxc asking whether the caller supplied that buffer and
    # whether the functional advertises the derivative. Here `prepare()`
    # guarantees every buffer the requested order needs, and dispatch already
    # picked the right order, so the guard is dropped. Note `[^)]*` will not do
    # for stripping it -- the condition itself contains parentheses.
    body = re.sub(r"if\s*\((?:[^()]|\([^()]*\))*\)\s*(?=out->)", "", body)
    raw = [s.strip() for s in body.split(";")]
    out = []
    for s in raw:
        s = " ".join(s.split())
        if not s:
            continue
        if s.startswith("double ") or s.startswith("const double "):
            continue
        if "assert(" in s or s.startswith("params =") or s.endswith("*params"):
            continue
        if re.match(r"^\w+\s*\*\s*params$", s):
            continue
        out.append(s)
    return out


# --------------------------------------------------------------------------
# Expression translation
# --------------------------------------------------------------------------

class Ctx:
    def __init__(self, fam: str, pol: bool, params: list[str], func: str = ""):
        self.func = func
        self.integrate_args = INTEGRATE_ARGS.get(func, [])
        self.fam = fam
        self.pol = pol
        self.params = set(params)
        self.locals: dict[str, str] = {}      # name -> "f64" | "bool"
        self.used: set[tuple[str, str]] = set()   # (module, fn name)
        self.used_consts: set[str] = set()
        self.inputs = INPUTS[fam]


def translate_expr(expr: str, ctx: Ctx) -> str:
    """C expression -> Rust expression, structure untouched."""
    protected: list[str] = []

    def protect(text: str) -> str:
        # The placeholder must contain no digits: the numeric-literal pass
        # below runs over the whole string and would otherwise rewrite the
        # placeholder's own index into a float.
        protected.append(text)
        n = len(protected) - 1
        tag = "".join(chr(ord("A") + int(c)) for c in str(n))
        return f"\x01{tag}\x01"

    # 1. input array refs -> the loop's aliases (pol) or a direct index (unpol)
    def inp(m: re.Match) -> str:
        name, k = m.group(1), int(m.group(2))
        if name not in ctx.inputs:
            raise Untranslatable(f"input array {name!r} not in family {ctx.fam}")
        if ctx.pol:
            return protect(f"{name}{k}")
        if k != 0:
            raise Untranslatable(f"unpolarized body indexes {name}[{k}]")
        return protect(f"{name}[ip]")

    expr = re.sub(r"\b(rho|sigma|lapl|tau)\[(\d+)\]", inp, expr)

    # 2. thresholds and functional parameters
    expr = re.sub(r"\bp->dens_threshold\b", lambda m: protect("dens_threshold"), expr)
    expr = re.sub(r"\bp->zeta_threshold\b", lambda m: protect("zeta_threshold"), expr)

    # Hybrid mixing data (`p->hyb_omega`, `p->hyb_coeff`) lives on the
    # functional rather than in its params struct, but reaches the kernel the
    # same way: one scalar per element.
    def hyb(m: re.Match) -> str:
        n = f"param_hyb_{m.group(1)}_{m.group(2)}"
        if n not in ctx.params:
            raise Untranslatable(f"unknown p->hyb_{m.group(1)}[{m.group(2)}]")
        return protect(n)

    expr = re.sub(r"\bp->hyb_(\w+)\[(\d+)\]", hyb, expr)

    # C's float.h epsilon.
    expr = re.sub(r"\bDBL_EPSILON\b", lambda m: protect("f64::EPSILON"), expr)

    # `xc_integrate(funcN, NULL, 0.0, x)` passes an integrand by pointer. The
    # math crate has each integrand pre-specialised, so the pointer becomes
    # part of the callee name and the functional's own parameters are appended
    # (the C integrand reads them off `params`).
    def integrate(m: re.Match) -> str:
        which, arg = m.group(1), m.group(2)
        name = INTEGRATE_NAME.get((ctx.func, which))
        if name is None:
            raise Untranslatable(
                f"no math-crate integrand for {ctx.func} xc_integrate({which})")
        ctx.used.add(("integrate", name))
        extra = "".join(", " + q for q in ctx.integrate_args)
        return protect(f"{name}(") + arg + protect(f"{extra})")

    expr = re.sub(r"\bxc_integrate\s*\(\s*(\w+)\s*,\s*NULL\s*,\s*[0-9.e+-]+\s*,\s*([^,()]+)\)",
                  integrate, expr)

    # `xbspline(u, ider, params)` / `cbspline(...)`: the params struct becomes
    # the flattened coefficient list.
    def bspline(m: re.Match) -> str:
        kind, u, ider = m.group(1), m.group(2), m.group(3)
        coeffs = [q for q in ctx.params if q.startswith(f"param_c{kind}_")]
        if not coeffs:
            raise Untranslatable(f"no b-spline coefficients for {kind}bspline")
        ctx.used.add(("bspline", f"case21_{kind}bspline"))
        order = sorted(coeffs, key=lambda q: int(q.rsplit("_", 1)[1]))
        return protect(f"case21_{kind}bspline(") + f"{u}, {ider}" + protect(
            "".join(", " + q for q in order) + ")")

    expr = re.sub(r"\b([xc])bspline\s*\(\s*([^,]+)\s*,\s*(\d+)\s*,\s*params\s*\)",
                  bspline, expr)

    def par(m: re.Match) -> str:
        # An array-valued parameter becomes one scalar per element:
        # `params->c_ab[0]` -> `param_c_ab_0`. Passing a slice instead would
        # put a bounds check and an indirection in the innermost loop for a
        # value that is a compile-time constant at every call site.
        n = m.group(1) + "".join(f"_{g}" for g in m.groups()[1:] if g is not None)
        if f"param_{n}" not in ctx.params:
            raise Untranslatable(f"unknown functional parameter params->{n}")
        return protect(f"param_{n}")

    expr = re.sub(r"\bparams->(\w+)(?:\[(\d+)\])?(?:\[(\d+)\])?", par, expr)

    # 3. calls
    def call(m: re.Match) -> str:
        n = m.group(1)
        if n in FUNCS:
            rust, mod = FUNCS[n]
            ctx.used.add((mod, rust))
            return protect(rust) + "("
        if n in LIBM:
            rust = LIBM[n]
            if rust == "POWF":
                return protect("rmath::pow") + "("
            if rust in ERF_MOD:
                ctx.used.add((ERF_MOD[rust], rust))
            return protect(rust) + "("
        return m.group(0)

    expr = re.sub(r"\b([A-Za-z_]\w*)\s*\(", call, expr)

    # 4. named constants
    def const(m: re.Match) -> str:
        n = m.group(0)
        if n in CONSTS:
            ctx.used_consts.add(n)
            return protect(n)
        return n

    expr = re.sub(r"\bM_[A-Z0-9_]+\b", const, expr)

    # 5. numeric literals. Safe now: every array index and identifier that
    #    could contain digits is behind a placeholder.
    expr = NUM.sub(lambda m: c_number_to_rust(m.group(1)), expr)

    # 6. anything still looking like an identifier must be a known local
    for m in IDENT.finditer(expr):
        n = m.group(0)
        if expr[m.start() - 1:m.start()] == "\x01":
            continue  # placeholder tag, restored below
        if n in ctx.locals or n == "ip":
            continue
        raise Untranslatable(f"unrecognised identifier {n!r}")

    for i, t in enumerate(protected):
        tag = "".join(chr(ord("A") + int(c)) for c in str(i))
        expr = expr.replace(f"\x01{tag}\x01", t)
    return expr


BOOL_TOP = re.compile(r"(?<![<>=!])(<=|>=|==|!=|<|>|&&|\|\|)(?![<>=])")


def is_bool_expr(expr: str) -> bool:
    """True if the *top level* of the expression is a comparison or logical
    connective, i.e. C stored a 0/1 in a double where Rust wants a `bool`."""
    depth = 0
    for i, ch in enumerate(expr):
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        elif depth == 0:
            m = BOOL_TOP.match(expr, i)
            if m:
                return True
    return False


# --------------------------------------------------------------------------
# Emission
# --------------------------------------------------------------------------

def mgga_outputs() -> list[list[str]]:
    """Order-sliced MGGA output names, read from libxc-core so the two cannot
    drift apart."""
    s = (REPO / "crates/libxc-core/src/output/mod.rs").read_text()
    m = re.search(r"pub struct MggaOutput<.*?\{(.*?)\n\}", s, re.S)
    names = re.findall(r"pub (\w+): Option<", m.group(1))

    def rank(n: str) -> int:
        if n == "zk":
            return 0
        mm = re.match(r"v(\d)", n)
        return int(mm.group(1)) if mm else 1

    return [[n for n in names if rank(n) == i] for i in range(5)]


OUT_WRITE = re.compile(
    r"^out->(\w+)\[\s*ip\s*\*\s*p->dim\.\w+\s*\+\s*(\d+)\s*\]\s*\+=\s*(.+)$")
ASSIGN = re.compile(r"^(\w+)\s*=\s*(.+)$")


def emit_function(fam: str, func: str, order: str, spin: str,
                  body: str, params: list[str],
                  vxc_type: bool = False) -> tuple[str, set[str], set[str]]:
    pol = spin == "pol"
    ctx = Ctx(fam, pol, params, func)
    oi = ORDERS.index(order)
    outs = (mgga_outputs() if fam == "mgga" else OUT_ORDER[fam])
    wanted = [n for grp in outs[:oi + 1] for n in grp]
    if vxc_type:
        wanted = [n for n in wanted if n != "zk"]
    if not wanted:
        raise Untranslatable(f"{order} has no outputs for this functional type")

    lines: list[str] = []
    seen_writes: set[str] = set()
    for st in statements(body):
        m = OUT_WRITE.match(st)
        if m:
            name, k, val = m.group(1), int(m.group(2)), m.group(3)
            if name not in wanted:
                raise Untranslatable(f"{order} writes unexpected output {name}")
            rhs = translate_expr(val, ctx)
            d = dim_of(name, fam, pol)
            if d == 1:
                idx = "ip"
            elif k == 0:
                idx = f"ip * {d}"
            else:
                idx = f"ip * {d} + {k}"
            lines.append(f"        {name}[{idx}] += {rhs};")
            seen_writes.add(name)
            continue
        m = ASSIGN.match(st)
        if not m:
            raise Untranslatable(f"unparsed statement: {st[:80]!r}")
        name, val = m.group(1), m.group(2)
        rhs = translate_expr(val, ctx)
        ctx.locals[name] = "bool" if is_bool_expr(val) else "f64"
        lines.append(f"        let {name} = {rhs};")

    # Signature
    sig = [f"    {n}: &[f64]," for n in ctx.inputs]
    sig += [f"    {n}: &mut [f64]," for n in wanted]
    sig += [f"    {p}: f64," for p in params]
    sig += ["    dens_threshold: f64,", "    zeta_threshold: f64,"]

    # Loop preamble: polarized bodies address rho[0]/rho[1] etc., so the loop
    # binds those once per point instead of re-indexing.
    pre: list[str] = []
    if pol:
        # Bind every component of every input, not only the ones this body
        # happens to read. Unused ones are dead and cost nothing, and it keeps
        # the preamble identical across orders of the same functional.
        for nm in ctx.inputs:
            d = dim_of(nm, fam, pol)
            for k in range(d):
                idx = f"ip * {d}" if k == 0 else f"ip * {d} + {k}"
                pre.append(f"        let {nm}{k} = {nm}[{idx}];")

    guard = wanted[0]
    gd = dim_of(guard, fam, pol)
    bound = f"{guard}.len()" if gd == 1 else f"{guard}.len() / {gd}"

    triple = (func, order, spin)
    if triple in SIMD_EXACT_FUNCS:
        head = "\n".join([
            f"//! {func.upper()} {order} {spin} kernel — explicit SIMD (bit-exact).",
            "//!",
            f"//! Auto-translated from `libxc-master/src/maple2c/{fam}_exc/{func}.c`",
            "//! by tools/translate_rayon/from_maple.py, then rewritten to",
            f"//! `wide::f64x8` by simd.py. {LANES_NOTE}",
            "",
        ])
        # The scalar path appends the two thresholds to the signature after
        # the functional's own parameters; the SIMD form takes the same
        # arguments in the same order, so they have to be included here too.
        in_dims = {n: dim_of(n, fam, pol) for n in ctx.inputs}
        out_dims = {n: dim_of(n, fam, pol) for n in wanted}
        body = simd_mod.simd_body(
            [l.strip() for l in lines], ctx.inputs, wanted,
            list(params) + ["dens_threshold", "zeta_threshold"],
            f"{func}_{order}_{spin}",
            in_dims=in_dims, out_dims=out_dims)
        return head + body, seen_writes, set(wanted)

    src = [
        f"//! {func.upper()} {order} {spin} kernel (rayon backend).",
        "//!",
        f"//! Auto-translated from `libxc-master/src/maple2c/{fam}_exc/{func}.c`",
        "//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact",
        "//! variable names and floating-point operation order.",
        "",
        "#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]",
        "",
        "use libxc_rkernel_math::rmath;",
    ]
    if ctx.used_consts:
        src.append("use libxc_rkernel_math::constants::{%s};" % ", ".join(sorted(ctx.used_consts)))
    by_mod: dict[str, set[str]] = {}
    for mod, name in ctx.used:
        by_mod.setdefault(mod, set()).add(name)
    for mod in sorted(by_mod):
        src.append(
            f"use libxc_rkernel_math::{mod}::{{{', '.join(sorted(by_mod[mod]))}}};")
    src += [
        "",
        "#[allow(unused_variables, non_snake_case)]",
        f"pub fn {func}_{order}_{spin}(",
        *sig,
        ") {",
        f"    for ip in 0..{bound} {{",
        *pre,
        *lines,
        "    }",
        "}",
        "",
    ]
    return "\n".join(src), seen_writes, set(wanted)


# --------------------------------------------------------------------------
# Driver
# --------------------------------------------------------------------------

def family_of(path: Path) -> str:
    return path.parent.name.split("_")[0]


def is_vxc_type(path: Path) -> bool:
    """`maple2c/<fam>_vxc/` holds potential-only functionals (`gga_x_lb`,
    `lda_xc_tih`): they define no energy density, so there is no `exc` function
    and `zk` is absent from every output list."""
    return path.parent.name.endswith("_vxc")


# Base kernels reachable only from a functional libxc keeps out of
# `xc_funcs.h`. Emitting one produces a crate nothing can reference: the eval
# layer is generated from `params.json`, which is itself filtered to the public
# header, so the dispatch module never appears.
#
# `lda_k_gds08_worker` is the only entry today. libxc declares it in
# `xc_funcs_worker.h` at id **100001**, does not resolve it through
# `xc_functional_get_number`, and returns it from no documented entry point.
# It could not have been called from here in any case -- `FunctionalId` is a
# `u16` and 100001 does not fit.
#
# Derived rather than hard-coded: a base is skipped when every functional that
# uses it is non-public. The four composites that mix this one
# (`gga_k_gds08`, `ghds10`, `ghds10r`, `tkvln`) are public and stay wired; they
# are simply not evaluable, which
# `verify/tests/composite_oracle.rs::KNOWN_GAPS` records with the reason.
def nonpublic_bases() -> set[str]:
    header = REPO / "libxc-master" / "src" / "xc_funcs.h"
    public = {m.group(1).lower() for m in
              re.finditer(r"^#define\s+XC_(\w+)\s+\d+", header.read_text(errors="replace"), re.M)}
    src = REPO / "libxc-master" / "src"
    skip: set[str] = set()
    for c in sorted(src.glob("*.c")):
        text = c.read_text(errors="replace")
        incs = re.findall(r"#include\s+[\"<]maple2c/[^/\">]+/([^/\">]+)\.c[\">]", text)
        if not incs:
            continue
        infos = re.findall(r"const\s+xc_func_info_type\s+xc_func_info_(\w+)\s*=", text)
        if infos and not any(i in public for i in infos):
            skip.update(incs)
    return skip


def maple_files() -> dict[str, Path]:
    skip = nonpublic_bases()
    out: dict[str, Path] = {}
    for d in ("lda_exc", "lda_vxc", "gga_exc", "gga_vxc", "mgga_exc", "mgga_vxc"):
        for p in sorted((MAPLE / d).glob("*.c")):
            if p.stem in skip:
                continue
            out[p.stem] = p
    return out


def load_params() -> dict:
    pj = REPO / "tools" / "translate_rayon" / "params.json"
    if not pj.is_file():
        print(f"missing {pj}; run extract_params.py --json {pj}", file=sys.stderr)
        sys.exit(2)
    return json.loads(pj.read_text())["resolved"]


def params_of(src: str, known: list[str] | None) -> list[str]:
    """Parameter list for the kernel signature.

    Taken from the C itself, in first-appearance order, so a functional whose
    `ext_params` defaults could not be resolved still gets a kernel -- it is
    only the *dispatch* that cannot be wired for those (see
    routing.rs UNSUPPORTED), not the kernel. When `params.json` does know the
    functional its order wins, because `gen_eval.py` passes the defaults
    positionally and the two must agree."""
    found: list[str] = []
    for m in re.finditer(r"\bparams->(\w+)(?:\[(\d+)\])?(?:\[(\d+)\])?", src):
        n = "param_" + m.group(1) + "".join(
            f"_{g}" for g in m.groups()[1:] if g is not None)
        if n not in found:
            found.append(n)
    # Hybrid mixing data is reached through `p->`, not `params->`, but is still
    # a per-functional constant the kernel takes as an argument.
    for m in re.finditer(r"\bp->hyb_(\w+)\[(\d+)\]", src):
        n = f"param_hyb_{m.group(1)}_{m.group(2)}"
        if n not in found:
            found.append(n)
    # `xbspline(u, ider, params)` hands the whole params struct to the spline,
    # so its coefficients never appear as `params->cx[k]` and have to be added
    # from the spline's own arity.
    for kind in ("x", "c"):
        if re.search(rf"\b{kind}bspline\s*\(", src):
            for k in range(BSPLINE_COEFFS):
                n = f"param_c{kind}_{k}"
                if n not in found:
                    found.append(n)
    if known is None:
        return found
    missing = [n for n in found if n not in known]
    if missing:
        raise Untranslatable(
            f"params.json is missing {missing}; its order drives the call site")
    return known


def emit_functional(func: str, path: Path, params: list[str] | None,
                    dry: bool) -> tuple[int, list[str]]:
    fam = family_of(path)
    text = path.read_text(errors="ignore")
    try:
        params = params_of(strip_comments(text), params)
    except Untranslatable as e:
        return 0, [f"{func}: {e}"]
    fns = split_functions(text)
    written, failed, cleared = 0, [], False
    crate = OUT / fam / func
    for (order, spin), body in sorted(fns.items()):
        try:
            src, _, _ = emit_function(fam, func, order, spin, body, params,
                                      is_vxc_type(path))
        except Untranslatable as e:
            failed.append(f"{func} {order}_{spin}: {e}")
            continue
        if not dry:
            if not cleared:
                # Wipe src/ rather than overwriting file by file. The previous
                # emitter split large outputs into directories (`lxc_pol/`)
                # that would otherwise sit alongside the new `lxc_pol.rs` and
                # collide as two definitions of the same module.
                if (crate / "src").is_dir():
                    shutil.rmtree(crate / "src")
                (crate / "src").mkdir(parents=True)
                cleared = True
            (crate / "src" / f"{order}_{spin}.rs").write_text(src)
        written += 1
    if written and not dry:
        mods = sorted(p.stem for p in (crate / "src").glob("*.rs") if p.stem != "lib")
        (crate / "src" / "lib.rs").write_text(
            f"//! {func.upper()} rayon kernels, generated by "
            "tools/translate_rayon/from_maple.py.\n\n"
            + "\n".join(f"pub mod {m};" for m in mods) + "\n")
        (crate / "Cargo.toml").write_text(
            f'[package]\nname = "libxc-rkernel-{func}"\nversion = "0.1.0"\n'
            'edition = "2024"\n\n[dependencies]\n'
            'libxc-rkernel-math = { path = "../../math" }\n')
    return written, failed


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--all", action="store_true")
    ap.add_argument("--func", action="append", default=[])
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    files = maple_files()
    resolved = load_params()
    names = args.func or (sorted(files) if args.all else [])
    if not names:
        ap.error("pass --all or --func NAME")

    tot_fn, all_failed, no_params, done = 0, [], [], 0
    for func in names:
        if func not in files:
            print(f"  no maple2c source for {func}", file=sys.stderr)
            continue
        info = resolved.get(func)
        if info is None:
            no_params.append(func)
        n, failed = emit_functional(
            func, files[func], info["params"] if info else None, args.dry_run)
        tot_fn += n
        all_failed += failed
        if n:
            done += 1

    print(f"emitted {tot_fn} kernel functions across {done} functionals"
          + (" (dry run)" if args.dry_run else ""))
    if no_params:
        print(f"{len(no_params)} have unresolved ext_params defaults: kernels "
              f"emitted, dispatch stays unwired (routing.rs UNSUPPORTED)")
    if all_failed:
        print(f"\n{len(all_failed)} untranslatable:")
        for f in all_failed[:40]:
            print("   ", f)
    return 0


if __name__ == "__main__":
    sys.exit(main())
