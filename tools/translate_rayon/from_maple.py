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
    # Added by tools/translate_rayon/simd_qualify.py; each line's
    # ratio is sweep ns/pt before -> after, fingerprint unchanged.
    ("gga_c_acgga", "exc", "pol"),  # 1.17x  (48.48 -> 41.36 ns/pt)
    ("gga_c_acgga", "vxc", "pol"),  # 1.32x  (56.47 -> 42.64 ns/pt)
    ("gga_c_acggap", "exc", "pol"),  # 1.16x  (48.70 -> 41.95 ns/pt)
    ("gga_c_acggap", "vxc", "pol"),  # 1.36x  (58.19 -> 42.69 ns/pt)
    ("gga_c_am05", "exc", "pol"),  # 1.18x  (50.89 -> 43.10 ns/pt)
    ("gga_c_am05", "vxc", "pol"),  # 1.25x  (52.74 -> 42.02 ns/pt)
    ("gga_c_bmk", "exc", "pol"),  # 1.36x  (63.01 -> 46.48 ns/pt)
    ("gga_c_bmk", "vxc", "pol"),  # 1.59x  (79.30 -> 49.88 ns/pt)
    ("gga_c_ccdf", "exc", "pol"),  # 1.03x  (36.80 -> 35.55 ns/pt)
    ("gga_c_ccdf", "vxc", "pol"),  # 1.06x  (37.52 -> 35.29 ns/pt)
    ("gga_c_chachiyo", "exc", "pol"),  # 1.12x  (45.16 -> 40.41 ns/pt)
    ("gga_c_chachiyo", "vxc", "pol"),  # 1.16x  (46.26 -> 39.84 ns/pt)
    ("gga_c_cs1", "exc", "pol"),  # 1.11x  (41.13 -> 36.89 ns/pt)
    ("gga_c_cs1", "vxc", "pol"),  # 1.20x  (44.43 -> 36.94 ns/pt)
    ("gga_c_gapc", "exc", "pol"),  # 1.20x  (50.56 -> 42.18 ns/pt)
    ("gga_c_gapc", "vxc", "pol"),  # 1.47x  (67.68 -> 45.95 ns/pt)
    ("gga_c_gaploc", "exc", "pol"),  # 1.25x  (54.92 -> 44.10 ns/pt)
    ("gga_c_gaploc", "vxc", "pol"),  # 1.50x  (70.60 -> 46.92 ns/pt)
    ("gga_c_hcth_a", "exc", "pol"),  # 1.29x  (67.93 -> 52.45 ns/pt)
    ("gga_c_hcth_a", "vxc", "pol"),  # 1.57x  (93.85 -> 59.73 ns/pt)
    ("gga_c_lm", "exc", "pol"),  # 1.16x  (50.03 -> 43.20 ns/pt)
    ("gga_c_lm", "vxc", "pol"),  # 1.23x  (53.37 -> 43.31 ns/pt)
    ("gga_c_lyp", "exc", "pol"),  # 1.19x  (49.87 -> 42.10 ns/pt)
    ("gga_c_lyp", "vxc", "pol"),  # 1.26x  (54.07 -> 42.95 ns/pt)
    ("gga_c_lypr", "exc", "pol"),  # 1.18x  (52.25 -> 44.21 ns/pt)
    ("gga_c_lypr", "vxc", "pol"),  # 1.34x  (65.83 -> 49.06 ns/pt)
    ("gga_c_op_b88", "exc", "pol"),  # 1.20x  (48.30 -> 40.17 ns/pt)
    ("gga_c_op_b88", "vxc", "pol"),  # 1.32x  (53.10 -> 40.17 ns/pt)
    ("gga_c_op_g96", "exc", "pol"),  # 1.13x  (44.79 -> 39.57 ns/pt)
    ("gga_c_op_g96", "vxc", "pol"),  # 1.18x  (46.92 -> 39.74 ns/pt)
    ("gga_c_op_pbe", "exc", "pol"),  # 1.13x  (44.70 -> 39.50 ns/pt)
    ("gga_c_op_pbe", "vxc", "pol"),  # 1.19x  (46.84 -> 39.30 ns/pt)
    ("gga_c_op_pw91", "exc", "pol"),  # 1.20x  (49.56 -> 41.27 ns/pt)
    ("gga_c_op_pw91", "vxc", "pol"),  # 1.34x  (56.78 -> 42.50 ns/pt)
    ("gga_c_op_xalpha", "exc", "pol"),  # 1.06x  (38.84 -> 36.59 ns/pt)
    ("gga_c_op_xalpha", "vxc", "pol"),  # 1.15x  (41.97 -> 36.43 ns/pt)
    ("gga_c_optc", "exc", "pol"),  # 1.53x  (85.96 -> 55.98 ns/pt)
    ("gga_c_optc", "vxc", "pol"),  # 1.98x  (132.08 -> 66.68 ns/pt)
    ("gga_c_p86", "exc", "pol"),  # 1.14x  (46.75 -> 41.06 ns/pt)
    ("gga_c_p86", "vxc", "pol"),  # 1.19x  (48.65 -> 41.02 ns/pt)
    ("gga_c_p86vwn", "exc", "pol"),  # 1.20x  (53.55 -> 44.71 ns/pt)
    ("gga_c_p86vwn", "vxc", "pol"),  # 1.28x  (61.09 -> 47.54 ns/pt)
    ("gga_c_pbe_erf_gws", "exc", "pol"),  # 1.34x  (65.45 -> 48.82 ns/pt)
    ("gga_c_pbe_erf_gws", "vxc", "pol"),  # 1.69x  (89.25 -> 52.93 ns/pt)
    ("gga_c_pbe_vwn", "exc", "pol"),  # 1.22x  (56.74 -> 46.41 ns/pt)
    ("gga_c_pbe_vwn", "vxc", "pol"),  # 1.40x  (66.03 -> 47.29 ns/pt)
    ("gga_c_pbeloc", "exc", "pol"),  # 1.17x  (48.24 -> 41.16 ns/pt)
    ("gga_c_pbeloc", "vxc", "pol"),  # 1.29x  (56.13 -> 43.51 ns/pt)
    ("gga_c_pw91", "exc", "pol"),  # 1.16x  (51.93 -> 44.81 ns/pt)
    ("gga_c_pw91", "vxc", "pol"),  # 1.38x  (62.79 -> 45.53 ns/pt)
    ("gga_c_q2d", "exc", "pol"),  # 1.31x  (57.63 -> 43.84 ns/pt)
    ("gga_c_q2d", "vxc", "pol"),  # 1.55x  (74.86 -> 48.42 ns/pt)
    ("gga_c_regtpss", "exc", "pol"),  # 1.21x  (48.65 -> 40.32 ns/pt)
    ("gga_c_regtpss", "vxc", "pol"),  # 1.34x  (55.16 -> 41.25 ns/pt)
    ("gga_c_revtca", "exc", "pol"),  # 1.10x  (49.35 -> 44.99 ns/pt)
    ("gga_c_revtca", "vxc", "pol"),  # 1.24x  (56.46 -> 45.41 ns/pt)
    ("gga_c_scan_e0", "exc", "pol"),  # 1.21x  (48.87 -> 40.44 ns/pt)
    ("gga_c_scan_e0", "vxc", "pol"),  # 1.28x  (52.67 -> 41.09 ns/pt)
    ("gga_c_sg4", "exc", "pol"),  # 1.18x  (50.02 -> 42.50 ns/pt)
    ("gga_c_sg4", "vxc", "pol"),  # 1.34x  (59.25 -> 44.21 ns/pt)
    ("gga_c_sogga11", "exc", "pol"),  # 1.13x  (45.76 -> 40.59 ns/pt)
    ("gga_c_sogga11", "vxc", "pol"),  # 1.25x  (51.59 -> 41.43 ns/pt)
    ("gga_c_tca", "exc", "pol"),  # 1.10x  (47.60 -> 43.45 ns/pt)
    ("gga_c_tca", "vxc", "pol"),  # 1.14x  (48.97 -> 42.94 ns/pt)
    ("gga_c_w94", "exc", "pol"),  # 1.08x  (40.52 -> 37.62 ns/pt)
    ("gga_c_w94", "vxc", "pol"),  # 1.11x  (41.45 -> 37.23 ns/pt)
    ("gga_c_wi", "exc", "pol"),  # 1.03x  (37.40 -> 36.22 ns/pt)
    ("gga_c_wi", "vxc", "pol"),  # 1.06x  (37.84 -> 35.77 ns/pt)
    ("gga_c_wl", "exc", "pol"),  # 1.06x  (39.45 -> 37.34 ns/pt)
    ("gga_c_wl", "vxc", "pol"),  # 1.07x  (40.34 -> 37.80 ns/pt)
    ("gga_c_zpbeint", "exc", "pol"),  # 1.17x  (52.28 -> 44.53 ns/pt)
    ("gga_c_zpbeint", "vxc", "pol"),  # 1.36x  (62.33 -> 45.70 ns/pt)
    ("gga_c_zvpbeint", "exc", "pol"),  # 1.23x  (53.85 -> 43.88 ns/pt)
    ("gga_c_zvpbeint", "vxc", "pol"),  # 1.38x  (63.24 -> 45.89 ns/pt)
    ("gga_c_zvpbeloc", "exc", "pol"),  # 1.24x  (54.16 -> 43.68 ns/pt)
    ("gga_c_zvpbeloc", "vxc", "pol"),  # 1.41x  (62.32 -> 44.26 ns/pt)
    ("gga_k_apbe", "exc", "pol"),  # 1.12x  (45.43 -> 40.45 ns/pt)
    ("gga_k_apbe", "vxc", "pol"),  # 1.15x  (45.88 -> 40.01 ns/pt)
    ("gga_k_apbeint", "exc", "pol"),  # 1.13x  (46.34 -> 40.99 ns/pt)
    ("gga_k_apbeint", "vxc", "pol"),  # 1.20x  (50.01 -> 41.66 ns/pt)
    ("gga_k_dk", "exc", "pol"),  # 1.15x  (45.29 -> 39.57 ns/pt)
    ("gga_k_dk", "vxc", "pol"),  # 1.21x  (48.52 -> 40.09 ns/pt)
    ("gga_k_exp4", "exc", "pol"),  # 1.18x  (52.78 -> 44.67 ns/pt)
    ("gga_k_exp4", "vxc", "pol"),  # 1.20x  (53.54 -> 44.75 ns/pt)
    ("gga_k_lc94", "exc", "pol"),  # 1.22x  (52.91 -> 43.47 ns/pt)
    ("gga_k_lc94", "vxc", "pol"),  # 1.30x  (56.77 -> 43.62 ns/pt)
    ("gga_k_lgap", "exc", "pol"),  # 1.12x  (47.03 -> 41.82 ns/pt)
    ("gga_k_lgap", "vxc", "pol"),  # 1.19x  (49.44 -> 41.70 ns/pt)
    ("gga_k_lgap_ge", "exc", "pol"),  # 1.12x  (45.71 -> 40.66 ns/pt)
    ("gga_k_lgap_ge", "vxc", "pol"),  # 1.17x  (47.60 -> 40.53 ns/pt)
    ("gga_k_lkt", "exc", "pol"),  # 1.20x  (51.67 -> 43.20 ns/pt)
    ("gga_k_llp", "exc", "pol"),  # 1.18x  (48.69 -> 41.17 ns/pt)
    ("gga_k_llp", "vxc", "pol"),  # 1.24x  (51.30 -> 41.23 ns/pt)
    ("gga_k_meyer", "exc", "pol"),  # 1.20x  (49.01 -> 40.73 ns/pt)
    ("gga_k_meyer", "vxc", "pol"),  # 1.26x  (52.76 -> 41.71 ns/pt)
    ("gga_k_ol1", "exc", "pol"),  # 1.11x  (44.58 -> 40.18 ns/pt)
    ("gga_k_ol1", "vxc", "pol"),  # 1.14x  (46.53 -> 40.76 ns/pt)
    ("gga_k_ol2", "exc", "pol"),  # 1.14x  (44.80 -> 39.44 ns/pt)
    ("gga_k_ol2", "vxc", "pol"),  # 1.16x  (45.85 -> 39.63 ns/pt)
    ("gga_k_pearson", "exc", "pol"),  # 1.14x  (45.26 -> 39.86 ns/pt)
    ("gga_k_pearson", "vxc", "pol"),  # 1.18x  (46.44 -> 39.21 ns/pt)
    ("gga_k_rational_p", "exc", "pol"),  # 1.26x  (53.72 -> 42.76 ns/pt)
    ("gga_k_rational_p", "vxc", "pol"),  # 1.28x  (56.11 -> 43.72 ns/pt)
    ("gga_k_thakkar", "exc", "pol"),  # 1.19x  (48.41 -> 40.67 ns/pt)
    ("gga_k_thakkar", "vxc", "pol"),  # 1.24x  (50.71 -> 40.76 ns/pt)
    ("gga_k_vt84f", "exc", "pol"),  # 1.20x  (53.62 -> 44.81 ns/pt)
    ("gga_k_vt84f", "vxc", "pol"),  # 1.28x  (58.33 -> 45.39 ns/pt)
    ("gga_x_2d_b88", "exc", "pol"),  # 1.06x  (38.92 -> 36.58 ns/pt)
    ("gga_x_2d_b88", "vxc", "pol"),  # 1.13x  (41.87 -> 37.14 ns/pt)
    ("gga_x_airy", "exc", "pol"),  # 1.36x  (64.10 -> 47.10 ns/pt)
    ("gga_x_airy", "vxc", "pol"),  # 1.41x  (80.99 -> 57.54 ns/pt)
    ("gga_x_b86", "exc", "pol"),  # 1.22x  (50.32 -> 41.36 ns/pt)
    ("gga_x_b86", "vxc", "pol"),  # 1.26x  (51.83 -> 41.23 ns/pt)
    ("gga_x_b88", "exc", "pol"),  # 1.17x  (50.36 -> 43.22 ns/pt)
    ("gga_x_b88", "vxc", "pol"),  # 1.25x  (53.62 -> 43.02 ns/pt)
    ("gga_x_bayesian", "exc", "pol"),  # 1.15x  (45.74 -> 39.92 ns/pt)
    ("gga_x_bayesian", "vxc", "pol"),  # 1.21x  (48.97 -> 40.45 ns/pt)
    ("gga_x_beefvdw", "exc", "pol"),  # 1.24x  (50.92 -> 40.93 ns/pt)
    ("gga_x_beefvdw", "vxc", "pol"),  # 1.42x  (59.60 -> 41.90 ns/pt)
    ("gga_x_bpccac", "exc", "pol"),  # 1.19x  (51.07 -> 42.89 ns/pt)
    ("gga_x_bpccac", "vxc", "pol"),  # 1.28x  (55.24 -> 43.17 ns/pt)
    ("gga_x_c09x", "exc", "pol"),  # 1.13x  (49.48 -> 43.63 ns/pt)
    ("gga_x_c09x", "vxc", "pol"),  # 1.18x  (51.40 -> 43.52 ns/pt)
    ("gga_x_cap", "exc", "pol"),  # 1.20x  (48.89 -> 40.76 ns/pt)
    ("gga_x_cap", "vxc", "pol"),  # 1.24x  (49.77 -> 40.28 ns/pt)
    ("gga_x_chachiyo", "exc", "pol"),  # 1.18x  (48.27 -> 40.81 ns/pt)
    ("gga_x_chachiyo", "vxc", "pol"),  # 1.24x  (50.60 -> 40.81 ns/pt)
    ("gga_x_ev93", "exc", "pol"),  # 1.15x  (46.79 -> 40.64 ns/pt)
    ("gga_x_ev93", "vxc", "pol"),  # 1.21x  (50.20 -> 41.66 ns/pt)
    ("gga_x_g96", "exc", "pol"),  # 1.09x  (45.17 -> 41.38 ns/pt)
    ("gga_x_g96", "vxc", "pol"),  # 1.14x  (46.13 -> 40.45 ns/pt)
    ("gga_x_hcth_a", "exc", "pol"),  # 1.18x  (48.86 -> 41.38 ns/pt)
    ("gga_x_hcth_a", "vxc", "pol"),  # 1.23x  (50.95 -> 41.33 ns/pt)
    ("gga_x_hjs_b88_v2", "exc", "pol"),  # 1.50x  (70.42 -> 47.00 ns/pt)
    ("gga_x_hjs_b88_v2", "vxc", "pol"),  # 1.82x  (94.19 -> 51.74 ns/pt)
    ("gga_x_htbs", "exc", "pol"),  # 1.20x  (51.31 -> 42.94 ns/pt)
    ("gga_x_htbs", "vxc", "pol"),  # 1.32x  (56.27 -> 42.74 ns/pt)
    ("gga_x_ityh", "exc", "pol"),  # 1.28x  (61.40 -> 47.96 ns/pt)
    ("gga_x_ityh", "vxc", "pol"),  # 1.51x  (75.77 -> 50.05 ns/pt)
    ("gga_x_ityh_optx", "exc", "pol"),  # 1.26x  (58.53 -> 46.26 ns/pt)
    ("gga_x_ityh_optx", "vxc", "pol"),  # 1.41x  (69.12 -> 49.19 ns/pt)
    ("gga_x_ityh_pbe", "exc", "pol"),  # 1.33x  (63.72 -> 47.74 ns/pt)
    ("gga_x_ityh_pbe", "vxc", "pol"),  # 1.45x  (74.07 -> 50.96 ns/pt)
    ("gga_x_lag", "exc", "pol"),  # 1.35x  (60.35 -> 44.69 ns/pt)
    ("gga_x_lag", "vxc", "pol"),  # 1.47x  (69.86 -> 47.49 ns/pt)
    ("gga_x_lg93", "exc", "pol"),  # 1.19x  (51.05 -> 42.93 ns/pt)
    ("gga_x_lg93", "vxc", "pol"),  # 1.35x  (60.43 -> 44.66 ns/pt)
    ("gga_x_lspbe", "exc", "pol"),  # 1.13x  (47.03 -> 41.56 ns/pt)
    ("gga_x_lspbe", "vxc", "pol"),  # 1.20x  (48.69 -> 40.55 ns/pt)
    ("gga_x_lsrpbe", "vxc", "pol"),  # 1.18x  (51.93 -> 43.92 ns/pt)
    ("gga_x_lv_rpw86", "exc", "pol"),  # 1.20x  (50.52 -> 42.01 ns/pt)
    ("gga_x_lv_rpw86", "vxc", "pol"),  # 1.26x  (53.41 -> 42.31 ns/pt)
    ("gga_x_mpbe", "exc", "pol"),  # 1.13x  (46.66 -> 41.19 ns/pt)
    ("gga_x_mpbe", "vxc", "pol"),  # 1.23x  (50.90 -> 41.53 ns/pt)
    ("gga_x_n12", "exc", "pol"),  # 1.19x  (50.98 -> 42.81 ns/pt)
    ("gga_x_n12", "vxc", "pol"),  # 1.29x  (56.67 -> 43.81 ns/pt)
    ("gga_x_ncap", "exc", "pol"),  # 1.29x  (55.27 -> 42.78 ns/pt)
    ("gga_x_ncap", "vxc", "pol"),  # 1.44x  (62.06 -> 43.05 ns/pt)
    ("gga_x_ol2", "exc", "pol"),  # 1.13x  (45.28 -> 39.96 ns/pt)
    ("gga_x_ol2", "vxc", "pol"),  # 1.22x  (47.32 -> 38.71 ns/pt)
    ("gga_x_optx", "exc", "pol"),  # 1.12x  (44.22 -> 39.41 ns/pt)
    ("gga_x_optx", "vxc", "pol"),  # 1.17x  (46.44 -> 39.87 ns/pt)
    ("gga_x_pbe_erf_gws", "exc", "pol"),  # 1.25x  (56.48 -> 45.20 ns/pt)
    ("gga_x_pbe_erf_gws", "vxc", "pol"),  # 1.45x  (68.38 -> 47.28 ns/pt)
    ("gga_x_pbea", "exc", "pol"),  # 1.22x  (50.85 -> 41.59 ns/pt)
    ("gga_x_pbea", "vxc", "pol"),  # 1.27x  (53.51 -> 42.01 ns/pt)
    ("gga_x_pbeint", "exc", "pol"),  # 1.14x  (46.44 -> 40.68 ns/pt)
    ("gga_x_pbeint", "vxc", "pol"),  # 1.22x  (50.14 -> 41.14 ns/pt)
    ("gga_x_pbepow", "exc", "pol"),  # 1.21x  (50.66 -> 41.91 ns/pt)
    ("gga_x_pbepow", "vxc", "pol"),  # 1.34x  (60.47 -> 45.17 ns/pt)
    ("gga_x_pbetrans", "exc", "pol"),  # 1.11x  (46.11 -> 41.40 ns/pt)
    ("gga_x_pbetrans", "vxc", "pol"),  # 1.20x  (49.58 -> 41.50 ns/pt)
    ("gga_x_pw86", "exc", "pol"),  # 1.21x  (50.84 -> 42.08 ns/pt)
    ("gga_x_pw86", "vxc", "pol"),  # 1.27x  (53.02 -> 41.82 ns/pt)
    ("gga_x_pw91", "exc", "pol"),  # 1.26x  (56.40 -> 44.88 ns/pt)
    ("gga_x_pw91", "vxc", "pol"),  # 1.34x  (60.82 -> 45.26 ns/pt)
    ("gga_x_q1d", "exc", "pol"),  # 1.16x  (46.75 -> 40.41 ns/pt)
    ("gga_x_q1d", "vxc", "pol"),  # 1.23x  (49.95 -> 40.69 ns/pt)
    ("gga_x_q2d", "exc", "pol"),  # 1.20x  (50.58 -> 42.28 ns/pt)
    ("gga_x_q2d", "vxc", "pol"),  # 1.41x  (62.77 -> 44.67 ns/pt)
    ("gga_x_rge2", "exc", "pol"),  # 1.15x  (45.88 -> 39.85 ns/pt)
    ("gga_x_rge2", "vxc", "pol"),  # 1.17x  (47.04 -> 40.12 ns/pt)
    ("gga_x_rpbe", "exc", "pol"),  # 1.15x  (47.73 -> 41.59 ns/pt)
    ("gga_x_rpbe", "vxc", "pol"),  # 1.17x  (47.52 -> 40.50 ns/pt)
    ("gga_x_sfat", "exc", "pol"),  # 1.32x  (71.73 -> 54.30 ns/pt)
    ("gga_x_sfat", "vxc", "pol"),  # 1.47x  (88.06 -> 59.73 ns/pt)
    ("gga_x_sfat_pbe", "vxc", "pol"),  # 1.40x  (81.07 -> 58.05 ns/pt)
    ("gga_x_sg4", "exc", "pol"),  # 1.14x  (45.99 -> 40.37 ns/pt)
    ("gga_x_sg4", "vxc", "pol"),  # 1.20x  (47.62 -> 39.85 ns/pt)
    ("gga_x_sogga11", "exc", "pol"),  # 1.18x  (48.63 -> 41.23 ns/pt)
    ("gga_x_sogga11", "vxc", "pol"),  # 1.25x  (51.95 -> 41.45 ns/pt)
    ("gga_x_ssb_sw", "exc", "pol"),  # 1.13x  (46.04 -> 40.59 ns/pt)
    ("gga_x_ssb_sw", "vxc", "pol"),  # 1.18x  (48.76 -> 41.17 ns/pt)
    ("gga_x_wc", "exc", "pol"),  # 1.18x  (52.93 -> 44.76 ns/pt)
    ("gga_xc_th1", "exc", "pol"),  # 1.17x  (49.32 -> 42.22 ns/pt)
    ("gga_xc_th1", "vxc", "pol"),  # 1.30x  (55.64 -> 42.84 ns/pt)
    ("gga_xc_th2", "exc", "pol"),  # 1.22x  (52.71 -> 43.40 ns/pt)
    ("gga_xc_th2", "vxc", "pol"),  # 1.35x  (61.77 -> 45.77 ns/pt)
    ("gga_xc_th3", "exc", "pol"),  # 1.23x  (57.81 -> 47.13 ns/pt)
    ("gga_xc_th3", "vxc", "pol"),  # 1.44x  (69.99 -> 48.64 ns/pt)
    ("hyb_gga_xc_wb97", "exc", "pol"),  # 1.48x  (79.28 -> 53.49 ns/pt)
    ("hyb_gga_xc_wb97", "vxc", "pol"),  # 1.78x  (116.80 -> 65.58 ns/pt)
    ("hyb_lda_xc_bn05", "exc", "pol"),  # 1.94x  (27.00 -> 13.90 ns/pt)
    ("hyb_lda_xc_bn05", "vxc", "pol"),  # 2.29x  (50.16 -> 21.94 ns/pt)
    ("hyb_mgga_x_dldf", "exc", "pol"),  # 1.34x  (25.52 -> 19.03 ns/pt)
    ("hyb_mgga_x_dldf", "vxc", "pol"),  # 1.45x  (27.71 -> 19.09 ns/pt)
    ("hyb_mgga_x_js18", "exc", "pol"),  # 1.83x  (105.09 -> 57.48 ns/pt)
    ("hyb_mgga_x_js18", "vxc", "pol"),  # 2.16x  (197.89 -> 91.67 ns/pt)
    ("hyb_mgga_x_m05", "exc", "pol"),  # 1.35x  (26.14 -> 19.41 ns/pt)
    ("hyb_mgga_x_m05", "vxc", "pol"),  # 1.56x  (31.72 -> 20.27 ns/pt)
    ("hyb_mgga_x_pjs18", "exc", "pol"),  # 1.89x  (89.30 -> 47.29 ns/pt)
    ("hyb_mgga_x_pjs18", "vxc", "pol"),  # 2.15x  (156.72 -> 72.81 ns/pt)
    ("hyb_mgga_xc_gas22", "exc", "pol"),  # 1.70x  (66.72 -> 39.35 ns/pt)
    ("hyb_mgga_xc_gas22", "vxc", "pol"),  # 2.13x  (115.62 -> 54.17 ns/pt)
    ("lda_c_2d_amgb", "exc", "pol"),  # 2.03x  (6.62 -> 3.27 ns/pt)
    ("lda_c_2d_amgb", "vxc", "pol"),  # 2.34x  (8.69 -> 3.71 ns/pt)
    ("lda_c_chachiyo", "exc", "pol"),  # 1.54x  (9.18 -> 5.96 ns/pt)
    ("lda_c_chachiyo", "vxc", "pol"),  # 1.67x  (10.09 -> 6.04 ns/pt)
    ("lda_c_chachiyo_mod", "exc", "pol"),  # 1.58x  (9.45 -> 5.97 ns/pt)
    ("lda_c_chachiyo_mod", "vxc", "pol"),  # 1.64x  (10.25 -> 6.26 ns/pt)
    ("lda_c_gk72", "exc", "pol"),  # 1.54x  (5.52 -> 3.59 ns/pt)
    ("lda_c_gk72", "vxc", "pol"),  # 1.75x  (7.14 -> 4.07 ns/pt)
    ("lda_c_gombas", "exc", "pol"),  # 1.78x  (4.53 -> 2.54 ns/pt)
    ("lda_c_gombas", "vxc", "pol"),  # 1.67x  (4.77 -> 2.85 ns/pt)
    ("lda_c_hl", "exc", "pol"),  # 1.60x  (9.45 -> 5.91 ns/pt)
    ("lda_c_hl", "vxc", "pol"),  # 1.68x  (10.34 -> 6.14 ns/pt)
    ("lda_c_ml1", "exc", "pol"),  # 1.86x  (12.91 -> 6.94 ns/pt)
    ("lda_c_ml1", "vxc", "pol"),  # 2.26x  (18.29 -> 8.08 ns/pt)
    ("lda_c_pmgb06", "exc", "pol"),  # 1.81x  (21.32 -> 11.76 ns/pt)
    ("lda_c_pmgb06", "vxc", "pol"),  # 2.49x  (36.24 -> 14.57 ns/pt)
    ("lda_c_pw", "exc", "pol"),  # 1.95x  (20.84 -> 10.67 ns/pt)
    ("lda_c_pw", "vxc", "pol"),  # 2.00x  (25.28 -> 12.64 ns/pt)
    ("lda_c_pw_erf", "exc", "pol"),  # 1.80x  (22.22 -> 12.33 ns/pt)
    ("lda_c_pw_erf", "vxc", "pol"),  # 2.13x  (34.21 -> 16.07 ns/pt)
    ("lda_c_pz", "exc", "pol"),  # 1.84x  (13.32 -> 7.23 ns/pt)
    ("lda_c_pz", "vxc", "pol"),  # 1.73x  (13.56 -> 7.86 ns/pt)
    ("lda_c_rc04", "exc", "pol"),  # 1.45x  (8.80 -> 6.08 ns/pt)
    ("lda_c_rc04", "vxc", "pol"),  # 1.55x  (9.47 -> 6.12 ns/pt)
    ("lda_c_rpa", "exc", "pol"),  # 1.39x  (4.86 -> 3.50 ns/pt)
    ("lda_c_rpa", "vxc", "pol"),  # 1.43x  (4.87 -> 3.41 ns/pt)
    ("lda_c_vwn_1", "exc", "pol"),  # 1.71x  (14.94 -> 8.75 ns/pt)
    ("lda_c_vwn_1", "vxc", "pol"),  # 1.85x  (17.98 -> 9.70 ns/pt)
    ("lda_c_vwn_2", "exc", "pol"),  # 1.65x  (27.50 -> 16.70 ns/pt)
    ("lda_c_vwn_2", "vxc", "pol"),  # 1.87x  (38.53 -> 20.58 ns/pt)
    ("lda_c_vwn_3", "exc", "pol"),  # 1.65x  (28.13 -> 17.08 ns/pt)
    ("lda_c_vwn_3", "vxc", "pol"),  # 1.98x  (39.61 -> 19.99 ns/pt)
    ("lda_c_vwn_4", "exc", "pol"),  # 1.84x  (17.15 -> 9.30 ns/pt)
    ("lda_c_vwn_4", "vxc", "pol"),  # 2.02x  (21.48 -> 10.64 ns/pt)
    ("lda_c_vwn_rpa", "exc", "pol"),  # 1.64x  (13.09 -> 8.00 ns/pt)
    ("lda_c_vwn_rpa", "vxc", "pol"),  # 1.95x  (16.56 -> 8.50 ns/pt)
    ("lda_c_w20", "exc", "pol"),  # 1.79x  (19.12 -> 10.66 ns/pt)
    ("lda_c_w20", "vxc", "pol"),  # 1.92x  (19.76 -> 10.28 ns/pt)
    ("lda_c_wigner", "exc", "pol"),  # 1.10x  (3.52 -> 3.19 ns/pt)
    ("lda_c_wigner", "vxc", "pol"),  # 1.25x  (3.88 -> 3.09 ns/pt)
    ("lda_k_tf", "exc", "pol"),  # 1.34x  (7.21 -> 5.39 ns/pt)
    ("lda_k_tf", "vxc", "pol"),  # 1.43x  (7.51 -> 5.27 ns/pt)
    ("lda_k_zlp", "exc", "pol"),  # 1.54x  (8.66 -> 5.64 ns/pt)
    ("lda_k_zlp", "vxc", "pol"),  # 1.52x  (8.81 -> 5.78 ns/pt)
    ("lda_x", "exc", "pol"),  # 1.56x  (7.49 -> 4.81 ns/pt)
    ("lda_x", "vxc", "pol"),  # 1.76x  (8.44 -> 4.80 ns/pt)
    ("lda_x_erf", "exc", "pol"),  # 1.84x  (23.03 -> 12.52 ns/pt)
    ("lda_x_erf", "vxc", "pol"),  # 2.01x  (33.27 -> 16.52 ns/pt)
    ("lda_x_rel", "exc", "pol"),  # 1.36x  (8.23 -> 6.05 ns/pt)
    ("lda_x_rel", "vxc", "pol"),  # 1.61x  (9.68 -> 6.01 ns/pt)
    ("lda_x_yukawa", "exc", "pol"),  # 1.66x  (25.00 -> 15.04 ns/pt)
    ("lda_x_yukawa", "vxc", "pol"),  # 2.17x  (34.67 -> 16.01 ns/pt)
    ("lda_xc_ksdt", "exc", "pol"),  # 2.27x  (39.59 -> 17.46 ns/pt)
    ("lda_xc_ksdt", "vxc", "pol"),  # 2.49x  (67.77 -> 27.22 ns/pt)
    ("lda_xc_teter93", "exc", "pol"),  # 1.43x  (8.10 -> 5.65 ns/pt)
    ("lda_xc_teter93", "vxc", "pol"),  # 1.64x  (9.75 -> 5.96 ns/pt)
    ("lda_xc_zlp", "exc", "pol"),  # 1.58x  (4.35 -> 2.76 ns/pt)
    ("lda_xc_zlp", "vxc", "pol"),  # 1.53x  (4.46 -> 2.91 ns/pt)
    ("mgga_c_b88", "exc", "pol"),  # 1.58x  (37.27 -> 23.55 ns/pt)
    ("mgga_c_b88", "vxc", "pol"),  # 1.86x  (46.21 -> 24.79 ns/pt)
    ("mgga_c_bc95", "exc", "pol"),  # 1.52x  (35.46 -> 23.26 ns/pt)
    ("mgga_c_bc95", "vxc", "pol"),  # 1.75x  (45.55 -> 26.01 ns/pt)
    ("mgga_c_cc", "exc", "pol"),  # 1.30x  (31.27 -> 24.12 ns/pt)
    ("mgga_c_cc", "vxc", "pol"),  # 1.55x  (38.50 -> 24.82 ns/pt)
    ("mgga_c_ccalda", "exc", "pol"),  # 1.39x  (30.66 -> 22.03 ns/pt)
    ("mgga_c_ccalda", "vxc", "pol"),  # 1.56x  (36.30 -> 23.24 ns/pt)
    ("mgga_c_kcis", "exc", "pol"),  # 1.93x  (75.74 -> 39.30 ns/pt)
    ("mgga_c_kcis", "vxc", "pol"),  # 2.54x  (141.38 -> 55.70 ns/pt)
    ("mgga_c_kcisk", "exc", "pol"),  # 2.03x  (87.22 -> 42.98 ns/pt)
    ("mgga_c_kcisk", "vxc", "pol"),  # 2.81x  (174.35 -> 62.06 ns/pt)
    ("mgga_c_m05", "exc", "pol"),  # 1.56x  (39.12 -> 25.09 ns/pt)
    ("mgga_c_m05", "vxc", "pol"),  # 1.98x  (55.56 -> 28.07 ns/pt)
    ("mgga_c_pkzb", "exc", "pol"),  # 1.83x  (62.35 -> 34.12 ns/pt)
    ("mgga_c_pkzb", "vxc", "pol"),  # 2.14x  (95.50 -> 44.64 ns/pt)
    ("mgga_c_r2scan", "exc", "pol"),  # 1.44x  (43.21 -> 29.95 ns/pt)
    ("mgga_c_r2scan", "vxc", "pol"),  # 1.82x  (67.25 -> 36.84 ns/pt)
    ("mgga_c_revscan", "exc", "pol"),  # 1.79x  (56.02 -> 31.34 ns/pt)
    ("mgga_c_revscan", "vxc", "pol"),  # 1.97x  (75.61 -> 38.33 ns/pt)
    ("mgga_c_revtpss", "exc", "pol"),  # 1.87x  (73.23 -> 39.07 ns/pt)
    ("mgga_c_revtpss", "vxc", "pol"),  # 2.81x  (133.17 -> 47.32 ns/pt)
    ("mgga_c_rmggac", "exc", "pol"),  # 1.34x  (40.10 -> 29.90 ns/pt)
    ("mgga_c_rmggac", "vxc", "pol"),  # 1.60x  (52.37 -> 32.78 ns/pt)
    ("mgga_c_rppscan", "exc", "pol"),  # 1.60x  (43.38 -> 27.10 ns/pt)
    ("mgga_c_rppscan", "vxc", "pol"),  # 1.94x  (58.19 -> 29.93 ns/pt)
    ("mgga_c_rregtm", "exc", "pol"),  # 1.62x  (49.09 -> 30.38 ns/pt)
    ("mgga_c_rscan", "exc", "pol"),  # 1.48x  (39.06 -> 26.43 ns/pt)
    ("mgga_c_rscan", "vxc", "pol"),  # 1.85x  (55.63 -> 30.05 ns/pt)
    ("mgga_c_scan", "exc", "pol"),  # 1.57x  (41.88 -> 26.72 ns/pt)
    ("mgga_c_scan", "vxc", "pol"),  # 2.06x  (56.91 -> 27.68 ns/pt)
    ("mgga_c_tpssloc", "exc", "pol"),  # 1.90x  (77.94 -> 41.00 ns/pt)
    ("mgga_c_tpssloc", "vxc", "pol"),  # 2.57x  (152.55 -> 59.45 ns/pt)
    ("mgga_c_vsxc", "exc", "pol"),  # 1.58x  (37.80 -> 23.86 ns/pt)
    ("mgga_c_vsxc", "vxc", "pol"),  # 1.90x  (51.94 -> 27.34 ns/pt)
    ("mgga_k_gea2", "exc", "pol"),  # 1.20x  (23.13 -> 19.22 ns/pt)
    ("mgga_k_gea2", "vxc", "pol"),  # 1.32x  (23.19 -> 17.56 ns/pt)
    ("mgga_k_gea4", "exc", "pol"),  # 1.28x  (24.12 -> 18.78 ns/pt)
    ("mgga_k_gea4", "vxc", "pol"),  # 1.37x  (25.51 -> 18.63 ns/pt)
    ("mgga_k_pc07", "exc", "pol"),  # 1.74x  (52.73 -> 30.23 ns/pt)
    ("mgga_k_pc07", "vxc", "pol"),  # 1.89x  (60.53 -> 32.00 ns/pt)
    ("mgga_k_rda", "exc", "pol"),  # 1.35x  (26.83 -> 19.80 ns/pt)
    ("mgga_k_rda", "vxc", "pol"),  # 1.53x  (31.14 -> 20.37 ns/pt)
    ("mgga_x_2d_js17", "exc", "pol"),  # 1.23x  (19.86 -> 16.11 ns/pt)
    ("mgga_x_2d_js17", "vxc", "pol"),  # 1.34x  (21.27 -> 15.88 ns/pt)
    ("mgga_x_br89_explicit", "exc", "pol"),  # 1.48x  (36.96 -> 25.01 ns/pt)
    ("mgga_x_br89_explicit", "vxc", "pol"),  # 1.83x  (49.69 -> 27.07 ns/pt)
    ("mgga_x_edmgga", "exc", "pol"),  # 1.50x  (31.06 -> 20.66 ns/pt)
    ("mgga_x_edmgga", "vxc", "pol"),  # 1.78x  (37.21 -> 20.88 ns/pt)
    ("mgga_x_ft98", "exc", "pol"),  # 1.34x  (28.28 -> 21.05 ns/pt)
    ("mgga_x_ft98", "vxc", "pol"),  # 1.73x  (36.93 -> 21.40 ns/pt)
    ("mgga_x_gvt4", "exc", "pol"),  # 1.23x  (23.95 -> 19.47 ns/pt)
    ("mgga_x_gvt4", "vxc", "pol"),  # 1.51x  (28.54 -> 18.85 ns/pt)
    ("mgga_x_gx", "exc", "pol"),  # 1.31x  (24.88 -> 18.96 ns/pt)
    ("mgga_x_gx", "vxc", "pol"),  # 1.51x  (28.01 -> 18.52 ns/pt)
    ("mgga_x_jk", "exc", "pol"),  # 1.32x  (27.13 -> 20.47 ns/pt)
    ("mgga_x_jk", "vxc", "pol"),  # 1.49x  (30.09 -> 20.23 ns/pt)
    ("mgga_x_lta", "exc", "pol"),  # 1.40x  (28.62 -> 20.39 ns/pt)
    ("mgga_x_lta", "vxc", "pol"),  # 1.49x  (29.07 -> 19.55 ns/pt)
    ("mgga_x_m11_l", "exc", "pol"),  # 1.64x  (57.97 -> 35.46 ns/pt)
    ("mgga_x_m11_l", "vxc", "pol"),  # 2.15x  (103.04 -> 47.88 ns/pt)
    ("mgga_x_mbeef", "exc", "pol"),  # 1.64x  (33.55 -> 20.45 ns/pt)
    ("mgga_x_mbeef", "vxc", "pol"),  # 2.48x  (63.30 -> 25.51 ns/pt)
    ("mgga_x_mbeefvdw", "exc", "pol"),  # 1.46x  (28.27 -> 19.43 ns/pt)
    ("mgga_x_mbeefvdw", "vxc", "pol"),  # 1.99x  (42.87 -> 21.60 ns/pt)
    ("mgga_x_mcml", "exc", "pol"),  # 1.62x  (33.40 -> 20.66 ns/pt)
    ("mgga_x_mcml", "vxc", "pol"),  # 2.50x  (65.54 -> 26.25 ns/pt)
    ("mgga_x_mvs", "exc", "pol"),  # 1.43x  (29.95 -> 20.88 ns/pt)
    ("mgga_x_mvs", "vxc", "pol"),  # 1.64x  (34.08 -> 20.85 ns/pt)
    ("mgga_x_mvsb", "exc", "pol"),  # 1.51x  (31.47 -> 20.88 ns/pt)
    ("mgga_x_mvsb", "vxc", "pol"),  # 1.60x  (35.78 -> 22.34 ns/pt)
    ("mgga_x_pbe_gx", "exc", "pol"),  # 1.35x  (24.98 -> 18.52 ns/pt)
    ("mgga_x_pbe_gx", "vxc", "pol"),  # 1.49x  (28.22 -> 18.93 ns/pt)
    ("mgga_x_pkzb", "exc", "pol"),  # 1.31x  (24.46 -> 18.67 ns/pt)
    ("mgga_x_pkzb", "vxc", "pol"),  # 1.41x  (25.77 -> 18.30 ns/pt)
    ("mgga_x_r2scan", "exc", "pol"),  # 1.41x  (33.99 -> 24.14 ns/pt)
    ("mgga_x_r2scan", "vxc", "pol"),  # 1.69x  (45.65 -> 27.08 ns/pt)
    ("mgga_x_r4scan", "exc", "pol"),  # 1.52x  (36.69 -> 24.08 ns/pt)
    ("mgga_x_r4scan", "vxc", "pol"),  # 2.01x  (57.50 -> 28.55 ns/pt)
    ("mgga_x_regtm", "exc", "pol"),  # 1.39x  (39.38 -> 28.37 ns/pt)
    ("mgga_x_regtm", "vxc", "pol"),  # 1.60x  (51.96 -> 32.43 ns/pt)
    ("mgga_x_regtpss", "exc", "pol"),  # 1.46x  (29.97 -> 20.57 ns/pt)
    ("mgga_x_regtpss", "vxc", "pol"),  # 1.82x  (42.46 -> 23.27 ns/pt)
    ("mgga_x_revtm", "exc", "pol"),  # 1.57x  (38.80 -> 24.73 ns/pt)
    ("mgga_x_revtm", "vxc", "pol"),  # 1.95x  (47.31 -> 24.23 ns/pt)
    ("mgga_x_rlda", "exc", "pol"),  # 1.21x  (22.52 -> 18.62 ns/pt)
    ("mgga_x_rlda", "vxc", "pol"),  # 1.31x  (23.34 -> 17.83 ns/pt)
    ("mgga_x_rppscan", "exc", "pol"),  # 1.43x  (35.34 -> 24.73 ns/pt)
    ("mgga_x_rppscan", "vxc", "pol"),  # 1.76x  (42.04 -> 23.84 ns/pt)
    ("mgga_x_rscan", "exc", "pol"),  # 1.53x  (37.92 -> 24.75 ns/pt)
    ("mgga_x_rscan", "vxc", "pol"),  # 1.95x  (57.86 -> 29.68 ns/pt)
    ("mgga_x_rtpss", "exc", "pol"),  # 1.42x  (29.53 -> 20.79 ns/pt)
    ("mgga_x_rtpss", "vxc", "pol"),  # 1.82x  (40.44 -> 22.22 ns/pt)
    ("mgga_x_sa_tpss", "exc", "pol"),  # 1.55x  (32.02 -> 20.69 ns/pt)
    ("mgga_x_sa_tpss", "vxc", "pol"),  # 1.99x  (46.92 -> 23.61 ns/pt)
    ("mgga_x_task", "exc", "pol"),  # 1.56x  (35.83 -> 22.91 ns/pt)
    ("mgga_x_task", "vxc", "pol"),  # 2.13x  (50.12 -> 23.57 ns/pt)
    ("mgga_x_tau_hcth", "exc", "pol"),  # 1.36x  (25.46 -> 18.68 ns/pt)
    ("mgga_x_tau_hcth", "vxc", "pol"),  # 1.46x  (28.61 -> 19.54 ns/pt)
    ("mgga_x_th", "exc", "pol"),  # 1.25x  (22.68 -> 18.20 ns/pt)
    ("mgga_x_th", "vxc", "pol"),  # 1.29x  (23.26 -> 17.96 ns/pt)
    ("mgga_x_tm", "exc", "pol"),  # 1.63x  (36.18 -> 22.23 ns/pt)
    ("mgga_x_tm", "vxc", "pol"),  # 1.72x  (41.78 -> 24.25 ns/pt)
    ("mgga_x_tpss", "exc", "pol"),  # 1.71x  (36.74 -> 21.50 ns/pt)
    ("mgga_x_tpss", "vxc", "pol"),  # 2.06x  (49.42 -> 24.03 ns/pt)
    ("mgga_x_vcml", "exc", "pol"),  # 1.64x  (33.23 -> 20.33 ns/pt)
    ("mgga_x_vcml", "vxc", "pol"),  # 2.50x  (64.66 -> 25.83 ns/pt)
    ("mgga_x_vt84", "exc", "pol"),  # 1.49x  (34.34 -> 23.01 ns/pt)
    ("mgga_x_vt84", "vxc", "pol"),  # 1.94x  (51.28 -> 26.49 ns/pt)
    ("mgga_xc_cc06", "exc", "pol"),  # 1.42x  (34.05 -> 23.96 ns/pt)
    ("mgga_xc_cc06", "vxc", "pol"),  # 1.58x  (37.05 -> 23.48 ns/pt)
    ("mgga_xc_lp90", "exc", "pol"),  # 1.22x  (20.80 -> 17.09 ns/pt)
    ("mgga_xc_lp90", "vxc", "pol"),  # 1.28x  (21.10 -> 16.43 ns/pt)
    ("mgga_xc_zlp", "exc", "pol"),  # 1.23x  (22.12 -> 18.03 ns/pt)
    ("mgga_xc_zlp", "vxc", "pol"),  # 1.27x  (22.14 -> 17.45 ns/pt)
    # Added by tools/translate_rayon/simd_qualify.py; each line's
    # ratio is sweep ns/pt before -> after, fingerprint unchanged.
    ("gga_c_op_xalpha", "exc", "unpol"),  # 1.59x  (7.38 -> 4.63 ns/pt)
    ("gga_c_w94", "exc", "unpol"),  # 1.62x  (7.67 -> 4.72 ns/pt)
    ("gga_c_w94", "vxc", "unpol"),  # 1.62x  (8.01 -> 4.95 ns/pt)
    ("gga_k_lkt", "vxc", "pol"),  # 1.30x  (58.46 -> 45.02 ns/pt)
    ("gga_k_thakkar", "vxc", "unpol"),  # 2.08x  (12.20 -> 5.87 ns/pt)
    ("gga_x_ak13", "exc", "pol"),  # 1.20x  (53.74 -> 44.86 ns/pt)
    ("gga_x_ak13", "vxc", "pol"),  # 1.26x  (57.32 -> 45.58 ns/pt)
    ("gga_x_lsrpbe", "exc", "pol"),  # 1.17x  (51.96 -> 44.46 ns/pt)
    ("gga_x_sfat_pbe", "exc", "pol"),  # 1.33x  (69.88 -> 52.66 ns/pt)
    ("gga_x_wc", "vxc", "pol"),  # 1.23x  (55.30 -> 45.00 ns/pt)
    ("mgga_c_cc", "vxc", "unpol"),  # 1.68x  (10.48 -> 6.25 ns/pt)
    ("mgga_c_cs", "exc", "pol"),  # 1.25x  (31.69 -> 25.35 ns/pt)
    ("mgga_c_cs", "vxc", "pol"),  # 1.43x  (34.66 -> 24.21 ns/pt)
    ("mgga_c_rregtm", "vxc", "pol"),  # 1.98x  (61.89 -> 31.32 ns/pt)
    ("mgga_c_tpss", "exc", "pol"),  # 2.41x  (97.61 -> 40.52 ns/pt)
    ("mgga_c_tpss", "vxc", "pol"),  # 2.87x  (151.50 -> 52.88 ns/pt)
    ("mgga_xc_cc06", "exc", "unpol"),  # 1.68x  (9.79 -> 5.84 ns/pt)
    ("mgga_xc_cc06", "vxc", "unpol"),  # 1.98x  (12.46 -> 6.30 ns/pt)
    # Added by tools/translate_rayon/simd_qualify.py; each line's
    # ratio is sweep ns/pt before -> after, fingerprint unchanged.
    ("gga_c_gaploc", "fxc", "unpol"),  # 3.07x  (103.20 -> 33.60 ns/pt)
    ("gga_c_hcth_a", "fxc", "unpol"),  # 2.18x  (45.58 -> 20.93 ns/pt)
    ("gga_c_lm", "fxc", "unpol"),  # 2.00x  (19.40 -> 9.72 ns/pt)
    ("gga_c_lyp", "fxc", "unpol"),  # 2.07x  (13.54 -> 6.55 ns/pt)
    ("gga_c_optc", "fxc", "unpol"),  # 2.35x  (54.94 -> 23.33 ns/pt)
    ("gga_c_p86vwn", "fxc", "unpol"),  # 2.15x  (31.39 -> 14.63 ns/pt)
    ("gga_c_pbe_erf_gws", "fxc", "unpol"),  # 2.68x  (55.56 -> 20.73 ns/pt)
    ("gga_c_pbe_vwn", "fxc", "unpol"),  # 2.24x  (34.12 -> 15.20 ns/pt)
    ("gga_c_pw91", "fxc", "unpol"),  # 2.44x  (32.42 -> 13.32 ns/pt)
    ("gga_c_q2d", "fxc", "unpol"),  # 2.61x  (49.73 -> 19.07 ns/pt)
    ("gga_c_revtca", "fxc", "unpol"),  # 1.61x  (16.88 -> 10.46 ns/pt)
    ("gga_c_tca", "fxc", "unpol"),  # 1.63x  (17.06 -> 10.47 ns/pt)
    ("gga_c_zvpbeloc", "fxc", "unpol"),  # 2.50x  (40.08 -> 16.04 ns/pt)
    ("gga_k_lkt", "fxc", "unpol"),  # 2.23x  (12.40 -> 5.57 ns/pt)
    ("gga_x_airy", "fxc", "unpol"),  # 1.78x  (40.73 -> 22.84 ns/pt)
    ("gga_x_b88", "fxc", "unpol"),  # 1.91x  (13.96 -> 7.33 ns/pt)
    ("gga_x_ityh_pbe", "fxc", "unpol"),  # 2.38x  (28.19 -> 11.83 ns/pt)
    ("gga_x_lag", "fxc", "unpol"),  # 1.99x  (27.20 -> 13.69 ns/pt)
    ("gga_x_lg93", "fxc", "unpol"),  # 2.06x  (21.13 -> 10.24 ns/pt)
    ("gga_x_pbe_erf_gws", "fxc", "unpol"),  # 2.41x  (27.33 -> 11.34 ns/pt)
    ("gga_x_pbea", "fxc", "unpol"),  # 1.88x  (11.71 -> 6.21 ns/pt)
    ("gga_x_pw91", "fxc", "unpol"),  # 2.11x  (22.98 -> 10.91 ns/pt)
    ("gga_x_q2d", "fxc", "unpol"),  # 2.08x  (15.57 -> 7.48 ns/pt)
    ("gga_x_sfat", "fxc", "unpol"),  # 2.69x  (35.85 -> 13.33 ns/pt)
    ("gga_x_sfat_pbe", "fxc", "unpol"),  # 2.34x  (41.52 -> 17.78 ns/pt)
    ("gga_xc_th2", "fxc", "unpol"),  # 2.03x  (14.20 -> 6.99 ns/pt)
    ("gga_xc_th3", "fxc", "unpol"),  # 2.11x  (23.10 -> 10.96 ns/pt)
    ("hyb_gga_xc_wb97", "fxc", "unpol"),  # 2.59x  (58.78 -> 22.67 ns/pt)
    ("hyb_mgga_x_js18", "fxc", "unpol"),  # 2.74x  (155.74 -> 56.81 ns/pt)
    ("hyb_mgga_x_pjs18", "fxc", "unpol"),  # 2.60x  (110.55 -> 42.50 ns/pt)
    ("hyb_mgga_xc_gas22", "fxc", "unpol"),  # 2.64x  (65.77 -> 24.93 ns/pt)
    ("lda_c_ml1", "fxc", "unpol"),  # 1.67x  (12.34 -> 7.37 ns/pt)
    ("lda_c_pw", "fxc", "unpol"),  # 2.30x  (18.10 -> 7.85 ns/pt)
    ("lda_c_pz", "fxc", "unpol"),  # 1.75x  (9.24 -> 5.27 ns/pt)
    ("lda_c_vwn", "fxc", "unpol"),  # 2.07x  (19.65 -> 9.50 ns/pt)
    ("lda_c_vwn_1", "fxc", "unpol"),  # 1.79x  (12.63 -> 7.05 ns/pt)
    ("lda_c_vwn_2", "fxc", "unpol"),  # 2.42x  (37.52 -> 15.49 ns/pt)
    ("lda_c_vwn_3", "fxc", "unpol"),  # 2.31x  (39.08 -> 16.90 ns/pt)
    ("lda_c_vwn_4", "fxc", "unpol"),  # 2.02x  (15.59 -> 7.72 ns/pt)
    ("lda_c_vwn_rpa", "fxc", "unpol"),  # 2.05x  (11.61 -> 5.66 ns/pt)
    ("lda_c_w20", "fxc", "unpol"),  # 2.42x  (27.14 -> 11.19 ns/pt)
    ("lda_xc_ksdt", "fxc", "unpol"),  # 2.97x  (61.94 -> 20.88 ns/pt)
    ("mgga_c_kcis", "fxc", "unpol"),  # 3.36x  (64.78 -> 19.27 ns/pt)
    ("mgga_c_kcisk", "fxc", "unpol"),  # 2.96x  (96.02 -> 32.47 ns/pt)
    ("mgga_c_r2scan", "fxc", "unpol"),  # 2.75x  (68.71 -> 24.96 ns/pt)
    ("mgga_c_revscan", "fxc", "unpol"),  # 2.73x  (54.74 -> 20.02 ns/pt)
    ("mgga_c_revtpss", "fxc", "unpol"),  # 3.11x  (133.60 -> 43.00 ns/pt)
    ("mgga_c_rregtm", "fxc", "unpol"),  # 2.71x  (42.23 -> 15.59 ns/pt)
    ("mgga_c_rscan", "fxc", "unpol"),  # 2.79x  (50.30 -> 18.02 ns/pt)
    ("mgga_c_scan", "fxc", "unpol"),  # 2.70x  (42.05 -> 15.56 ns/pt)
    ("mgga_c_tpss", "fxc", "unpol"),  # 2.74x  (103.54 -> 37.81 ns/pt)
    ("mgga_c_tpssloc", "fxc", "unpol"),  # 2.59x  (118.20 -> 45.58 ns/pt)
    ("mgga_k_pc07", "fxc", "unpol"),  # 2.25x  (31.68 -> 14.10 ns/pt)
    ("mgga_x_m11_l", "fxc", "unpol"),  # 2.79x  (78.94 -> 28.28 ns/pt)
    ("mgga_x_r2scan", "fxc", "unpol"),  # 2.88x  (39.91 -> 13.88 ns/pt)
    ("mgga_x_r4scan", "fxc", "unpol"),  # 3.18x  (47.80 -> 15.05 ns/pt)
    ("mgga_x_regtm", "fxc", "unpol"),  # 3.11x  (39.61 -> 12.74 ns/pt)
    ("mgga_x_revtm", "fxc", "unpol"),  # 3.11x  (31.24 -> 10.04 ns/pt)
    ("mgga_x_rppscan", "fxc", "unpol"),  # 2.93x  (29.51 -> 10.06 ns/pt)
    ("mgga_x_rscan", "fxc", "unpol"),  # 2.79x  (46.00 -> 16.49 ns/pt)
    ("mgga_x_scan", "fxc", "unpol"),  # 2.59x  (34.33 -> 13.25 ns/pt)
    ("mgga_x_task", "fxc", "unpol"),  # 2.72x  (32.11 -> 11.80 ns/pt)
    ("mgga_x_tm", "fxc", "unpol"),  # 2.79x  (27.66 -> 9.90 ns/pt)
    ("mgga_x_tpss", "fxc", "unpol"),  # 2.47x  (33.92 -> 13.76 ns/pt)
    # Added by tools/translate_rayon/simd_qualify.py; each line's
    # ratio is sweep ns/pt before -> after, fingerprint unchanged.
    ("gga_c_acgga", "fxc", "unpol"),  # 2.58x  (26.62 -> 10.34 ns/pt)
    ("gga_c_acggap", "fxc", "unpol"),  # 2.71x  (31.02 -> 11.46 ns/pt)
    ("gga_c_am05", "fxc", "unpol"),  # 2.08x  (12.90 -> 6.21 ns/pt)
    ("gga_c_bmk", "fxc", "unpol"),  # 2.50x  (27.08 -> 10.81 ns/pt)
    ("gga_c_chachiyo", "fxc", "unpol"),  # 1.96x  (11.84 -> 6.05 ns/pt)
    ("gga_c_gapc", "fxc", "unpol"),  # 3.48x  (97.97 -> 28.17 ns/pt)
    ("gga_c_lypr", "fxc", "unpol"),  # 2.00x  (20.88 -> 10.46 ns/pt)
    ("gga_c_op_pbe", "fxc", "unpol"),  # 2.30x  (15.80 -> 6.88 ns/pt)
    ("gga_c_op_pw91", "fxc", "unpol"),  # 2.39x  (21.56 -> 9.02 ns/pt)
    ("gga_c_p86", "fxc", "unpol"),  # 2.02x  (14.07 -> 6.96 ns/pt)
    ("gga_c_pbeloc", "fxc", "unpol"),  # 2.44x  (23.40 -> 9.57 ns/pt)
    ("gga_c_regtpss", "fxc", "unpol"),  # 2.60x  (23.97 -> 9.21 ns/pt)
    ("gga_c_scan_e0", "fxc", "unpol"),  # 2.46x  (20.04 -> 8.13 ns/pt)
    ("gga_c_sg4", "fxc", "unpol"),  # 2.56x  (30.86 -> 12.06 ns/pt)
    ("gga_c_zpbeint", "fxc", "unpol"),  # 2.49x  (25.38 -> 10.18 ns/pt)
    ("gga_c_zvpbeint", "fxc", "unpol"),  # 2.37x  (24.23 -> 10.24 ns/pt)
    ("gga_k_exp4", "fxc", "unpol"),  # 1.77x  (10.99 -> 6.20 ns/pt)
    ("gga_k_lc94", "fxc", "unpol"),  # 2.08x  (18.29 -> 8.80 ns/pt)
    ("gga_k_rational_p", "fxc", "unpol"),  # 1.83x  (10.98 -> 6.00 ns/pt)
    ("gga_k_vt84f", "fxc", "unpol"),  # 2.08x  (14.85 -> 7.14 ns/pt)
    ("gga_x_ak13", "fxc", "unpol"),  # 2.06x  (13.09 -> 6.34 ns/pt)
    ("gga_x_bpccac", "fxc", "unpol"),  # 2.10x  (17.30 -> 8.25 ns/pt)
    ("gga_x_c09x", "fxc", "unpol"),  # 1.86x  (9.99 -> 5.36 ns/pt)
    ("gga_x_hjs_b88_v2", "fxc", "unpol"),  # 3.46x  (56.21 -> 16.23 ns/pt)
    ("gga_x_htbs", "fxc", "unpol"),  # 2.22x  (16.61 -> 7.47 ns/pt)
    ("gga_x_ityh", "fxc", "unpol"),  # 2.32x  (30.13 -> 12.99 ns/pt)
    ("gga_x_ityh_optx", "fxc", "unpol"),  # 2.14x  (27.22 -> 12.71 ns/pt)
    ("gga_x_lsrpbe", "fxc", "unpol"),  # 1.76x  (10.10 -> 5.73 ns/pt)
    ("gga_x_lv_rpw86", "fxc", "unpol"),  # 2.01x  (14.24 -> 7.10 ns/pt)
    ("gga_x_ncap", "fxc", "unpol"),  # 2.53x  (20.89 -> 8.26 ns/pt)
    ("gga_x_pw86", "fxc", "unpol"),  # 2.02x  (12.14 -> 6.00 ns/pt)
    ("gga_x_wc", "fxc", "unpol"),  # 1.92x  (11.64 -> 6.06 ns/pt)
    ("gga_xc_th1", "fxc", "unpol"),  # 2.21x  (12.94 -> 5.87 ns/pt)
    ("hyb_lda_xc_bn05", "fxc", "unpol"),  # 2.61x  (23.47 -> 9.01 ns/pt)
    ("lda_c_1d_csc", "fxc", "unpol"),  # 1.05x  (6.56 -> 6.23 ns/pt)
    ("lda_c_pmgb06", "fxc", "unpol"),  # 2.81x  (24.13 -> 8.58 ns/pt)
    ("lda_c_pw_erf", "fxc", "unpol"),  # 2.45x  (22.56 -> 9.22 ns/pt)
    ("lda_x_erf", "fxc", "unpol"),  # 2.26x  (13.52 -> 5.98 ns/pt)
    ("lda_x_yukawa", "fxc", "unpol"),  # 2.52x  (17.75 -> 7.04 ns/pt)
    ("mgga_c_b88", "fxc", "unpol"),  # 2.95x  (24.63 -> 8.36 ns/pt)
    ("mgga_c_bc95", "fxc", "unpol"),  # 2.43x  (22.84 -> 9.39 ns/pt)
    ("mgga_c_m05", "fxc", "unpol"),  # 2.66x  (30.79 -> 11.57 ns/pt)
    ("mgga_c_pkzb", "fxc", "unpol"),  # 3.16x  (39.82 -> 12.61 ns/pt)
    ("mgga_c_rmggac", "fxc", "unpol"),  # 2.77x  (41.47 -> 14.95 ns/pt)
    ("mgga_c_rppscan", "fxc", "unpol"),  # 2.97x  (35.41 -> 11.92 ns/pt)
    ("mgga_c_vsxc", "fxc", "unpol"),  # 2.43x  (31.79 -> 13.09 ns/pt)
    ("mgga_x_br89_explicit", "fxc", "unpol"),  # 2.80x  (46.22 -> 16.49 ns/pt)
    ("mgga_x_gvt4", "fxc", "unpol"),  # 2.15x  (13.15 -> 6.11 ns/pt)
    ("mgga_x_lta", "fxc", "unpol"),  # 2.19x  (10.19 -> 4.65 ns/pt)
    ("mgga_x_mvs", "fxc", "unpol"),  # 2.06x  (17.29 -> 8.41 ns/pt)
    ("mgga_x_mvsb", "fxc", "unpol"),  # 2.75x  (21.06 -> 7.66 ns/pt)
    ("mgga_x_vt84", "fxc", "unpol"),  # 3.23x  (36.97 -> 11.43 ns/pt)
    # Added by tools/translate_rayon/simd_qualify.py; each line's
    # ratio is sweep ns/pt before -> after, fingerprint unchanged.
    ("gga_c_acgga", "kxc", "unpol"),  # 2.85x  (50.25 -> 17.60 ns/pt)
    ("gga_c_acgga", "lxc", "pol"),  # 2.59x  (1070.06 -> 413.24 ns/pt)
    ("gga_c_acggap", "kxc", "pol"),  # 3.20x  (480.94 -> 150.10 ns/pt)
    ("gga_c_acggap", "kxc", "unpol"),  # 3.53x  (89.64 -> 25.38 ns/pt)
    ("gga_c_acggap", "lxc", "pol"),  # 3.39x  (2111.79 -> 622.50 ns/pt)
    ("gga_c_am05", "kxc", "pol"),  # 1.62x  (79.64 -> 49.19 ns/pt)
    ("gga_c_am05", "kxc", "unpol"),  # 2.30x  (18.10 -> 7.88 ns/pt)
    ("gga_c_am05", "lxc", "pol"),  # 1.91x  (140.42 -> 73.53 ns/pt)
    ("gga_c_bmk", "kxc", "pol"),  # 2.88x  (241.67 -> 83.94 ns/pt)
    ("gga_c_bmk", "kxc", "unpol"),  # 2.55x  (41.06 -> 16.13 ns/pt)
    ("gga_c_bmk", "lxc", "pol"),  # 3.13x  (519.62 -> 166.00 ns/pt)
    ("gga_c_bmk", "lxc", "unpol"),  # 3.22x  (120.49 -> 37.38 ns/pt)
    ("gga_c_ccdf", "fxc", "unpol"),  # 1.66x  (7.02 -> 4.24 ns/pt)
    ("gga_c_chachiyo", "kxc", "pol"),  # 1.71x  (79.96 -> 46.69 ns/pt)
    ("gga_c_chachiyo", "kxc", "unpol"),  # 2.10x  (14.85 -> 7.08 ns/pt)
    ("gga_c_chachiyo", "lxc", "pol"),  # 2.13x  (158.19 -> 74.24 ns/pt)
    ("gga_c_chachiyo", "lxc", "unpol"),  # 2.35x  (23.04 -> 9.82 ns/pt)
    ("gga_c_gapc", "kxc", "pol"),  # 2.92x  (411.85 -> 140.83 ns/pt)
    ("gga_c_gapc", "kxc", "unpol"),  # 2.89x  (280.40 -> 96.90 ns/pt)
    ("gga_c_gapc", "lxc", "pol"),  # 3.52x  (1877.54 -> 532.93 ns/pt)
    ("gga_c_gaploc", "kxc", "pol"),  # 2.84x  (386.14 -> 135.86 ns/pt)
    ("gga_c_gaploc", "kxc", "unpol"),  # 2.97x  (197.10 -> 66.28 ns/pt)
    ("gga_c_gaploc", "lxc", "pol"),  # 3.45x  (2066.28 -> 598.90 ns/pt)
    ("gga_c_gaploc", "lxc", "unpol"),  # 2.98x  (795.18 -> 267.33 ns/pt)
    ("gga_c_hcth_a", "kxc", "pol"),  # 2.75x  (274.41 -> 99.66 ns/pt)
    ("gga_c_hcth_a", "kxc", "unpol"),  # 2.52x  (52.62 -> 20.87 ns/pt)
    ("gga_c_hcth_a", "lxc", "pol"),  # 2.85x  (695.58 -> 244.51 ns/pt)
    ("gga_c_hcth_a", "lxc", "unpol"),  # 3.02x  (99.28 -> 32.91 ns/pt)
    ("gga_c_lm", "kxc", "pol"),  # 1.84x  (98.23 -> 53.35 ns/pt)
    ("gga_c_lm", "kxc", "unpol"),  # 2.04x  (17.99 -> 8.80 ns/pt)
    ("gga_c_lm", "lxc", "pol"),  # 2.30x  (190.93 -> 82.94 ns/pt)
    ("gga_c_lm", "lxc", "unpol"),  # 2.29x  (23.98 -> 10.47 ns/pt)
    ("gga_c_lyp", "kxc", "pol"),  # 1.92x  (101.83 -> 52.95 ns/pt)
    ("gga_c_lyp", "kxc", "unpol"),  # 2.50x  (14.05 -> 5.62 ns/pt)
    ("gga_c_lyp", "lxc", "pol"),  # 2.28x  (197.17 -> 86.58 ns/pt)
    ("gga_c_lyp", "lxc", "unpol"),  # 2.77x  (19.89 -> 7.18 ns/pt)
    ("gga_c_lypr", "kxc", "pol"),  # 2.24x  (164.26 -> 73.43 ns/pt)
    ("gga_c_lypr", "lxc", "pol"),  # 2.76x  (378.84 -> 137.35 ns/pt)
    ("gga_c_op_b88", "fxc", "unpol"),  # 2.54x  (18.27 -> 7.19 ns/pt)
    ("gga_c_op_b88", "kxc", "pol"),  # 2.30x  (131.33 -> 57.06 ns/pt)
    ("gga_c_op_b88", "kxc", "unpol"),  # 2.85x  (31.12 -> 10.92 ns/pt)
    ("gga_c_op_b88", "lxc", "pol"),  # 2.85x  (383.57 -> 134.69 ns/pt)
    ("gga_c_op_b88", "lxc", "unpol"),  # 3.41x  (64.24 -> 18.85 ns/pt)
    ("gga_c_op_g96", "fxc", "unpol"),  # 2.31x  (14.68 -> 6.36 ns/pt)
    ("gga_c_op_g96", "kxc", "pol"),  # 2.14x  (120.69 -> 56.39 ns/pt)
    ("gga_c_op_g96", "kxc", "unpol"),  # 2.62x  (25.38 -> 9.68 ns/pt)
    ("gga_c_op_g96", "lxc", "pol"),  # 2.82x  (375.70 -> 133.28 ns/pt)
    ("gga_c_op_g96", "lxc", "unpol"),  # 3.29x  (52.72 -> 16.02 ns/pt)
    ("gga_c_op_pbe", "kxc", "unpol"),  # 2.62x  (24.32 -> 9.30 ns/pt)
    ("gga_c_op_pbe", "lxc", "unpol"),  # 2.94x  (52.46 -> 17.87 ns/pt)
    ("gga_c_op_pw91", "kxc", "pol"),  # 2.95x  (200.60 -> 67.96 ns/pt)
    ("gga_c_op_pw91", "kxc", "unpol"),  # 2.65x  (35.19 -> 13.26 ns/pt)
    ("gga_c_op_pw91", "lxc", "pol"),  # 3.48x  (588.05 -> 168.74 ns/pt)
    ("gga_c_op_pw91", "lxc", "unpol"),  # 2.96x  (71.39 -> 24.10 ns/pt)
    ("gga_c_op_xalpha", "fxc", "unpol"),  # 1.84x  (7.80 -> 4.23 ns/pt)
    ("gga_c_optc", "kxc", "pol"),  # 3.04x  (694.07 -> 228.55 ns/pt)
    ("gga_c_optc", "kxc", "unpol"),  # 2.60x  (71.55 -> 27.47 ns/pt)
    ("gga_c_optc", "lxc", "pol"),  # 4.99x  (4117.50 -> 824.66 ns/pt)
    ("gga_c_optc", "lxc", "unpol"),  # 3.29x  (254.94 -> 77.45 ns/pt)
    ("gga_c_p86", "kxc", "unpol"),  # 2.31x  (19.24 -> 8.31 ns/pt)
    ("gga_c_p86", "lxc", "unpol"),  # 2.62x  (28.42 -> 10.83 ns/pt)
    ("gga_c_p86vwn", "kxc", "pol"),  # 2.00x  (120.96 -> 60.52 ns/pt)
    ("gga_c_p86vwn", "kxc", "unpol"),  # 2.34x  (34.59 -> 14.80 ns/pt)
    ("gga_c_p86vwn", "lxc", "pol"),  # 2.53x  (266.39 -> 105.14 ns/pt)
    ("gga_c_p86vwn", "lxc", "unpol"),  # 2.54x  (59.47 -> 23.43 ns/pt)
    ("gga_c_pbe", "kxc", "pol"),  # 2.33x  (151.23 -> 64.79 ns/pt)
    ("gga_c_pbe", "kxc", "unpol"),  # 2.74x  (29.63 -> 10.82 ns/pt)
    ("gga_c_pbe", "lxc", "pol"),  # 2.80x  (420.21 -> 150.10 ns/pt)
    ("gga_c_pbe", "lxc", "unpol"),  # 2.68x  (55.35 -> 20.63 ns/pt)
    ("gga_c_pbe_erf_gws", "kxc", "pol"),  # 2.80x  (735.06 -> 262.67 ns/pt)
    ("gga_c_pbe_erf_gws", "kxc", "unpol"),  # 3.72x  (109.68 -> 29.50 ns/pt)
    ("gga_c_pbe_erf_gws", "lxc", "pol"),  # 4.12x  (3160.70 -> 766.24 ns/pt)
    ("gga_c_pbe_erf_gws", "lxc", "unpol"),  # 3.28x  (187.44 -> 57.18 ns/pt)
    ("gga_c_pbe_vwn", "kxc", "pol"),  # 2.57x  (184.84 -> 71.82 ns/pt)
    ("gga_c_pbe_vwn", "kxc", "unpol"),  # 2.68x  (41.84 -> 15.63 ns/pt)
    ("gga_c_pbe_vwn", "lxc", "pol"),  # 2.80x  (453.12 -> 161.90 ns/pt)
    ("gga_c_pbe_vwn", "lxc", "unpol"),  # 3.04x  (84.03 -> 27.64 ns/pt)
    ("gga_c_pbeloc", "kxc", "pol"),  # 3.29x  (331.82 -> 100.99 ns/pt)
    ("gga_c_pbeloc", "kxc", "unpol"),  # 2.93x  (42.07 -> 14.37 ns/pt)
    ("gga_c_pbeloc", "lxc", "pol"),  # 2.79x  (687.69 -> 246.04 ns/pt)
    ("gga_c_pbeloc", "lxc", "unpol"),  # 3.30x  (146.56 -> 44.44 ns/pt)
    ("gga_c_pw91", "kxc", "pol"),  # 2.46x  (203.42 -> 82.52 ns/pt)
    ("gga_c_pw91", "kxc", "unpol"),  # 2.52x  (40.63 -> 16.14 ns/pt)
    ("gga_c_pw91", "lxc", "pol"),  # 2.76x  (605.67 -> 219.20 ns/pt)
    ("gga_c_pw91", "lxc", "unpol"),  # 2.75x  (85.15 -> 30.97 ns/pt)
    ("gga_c_q2d", "kxc", "pol"),  # 2.93x  (460.47 -> 157.40 ns/pt)
    ("gga_c_q2d", "kxc", "unpol"),  # 3.07x  (76.56 -> 24.95 ns/pt)
    ("gga_c_q2d", "lxc", "pol"),  # 3.99x  (2563.48 -> 643.13 ns/pt)
    ("gga_c_q2d", "lxc", "unpol"),  # 3.10x  (176.73 -> 57.05 ns/pt)
    ("gga_c_regtpss", "kxc", "pol"),  # 2.92x  (262.68 -> 90.06 ns/pt)
    ("gga_c_regtpss", "kxc", "unpol"),  # 2.72x  (46.71 -> 17.16 ns/pt)
    ("gga_c_regtpss", "lxc", "pol"),  # 2.95x  (692.34 -> 234.35 ns/pt)
    ("gga_c_revtca", "kxc", "pol"),  # 2.14x  (136.74 -> 64.00 ns/pt)
    ("gga_c_revtca", "kxc", "unpol"),  # 1.71x  (18.55 -> 10.86 ns/pt)
    ("gga_c_revtca", "lxc", "pol"),  # 3.06x  (477.25 -> 155.77 ns/pt)
    ("gga_c_revtca", "lxc", "unpol"),  # 1.95x  (31.03 -> 15.96 ns/pt)
    ("gga_c_scan_e0", "kxc", "pol"),  # 3.26x  (219.91 -> 67.47 ns/pt)
    ("gga_c_scan_e0", "kxc", "unpol"),  # 2.64x  (31.49 -> 11.91 ns/pt)
    ("gga_c_scan_e0", "lxc", "pol"),  # 2.75x  (381.05 -> 138.41 ns/pt)
    ("gga_c_sg4", "kxc", "pol"),  # 2.85x  (291.51 -> 102.37 ns/pt)
    ("gga_c_sg4", "lxc", "pol"),  # 2.74x  (1226.81 -> 448.31 ns/pt)
    ("gga_c_sg4", "lxc", "unpol"),  # 3.08x  (146.12 -> 47.37 ns/pt)
    ("gga_c_sogga11", "fxc", "unpol"),  # 2.57x  (19.50 -> 7.58 ns/pt)
    ("gga_c_sogga11", "kxc", "pol"),  # 3.19x  (301.89 -> 94.48 ns/pt)
    ("gga_c_sogga11", "kxc", "unpol"),  # 2.00x  (37.44 -> 18.69 ns/pt)
    ("gga_c_sogga11", "lxc", "pol"),  # 2.87x  (682.07 -> 238.10 ns/pt)
    ("gga_c_sogga11", "lxc", "unpol"),  # 3.23x  (96.33 -> 29.84 ns/pt)
    ("gga_c_tca", "kxc", "pol"),  # 1.40x  (68.35 -> 48.89 ns/pt)
    ("gga_c_tca", "kxc", "unpol"),  # 1.67x  (18.41 -> 11.01 ns/pt)
    ("gga_c_tca", "lxc", "pol"),  # 2.02x  (164.37 -> 81.29 ns/pt)
    ("gga_c_tca", "lxc", "unpol"),  # 1.91x  (31.16 -> 16.28 ns/pt)
    ("gga_c_w94", "fxc", "unpol"),  # 1.81x  (7.51 -> 4.14 ns/pt)
    ("gga_c_w94", "kxc", "pol"),  # 1.25x  (53.09 -> 42.38 ns/pt)
    ("gga_c_w94", "lxc", "pol"),  # 1.71x  (113.42 -> 66.32 ns/pt)
    ("gga_c_wi", "fxc", "unpol"),  # 1.95x  (8.29 -> 4.24 ns/pt)
    ("gga_c_wl", "fxc", "unpol"),  # 1.59x  (5.29 -> 3.33 ns/pt)
    ("gga_c_wl", "kxc", "unpol"),  # 1.84x  (10.52 -> 5.71 ns/pt)
    ("gga_c_wl", "lxc", "unpol"),  # 2.28x  (16.98 -> 7.46 ns/pt)
    ("gga_c_zpbeint", "kxc", "pol"),  # 2.56x  (191.88 -> 74.99 ns/pt)
    ("gga_c_zpbeint", "kxc", "unpol"),  # 2.71x  (64.86 -> 23.89 ns/pt)
    ("gga_c_zpbeint", "lxc", "pol"),  # 3.04x  (643.39 -> 211.33 ns/pt)
    ("gga_c_zpbeint", "lxc", "unpol"),  # 3.05x  (90.20 -> 29.61 ns/pt)
    ("gga_c_zvpbeint", "kxc", "pol"),  # 2.83x  (229.18 -> 80.89 ns/pt)
    ("gga_c_zvpbeint", "kxc", "unpol"),  # 2.76x  (65.36 -> 23.69 ns/pt)
    ("gga_c_zvpbeint", "lxc", "pol"),  # 3.05x  (749.13 -> 245.51 ns/pt)
    ("gga_c_zvpbeint", "lxc", "unpol"),  # 2.84x  (90.32 -> 31.75 ns/pt)
    ("gga_c_zvpbeloc", "kxc", "pol"),  # 2.97x  (227.64 -> 76.76 ns/pt)
    ("gga_c_zvpbeloc", "kxc", "unpol"),  # 2.89x  (46.89 -> 16.24 ns/pt)
    ("gga_c_zvpbeloc", "lxc", "pol"),  # 2.85x  (994.73 -> 348.92 ns/pt)
    ("gga_c_zvpbeloc", "lxc", "unpol"),  # 3.23x  (97.69 -> 30.29 ns/pt)
    ("gga_k_apbe", "fxc", "unpol"),  # 1.74x  (8.32 -> 4.77 ns/pt)
    ("gga_k_apbe", "kxc", "pol"),  # 1.50x  (70.80 -> 47.35 ns/pt)
    ("gga_k_apbe", "lxc", "pol"),  # 1.77x  (121.90 -> 68.98 ns/pt)
    ("gga_k_apbeint", "fxc", "unpol"),  # 2.07x  (11.97 -> 5.79 ns/pt)
    ("gga_k_apbeint", "kxc", "pol"),  # 1.70x  (84.37 -> 49.58 ns/pt)
    ("gga_k_dk", "fxc", "unpol"),  # 2.23x  (10.70 -> 4.79 ns/pt)
    ("gga_k_exp4", "kxc", "pol"),  # 1.38x  (63.31 -> 45.96 ns/pt)
    ("gga_k_exp4", "kxc", "unpol"),  # 1.81x  (12.45 -> 6.86 ns/pt)
    ("gga_k_exp4", "lxc", "pol"),  # 1.52x  (101.03 -> 66.49 ns/pt)
    ("gga_k_exp4", "lxc", "unpol"),  # 1.98x  (17.22 -> 8.70 ns/pt)
    ("gga_k_lc94", "kxc", "pol"),  # 1.64x  (79.81 -> 48.57 ns/pt)
    ("gga_k_lc94", "lxc", "pol"),  # 1.93x  (130.33 -> 67.69 ns/pt)
    ("gga_k_lgap", "fxc", "unpol"),  # 1.86x  (9.63 -> 5.17 ns/pt)
    ("gga_k_lgap", "kxc", "pol"),  # 1.41x  (63.27 -> 44.74 ns/pt)
    ("gga_k_lgap", "kxc", "unpol"),  # 1.94x  (11.11 -> 5.73 ns/pt)
    ("gga_k_lgap", "lxc", "pol"),  # 1.63x  (106.94 -> 65.44 ns/pt)
    ("gga_k_lgap", "lxc", "unpol"),  # 2.22x  (15.51 -> 6.98 ns/pt)
    ("gga_k_lgap_ge", "fxc", "unpol"),  # 1.68x  (8.39 -> 4.98 ns/pt)
    ("gga_k_lkt", "kxc", "pol"),  # 1.43x  (63.73 -> 44.62 ns/pt)
    ("gga_k_lkt", "kxc", "unpol"),  # 2.52x  (20.89 -> 8.28 ns/pt)
    ("gga_k_lkt", "lxc", "pol"),  # 1.59x  (98.70 -> 62.02 ns/pt)
    ("gga_k_lkt", "lxc", "unpol"),  # 2.62x  (29.00 -> 11.06 ns/pt)
    ("gga_k_llp", "fxc", "unpol"),  # 1.96x  (11.00 -> 5.61 ns/pt)
    ("gga_k_llp", "kxc", "pol"),  # 1.44x  (66.28 -> 46.12 ns/pt)
    ("gga_k_llp", "kxc", "unpol"),  # 2.19x  (14.56 -> 6.64 ns/pt)
    ("gga_k_llp", "lxc", "pol"),  # 1.72x  (113.20 -> 65.88 ns/pt)
    ("gga_k_llp", "lxc", "unpol"),  # 1.80x  (22.09 -> 12.30 ns/pt)
    ("gga_k_meyer", "fxc", "unpol"),  # 2.19x  (15.46 -> 7.08 ns/pt)
    ("gga_k_meyer", "kxc", "pol"),  # 1.85x  (96.52 -> 52.16 ns/pt)
    ("gga_k_meyer", "kxc", "unpol"),  # 1.86x  (28.35 -> 15.21 ns/pt)
    ("gga_k_meyer", "lxc", "pol"),  # 2.19x  (185.90 -> 84.77 ns/pt)
    ("gga_k_meyer", "lxc", "unpol"),  # 2.38x  (67.29 -> 28.29 ns/pt)
    ("gga_k_ol1", "fxc", "unpol"),  # 1.53x  (7.80 -> 5.11 ns/pt)
    ("gga_k_ol2", "fxc", "unpol"),  # 1.99x  (8.56 -> 4.31 ns/pt)
    ("gga_k_pearson", "fxc", "unpol"),  # 1.67x  (7.88 -> 4.71 ns/pt)
    ("gga_k_rational_p", "kxc", "pol"),  # 1.49x  (71.15 -> 47.70 ns/pt)
    ("gga_k_rational_p", "kxc", "unpol"),  # 2.08x  (13.46 -> 6.49 ns/pt)
    ("gga_k_rational_p", "lxc", "pol"),  # 1.88x  (134.24 -> 71.57 ns/pt)
    ("gga_k_rational_p", "lxc", "unpol"),  # 2.22x  (17.99 -> 8.12 ns/pt)
    ("gga_k_thakkar", "fxc", "unpol"),  # 2.17x  (11.69 -> 5.39 ns/pt)
    ("gga_k_vt84f", "kxc", "pol"),  # 1.67x  (85.06 -> 51.00 ns/pt)
    ("gga_k_vt84f", "kxc", "unpol"),  # 2.30x  (24.27 -> 10.56 ns/pt)
    ("gga_k_vt84f", "lxc", "pol"),  # 2.25x  (170.51 -> 75.80 ns/pt)
    ("gga_k_vt84f", "lxc", "unpol"),  # 3.12x  (46.56 -> 14.95 ns/pt)
    ("gga_x_2d_b88", "kxc", "pol"),  # 1.39x  (62.00 -> 44.70 ns/pt)
    ("gga_x_2d_b88", "lxc", "pol"),  # 1.65x  (107.68 -> 65.25 ns/pt)
    ("gga_x_airy", "kxc", "pol"),  # 1.65x  (126.37 -> 76.56 ns/pt)
    ("gga_x_airy", "kxc", "unpol"),  # 1.82x  (42.51 -> 23.31 ns/pt)
    ("gga_x_airy", "lxc", "pol"),  # 1.82x  (208.57 -> 114.52 ns/pt)
    ("gga_x_airy", "lxc", "unpol"),  # 2.12x  (78.04 -> 36.80 ns/pt)
    ("gga_x_ak13", "kxc", "pol"),  # 1.53x  (71.59 -> 46.87 ns/pt)
    ("gga_x_ak13", "kxc", "unpol"),  # 2.54x  (17.87 -> 7.04 ns/pt)
    ("gga_x_ak13", "lxc", "pol"),  # 1.78x  (118.60 -> 66.59 ns/pt)
    ("gga_x_ak13", "lxc", "unpol"),  # 2.44x  (24.85 -> 10.19 ns/pt)
    ("gga_x_b86", "fxc", "unpol"),  # 2.01x  (10.60 -> 5.27 ns/pt)
    ("gga_x_b86", "kxc", "pol"),  # 1.44x  (63.78 -> 44.16 ns/pt)
    ("gga_x_b86", "kxc", "unpol"),  # 2.04x  (12.77 -> 6.26 ns/pt)
    ("gga_x_b86", "lxc", "pol"),  # 1.59x  (101.41 -> 63.62 ns/pt)
    ("gga_x_b86", "lxc", "unpol"),  # 2.43x  (17.93 -> 7.38 ns/pt)
    ("gga_x_b88", "kxc", "pol"),  # 1.43x  (64.87 -> 45.40 ns/pt)
    ("gga_x_b88", "kxc", "unpol"),  # 2.04x  (14.02 -> 6.87 ns/pt)
    ("gga_x_b88", "lxc", "pol"),  # 1.69x  (109.84 -> 65.12 ns/pt)
    ("gga_x_b88", "lxc", "unpol"),  # 2.39x  (21.45 -> 8.98 ns/pt)
    ("gga_x_bayesian", "fxc", "unpol"),  # 2.05x  (11.38 -> 5.54 ns/pt)
    ("gga_x_beefvdw", "fxc", "unpol"),  # 3.00x  (22.77 -> 7.60 ns/pt)
    ("gga_x_beefvdw", "lxc", "unpol"),  # 4.18x  (147.69 -> 35.34 ns/pt)
    ("gga_x_bpccac", "kxc", "pol"),  # 1.83x  (96.96 -> 52.90 ns/pt)
    ("gga_x_bpccac", "kxc", "unpol"),  # 2.49x  (28.83 -> 11.56 ns/pt)
    ("gga_x_bpccac", "lxc", "pol"),  # 2.14x  (173.39 -> 80.93 ns/pt)
    ("gga_x_bpccac", "lxc", "unpol"),  # 2.92x  (58.07 -> 19.88 ns/pt)
    ("gga_x_c09x", "kxc", "pol"),  # 1.36x  (60.33 -> 44.46 ns/pt)
    ("gga_x_c09x", "kxc", "unpol"),  # 1.96x  (11.28 -> 5.76 ns/pt)
    ("gga_x_c09x", "lxc", "pol"),  # 1.47x  (92.17 -> 62.81 ns/pt)
    ("gga_x_c09x", "lxc", "unpol"),  # 2.34x  (15.60 -> 6.66 ns/pt)
    ("gga_x_cap", "fxc", "unpol"),  # 2.13x  (12.32 -> 5.79 ns/pt)
    ("gga_x_cap", "kxc", "pol"),  # 1.48x  (68.95 -> 46.62 ns/pt)
    ("gga_x_cap", "kxc", "unpol"),  # 1.80x  (18.00 -> 10.01 ns/pt)
    ("gga_x_cap", "lxc", "pol"),  # 1.80x  (121.91 -> 67.89 ns/pt)
    ("gga_x_cap", "lxc", "unpol"),  # 2.11x  (29.53 -> 13.99 ns/pt)
    ("gga_x_chachiyo", "fxc", "unpol"),  # 2.32x  (12.64 -> 5.44 ns/pt)
    ("gga_x_ev93", "fxc", "unpol"),  # 1.89x  (10.67 -> 5.66 ns/pt)
    ("gga_x_ev93", "kxc", "unpol"),  # 2.30x  (19.99 -> 8.69 ns/pt)
    ("gga_x_ev93", "lxc", "unpol"),  # 2.43x  (28.15 -> 11.59 ns/pt)
    ("gga_x_g96", "fxc", "unpol"),  # 1.59x  (7.76 -> 4.87 ns/pt)
    ("gga_x_g96", "kxc", "unpol"),  # 1.75x  (13.38 -> 7.65 ns/pt)
    ("gga_x_g96", "lxc", "unpol"),  # 2.02x  (16.09 -> 7.96 ns/pt)
    ("gga_x_hcth_a", "fxc", "unpol"),  # 1.93x  (11.44 -> 5.93 ns/pt)
    ("gga_x_hcth_a", "kxc", "pol"),  # 1.83x  (88.14 -> 48.10 ns/pt)
    ("gga_x_hcth_a", "kxc", "unpol"),  # 1.51x  (15.28 -> 10.11 ns/pt)
    ("gga_x_hcth_a", "lxc", "pol"),  # 2.11x  (147.64 -> 70.11 ns/pt)
    ("gga_x_hcth_a", "lxc", "unpol"),  # 1.83x  (22.34 -> 12.22 ns/pt)
    ("gga_x_hjs_b88_v2", "kxc", "pol"),  # 3.35x  (514.55 -> 153.46 ns/pt)
    ("gga_x_hjs_b88_v2", "kxc", "unpol"),  # 3.67x  (157.58 -> 42.91 ns/pt)
    ("gga_x_hjs_b88_v2", "lxc", "pol"),  # 4.04x  (3151.21 -> 780.07 ns/pt)
    ("gga_x_hjs_b88_v2", "lxc", "unpol"),  # 3.37x  (404.43 -> 120.03 ns/pt)
    ("gga_x_htbs", "kxc", "pol"),  # 1.77x  (88.03 -> 49.84 ns/pt)
    ("gga_x_htbs", "kxc", "unpol"),  # 2.48x  (24.95 -> 10.05 ns/pt)
    ("gga_x_htbs", "lxc", "pol"),  # 2.10x  (153.36 -> 73.10 ns/pt)
    ("gga_x_htbs", "lxc", "unpol"),  # 2.87x  (43.04 -> 15.01 ns/pt)
    ("gga_x_ityh", "kxc", "pol"),  # 2.33x  (202.89 -> 87.00 ns/pt)
    ("gga_x_ityh", "lxc", "pol"),  # 2.97x  (510.35 -> 171.61 ns/pt)
    ("gga_x_ityh_optx", "kxc", "pol"),  # 2.58x  (273.71 -> 106.29 ns/pt)
    ("gga_x_ityh_optx", "kxc", "unpol"),  # 2.37x  (66.23 -> 27.91 ns/pt)
    ("gga_x_ityh_optx", "lxc", "pol"),  # 3.01x  (687.48 -> 228.64 ns/pt)
    ("gga_x_ityh_optx", "lxc", "unpol"),  # 2.82x  (139.35 -> 49.39 ns/pt)
    ("gga_x_ityh_pbe", "kxc", "pol"),  # 2.52x  (287.54 -> 114.15 ns/pt)
    ("gga_x_ityh_pbe", "kxc", "unpol"),  # 2.69x  (69.33 -> 25.78 ns/pt)
    ("gga_x_ityh_pbe", "lxc", "pol"),  # 2.94x  (551.67 -> 187.60 ns/pt)
    ("gga_x_ityh_pbe", "lxc", "unpol"),  # 3.21x  (154.37 -> 48.01 ns/pt)
    ("gga_x_lag", "kxc", "pol"),  # 1.67x  (100.51 -> 60.27 ns/pt)
    ("gga_x_lag", "kxc", "unpol"),  # 1.93x  (25.60 -> 13.24 ns/pt)
    ("gga_x_lag", "lxc", "pol"),  # 1.92x  (192.03 -> 100.00 ns/pt)
    ("gga_x_lag", "lxc", "unpol"),  # 2.00x  (37.98 -> 18.94 ns/pt)
    ("gga_x_lg93", "kxc", "pol"),  # 1.69x  (91.27 -> 53.95 ns/pt)
    ("gga_x_lg93", "kxc", "unpol"),  # 2.32x  (22.13 -> 9.54 ns/pt)
    ("gga_x_lg93", "lxc", "pol"),  # 2.23x  (193.96 -> 87.09 ns/pt)
    ("gga_x_lg93", "lxc", "unpol"),  # 2.66x  (36.12 -> 13.59 ns/pt)
    ("gga_x_lspbe", "fxc", "unpol"),  # 1.90x  (9.84 -> 5.19 ns/pt)
    ("gga_x_lspbe", "kxc", "pol"),  # 1.39x  (61.08 -> 44.07 ns/pt)
    ("gga_x_lspbe", "kxc", "unpol"),  # 2.07x  (12.23 -> 5.92 ns/pt)
    ("gga_x_lspbe", "lxc", "pol"),  # 1.46x  (92.20 -> 63.37 ns/pt)
    ("gga_x_lspbe", "lxc", "unpol"),  # 2.42x  (16.39 -> 6.77 ns/pt)
    ("gga_x_lsrpbe", "kxc", "pol"),  # 1.40x  (62.31 -> 44.68 ns/pt)
    ("gga_x_lsrpbe", "kxc", "unpol"),  # 1.98x  (11.91 -> 6.00 ns/pt)
    ("gga_x_lsrpbe", "lxc", "pol"),  # 1.49x  (95.32 -> 64.13 ns/pt)
    ("gga_x_lsrpbe", "lxc", "unpol"),  # 2.48x  (15.73 -> 6.34 ns/pt)
    ("gga_x_lv_rpw86", "kxc", "pol"),  # 1.66x  (81.38 -> 49.10 ns/pt)
    ("gga_x_lv_rpw86", "kxc", "unpol"),  # 2.31x  (21.40 -> 9.28 ns/pt)
    ("gga_x_lv_rpw86", "lxc", "pol"),  # 2.21x  (161.40 -> 73.06 ns/pt)
    ("gga_x_lv_rpw86", "lxc", "unpol"),  # 2.47x  (40.76 -> 16.50 ns/pt)
    ("gga_x_mpbe", "fxc", "unpol"),  # 2.11x  (13.08 -> 6.19 ns/pt)
    ("gga_x_mpbe", "kxc", "unpol"),  # 2.29x  (23.10 -> 10.07 ns/pt)
    ("gga_x_mpbe", "lxc", "unpol"),  # 2.52x  (29.79 -> 11.84 ns/pt)
    ("gga_x_n12", "fxc", "unpol"),  # 2.60x  (17.29 -> 6.64 ns/pt)
    ("gga_x_n12", "kxc", "pol"),  # 2.04x  (114.99 -> 56.36 ns/pt)
    ("gga_x_n12", "lxc", "pol"),  # 2.29x  (197.83 -> 86.48 ns/pt)
    ("gga_x_ncap", "kxc", "pol"),  # 2.32x  (156.09 -> 67.28 ns/pt)
    ("gga_x_ncap", "kxc", "unpol"),  # 3.03x  (61.41 -> 20.24 ns/pt)
    ("gga_x_ncap", "lxc", "pol"),  # 2.85x  (337.10 -> 118.07 ns/pt)
    ("gga_x_ncap", "lxc", "unpol"),  # 3.33x  (156.36 -> 46.94 ns/pt)
    ("gga_x_ol2", "fxc", "unpol"),  # 1.91x  (8.32 -> 4.35 ns/pt)
    ("gga_x_optx", "fxc", "unpol"),  # 1.90x  (8.33 -> 4.38 ns/pt)
    ("gga_x_pbe", "kxc", "pol"),  # 1.33x  (58.94 -> 44.29 ns/pt)
    ("gga_x_pbe", "kxc", "unpol"),  # 1.91x  (9.69 -> 5.08 ns/pt)
    ("gga_x_pbe", "lxc", "pol"),  # 1.54x  (98.82 -> 64.27 ns/pt)
    ("gga_x_pbe", "lxc", "unpol"),  # 2.01x  (12.64 -> 6.30 ns/pt)
    ("gga_x_pbe_erf_gws", "kxc", "pol"),  # 2.25x  (141.16 -> 62.85 ns/pt)
    ("gga_x_pbe_erf_gws", "lxc", "pol"),  # 2.75x  (252.71 -> 91.87 ns/pt)
    ("gga_x_pbea", "kxc", "pol"),  # 1.48x  (69.23 -> 46.66 ns/pt)
    ("gga_x_pbea", "kxc", "unpol"),  # 1.86x  (12.54 -> 6.75 ns/pt)
    ("gga_x_pbea", "lxc", "pol"),  # 1.67x  (110.91 -> 66.45 ns/pt)
    ("gga_x_pbea", "lxc", "unpol"),  # 2.20x  (16.27 -> 7.39 ns/pt)
    ("gga_x_pbeint", "fxc", "unpol"),  # 2.06x  (11.59 -> 5.63 ns/pt)
    ("gga_x_pbeint", "kxc", "unpol"),  # 2.29x  (21.33 -> 9.33 ns/pt)
    ("gga_x_pbeint", "lxc", "unpol"),  # 2.62x  (31.99 -> 12.19 ns/pt)
    ("gga_x_pbetrans", "fxc", "unpol"),  # 2.27x  (12.40 -> 5.48 ns/pt)
    ("gga_x_pbetrans", "kxc", "pol"),  # 1.59x  (76.53 -> 48.25 ns/pt)
    ("gga_x_pbetrans", "kxc", "unpol"),  # 2.69x  (20.78 -> 7.72 ns/pt)
    ("gga_x_pbetrans", "lxc", "pol"),  # 2.22x  (170.11 -> 76.69 ns/pt)
    ("gga_x_pbetrans", "lxc", "unpol"),  # 3.28x  (46.21 -> 14.07 ns/pt)
    ("gga_x_pw86", "kxc", "pol"),  # 1.49x  (68.93 -> 46.13 ns/pt)
    ("gga_x_pw86", "kxc", "unpol"),  # 2.12x  (13.72 -> 6.48 ns/pt)
    ("gga_x_pw86", "lxc", "pol"),  # 1.75x  (116.60 -> 66.78 ns/pt)
    ("gga_x_pw86", "lxc", "unpol"),  # 2.38x  (18.23 -> 7.66 ns/pt)
    ("gga_x_pw91", "kxc", "pol"),  # 1.65x  (82.28 -> 50.00 ns/pt)
    ("gga_x_pw91", "kxc", "unpol"),  # 2.34x  (23.19 -> 9.93 ns/pt)
    ("gga_x_pw91", "lxc", "pol"),  # 1.90x  (133.54 -> 70.47 ns/pt)
    ("gga_x_pw91", "lxc", "unpol"),  # 3.57x  (49.30 -> 13.80 ns/pt)
    ("gga_x_q1d", "fxc", "unpol"),  # 2.00x  (11.59 -> 5.78 ns/pt)
    ("gga_x_q1d", "kxc", "unpol"),  # 2.50x  (30.75 -> 12.30 ns/pt)
    ("gga_x_q1d", "lxc", "unpol"),  # 2.97x  (52.70 -> 17.77 ns/pt)
    ("gga_x_q2d", "kxc", "pol"),  # 1.84x  (92.86 -> 50.44 ns/pt)
    ("gga_x_q2d", "lxc", "pol"),  # 2.27x  (180.04 -> 79.34 ns/pt)
    ("gga_x_rge2", "fxc", "unpol"),  # 1.65x  (7.80 -> 4.74 ns/pt)
    ("gga_x_rge2", "kxc", "unpol"),  # 1.64x  (14.54 -> 8.86 ns/pt)
    ("gga_x_rge2", "lxc", "unpol"),  # 1.97x  (19.56 -> 9.94 ns/pt)
    ("gga_x_rpbe", "fxc", "unpol"),  # 1.74x  (8.51 -> 4.88 ns/pt)
    ("gga_x_rpbe", "kxc", "pol"),  # 1.37x  (60.65 -> 44.28 ns/pt)
    ("gga_x_rpbe", "kxc", "unpol"),  # 1.95x  (10.02 -> 5.13 ns/pt)
    ("gga_x_rpbe", "lxc", "pol"),  # 1.63x  (105.50 -> 64.87 ns/pt)
    ("gga_x_rpbe", "lxc", "unpol"),  # 2.18x  (12.51 -> 5.74 ns/pt)
    ("gga_x_sfat", "kxc", "pol"),  # 3.03x  (345.95 -> 114.17 ns/pt)
    ("gga_x_sfat", "lxc", "pol"),  # 3.13x  (881.82 -> 281.70 ns/pt)
    ("gga_x_sfat_pbe", "kxc", "pol"),  # 3.03x  (334.00 -> 110.36 ns/pt)
    ("gga_x_sfat_pbe", "kxc", "unpol"),  # 2.59x  (51.00 -> 19.71 ns/pt)
    ("gga_x_sfat_pbe", "lxc", "pol"),  # 3.23x  (912.52 -> 282.62 ns/pt)
    ("gga_x_sfat_pbe", "lxc", "unpol"),  # 3.17x  (134.21 -> 42.37 ns/pt)
    ("gga_x_sg4", "fxc", "unpol"),  # 1.93x  (10.21 -> 5.30 ns/pt)
    ("gga_x_sg4", "kxc", "unpol"),  # 2.00x  (22.70 -> 11.35 ns/pt)
    ("gga_x_sg4", "lxc", "unpol"),  # 2.40x  (36.30 -> 15.14 ns/pt)
    ("gga_x_sogga11", "fxc", "unpol"),  # 2.56x  (16.67 -> 6.51 ns/pt)
    ("gga_x_sogga11", "kxc", "pol"),  # 1.80x  (94.66 -> 52.64 ns/pt)
    ("gga_x_sogga11", "kxc", "unpol"),  # 2.89x  (29.43 -> 10.18 ns/pt)
    ("gga_x_sogga11", "lxc", "pol"),  # 2.14x  (167.38 -> 78.15 ns/pt)
    ("gga_x_sogga11", "lxc", "unpol"),  # 3.25x  (52.27 -> 16.09 ns/pt)
    ("gga_x_ssb_sw", "fxc", "unpol"),  # 1.90x  (10.90 -> 5.74 ns/pt)
    ("gga_x_ssb_sw", "lxc", "unpol"),  # 2.24x  (31.01 -> 13.82 ns/pt)
    ("gga_x_wc", "kxc", "pol"),  # 1.45x  (66.83 -> 46.03 ns/pt)
    ("gga_x_wc", "kxc", "unpol"),  # 2.05x  (14.19 -> 6.93 ns/pt)
    ("gga_x_wc", "lxc", "pol"),  # 1.99x  (132.05 -> 66.47 ns/pt)
    ("gga_x_wc", "lxc", "unpol"),  # 2.18x  (20.33 -> 9.34 ns/pt)
    ("gga_xc_th1", "kxc", "pol"),  # 2.03x  (113.34 -> 55.73 ns/pt)
    ("gga_xc_th1", "kxc", "unpol"),  # 2.20x  (13.92 -> 6.32 ns/pt)
    ("gga_xc_th1", "lxc", "pol"),  # 2.38x  (203.00 -> 85.24 ns/pt)
    ("gga_xc_th1", "lxc", "unpol"),  # 2.46x  (18.27 -> 7.43 ns/pt)
    ("gga_xc_th2", "kxc", "pol"),  # 1.97x  (115.23 -> 58.43 ns/pt)
    ("gga_xc_th2", "kxc", "unpol"),  # 2.19x  (15.43 -> 7.05 ns/pt)
    ("gga_xc_th2", "lxc", "pol"),  # 2.44x  (205.59 -> 84.25 ns/pt)
    ("gga_xc_th2", "lxc", "unpol"),  # 2.49x  (18.45 -> 7.42 ns/pt)
    ("gga_xc_th3", "kxc", "pol"),  # 2.26x  (140.03 -> 61.92 ns/pt)
    ("gga_xc_th3", "kxc", "unpol"),  # 2.17x  (26.16 -> 12.08 ns/pt)
    ("gga_xc_th3", "lxc", "pol"),  # 2.10x  (194.60 -> 92.59 ns/pt)
    ("gga_xc_th3", "lxc", "unpol"),  # 2.46x  (23.10 -> 9.37 ns/pt)
    ("hyb_gga_xc_wb97", "kxc", "pol"),  # 2.86x  (353.41 -> 123.50 ns/pt)
    ("hyb_gga_xc_wb97", "kxc", "unpol"),  # 2.88x  (66.01 -> 22.94 ns/pt)
    ("hyb_gga_xc_wb97", "lxc", "pol"),  # 3.02x  (878.10 -> 290.72 ns/pt)
    ("hyb_gga_xc_wb97", "lxc", "unpol"),  # 2.98x  (115.55 -> 38.78 ns/pt)
    ("hyb_lda_xc_bn05", "kxc", "pol"),  # 3.30x  (134.73 -> 40.81 ns/pt)
    ("hyb_lda_xc_bn05", "kxc", "unpol"),  # 2.38x  (41.92 -> 17.60 ns/pt)
    ("hyb_lda_xc_bn05", "lxc", "pol"),  # 3.71x  (319.21 -> 86.01 ns/pt)
    ("hyb_lda_xc_bn05", "lxc", "unpol"),  # 2.54x  (43.52 -> 17.12 ns/pt)
    ("hyb_mgga_x_dldf", "fxc", "unpol"),  # 2.21x  (11.04 -> 4.98 ns/pt)
    ("hyb_mgga_x_m05", "fxc", "unpol"),  # 2.76x  (20.39 -> 7.39 ns/pt)
    ("lda_c_1d_csc", "kxc", "pol"),  # 2.54x  (18.12 -> 7.12 ns/pt)
    ("lda_c_1d_csc", "kxc", "unpol"),  # 2.35x  (8.24 -> 3.51 ns/pt)
    ("lda_c_1d_csc", "lxc", "pol"),  # 1.91x  (28.80 -> 15.08 ns/pt)
    ("lda_c_1d_csc", "lxc", "unpol"),  # 1.78x  (12.65 -> 7.11 ns/pt)
    ("lda_c_2d_amgb", "fxc", "unpol"),  # 2.47x  (5.08 -> 2.06 ns/pt)
    ("lda_c_2d_amgb", "kxc", "pol"),  # 3.35x  (37.33 -> 11.13 ns/pt)
    ("lda_c_2d_amgb", "kxc", "unpol"),  # 2.69x  (9.01 -> 3.34 ns/pt)
    ("lda_c_2d_amgb", "lxc", "unpol"),  # 3.00x  (14.91 -> 4.96 ns/pt)
    ("lda_c_chachiyo", "fxc", "unpol"),  # 1.94x  (6.28 -> 3.25 ns/pt)
    ("lda_c_chachiyo", "kxc", "pol"),  # 2.13x  (14.63 -> 6.86 ns/pt)
    ("lda_c_chachiyo", "lxc", "pol"),  # 2.55x  (20.62 -> 8.09 ns/pt)
    ("lda_c_chachiyo_mod", "fxc", "unpol"),  # 1.84x  (6.05 -> 3.30 ns/pt)
    ("lda_c_chachiyo_mod", "kxc", "pol"),  # 2.37x  (16.41 -> 6.92 ns/pt)
    ("lda_c_chachiyo_mod", "lxc", "pol"),  # 3.02x  (27.32 -> 9.03 ns/pt)
    ("lda_c_gk72", "fxc", "unpol"),  # 2.21x  (7.12 -> 3.23 ns/pt)
    ("lda_c_gk72", "lxc", "pol"),  # 2.52x  (14.54 -> 5.77 ns/pt)
    ("lda_c_gk72", "lxc", "unpol"),  # 2.69x  (11.70 -> 4.34 ns/pt)
    ("lda_c_gombas", "fxc", "unpol"),  # 2.25x  (4.56 -> 2.02 ns/pt)
    ("lda_c_gombas", "kxc", "pol"),  # 2.08x  (8.52 -> 4.10 ns/pt)
    ("lda_c_gombas", "kxc", "unpol"),  # 2.21x  (7.20 -> 3.25 ns/pt)
    ("lda_c_gombas", "lxc", "pol"),  # 2.60x  (10.68 -> 4.12 ns/pt)
    ("lda_c_gombas", "lxc", "unpol"),  # 2.50x  (9.31 -> 3.73 ns/pt)
    ("lda_c_hl", "fxc", "unpol"),  # 2.01x  (7.52 -> 3.74 ns/pt)
    ("lda_c_hl", "kxc", "pol"),  # 2.49x  (17.66 -> 7.09 ns/pt)
    ("lda_c_hl", "lxc", "pol"),  # 2.73x  (23.98 -> 8.78 ns/pt)
    ("lda_c_pmgb06", "kxc", "pol"),  # 3.29x  (244.22 -> 74.31 ns/pt)
    ("lda_c_pmgb06", "kxc", "unpol"),  # 2.67x  (51.71 -> 19.35 ns/pt)
    ("lda_c_pmgb06", "lxc", "pol"),  # 3.21x  (777.31 -> 241.92 ns/pt)
    ("lda_c_pmgb06", "lxc", "unpol"),  # 3.17x  (58.98 -> 18.63 ns/pt)
    ("lda_c_pw", "kxc", "pol"),  # 2.72x  (39.36 -> 14.46 ns/pt)
    ("lda_c_pw", "kxc", "unpol"),  # 2.54x  (17.40 -> 6.84 ns/pt)
    ("lda_c_pw", "lxc", "pol"),  # 3.10x  (84.76 -> 27.37 ns/pt)
    ("lda_c_pw", "lxc", "unpol"),  # 2.91x  (24.85 -> 8.54 ns/pt)
    ("lda_c_pw_erf", "kxc", "pol"),  # 3.25x  (256.50 -> 79.02 ns/pt)
    ("lda_c_pw_erf", "kxc", "unpol"),  # 2.79x  (53.58 -> 19.23 ns/pt)
    ("lda_c_pw_erf", "lxc", "pol"),  # 2.98x  (869.25 -> 292.19 ns/pt)
    ("lda_c_pw_erf", "lxc", "unpol"),  # 3.15x  (62.09 -> 19.71 ns/pt)
    ("lda_c_pz", "kxc", "pol"),  # 2.21x  (16.08 -> 7.29 ns/pt)
    ("lda_c_pz", "kxc", "unpol"),  # 1.89x  (8.28 -> 4.38 ns/pt)
    ("lda_c_pz", "lxc", "pol"),  # 2.52x  (22.12 -> 8.77 ns/pt)
    ("lda_c_pz", "lxc", "unpol"),  # 2.02x  (9.83 -> 4.88 ns/pt)
    ("lda_c_rc04", "fxc", "unpol"),  # 1.54x  (5.34 -> 3.47 ns/pt)
    ("lda_c_rc04", "kxc", "pol"),  # 2.08x  (25.01 -> 12.03 ns/pt)
    ("lda_c_rc04", "kxc", "unpol"),  # 1.73x  (6.25 -> 3.60 ns/pt)
    ("lda_c_rc04", "lxc", "unpol"),  # 1.69x  (6.62 -> 3.92 ns/pt)
    ("lda_c_rpa", "fxc", "unpol"),  # 1.81x  (4.00 -> 2.22 ns/pt)
    ("lda_c_rpa", "kxc", "pol"),  # 1.76x  (7.31 -> 4.15 ns/pt)
    ("lda_c_rpa", "lxc", "pol"),  # 1.80x  (7.28 -> 4.04 ns/pt)
    ("lda_c_rpa", "lxc", "unpol"),  # 1.92x  (5.91 -> 3.08 ns/pt)
    ("lda_c_vwn", "kxc", "pol"),  # 4.14x  (77.71 -> 18.78 ns/pt)
    ("lda_c_vwn", "kxc", "unpol"),  # 2.44x  (22.80 -> 9.35 ns/pt)
    ("lda_c_vwn", "lxc", "pol"),  # 3.32x  (112.41 -> 33.87 ns/pt)
    ("lda_c_vwn", "lxc", "unpol"),  # 2.76x  (36.98 -> 13.41 ns/pt)
    ("lda_c_vwn_1", "kxc", "pol"),  # 2.56x  (30.73 -> 12.03 ns/pt)
    ("lda_c_vwn_1", "lxc", "pol"),  # 2.67x  (53.40 -> 20.01 ns/pt)
    ("lda_c_vwn_1", "lxc", "unpol"),  # 2.75x  (50.34 -> 18.29 ns/pt)
    ("lda_c_vwn_2", "kxc", "pol"),  # 3.05x  (79.61 -> 26.09 ns/pt)
    ("lda_c_vwn_2", "kxc", "unpol"),  # 2.58x  (61.88 -> 24.00 ns/pt)
    ("lda_c_vwn_2", "lxc", "pol"),  # 3.40x  (157.35 -> 46.29 ns/pt)
    ("lda_c_vwn_2", "lxc", "unpol"),  # 3.25x  (127.12 -> 39.07 ns/pt)
    ("lda_c_vwn_3", "kxc", "pol"),  # 3.22x  (86.95 -> 27.02 ns/pt)
    ("lda_c_vwn_3", "kxc", "unpol"),  # 2.67x  (65.33 -> 24.46 ns/pt)
    ("lda_c_vwn_3", "lxc", "pol"),  # 3.31x  (195.87 -> 59.19 ns/pt)
    ("lda_c_vwn_3", "lxc", "unpol"),  # 3.06x  (130.84 -> 42.72 ns/pt)
    ("lda_c_vwn_4", "kxc", "pol"),  # 2.77x  (54.49 -> 19.65 ns/pt)
    ("lda_c_vwn_4", "kxc", "unpol"),  # 2.36x  (32.93 -> 13.94 ns/pt)
    ("lda_c_vwn_4", "lxc", "pol"),  # 3.62x  (122.13 -> 33.75 ns/pt)
    ("lda_c_vwn_rpa", "kxc", "pol"),  # 2.92x  (30.04 -> 10.29 ns/pt)
    ("lda_c_vwn_rpa", "lxc", "pol"),  # 3.11x  (51.21 -> 16.45 ns/pt)
    ("lda_c_w20", "kxc", "pol"),  # 2.73x  (58.57 -> 21.48 ns/pt)
    ("lda_c_w20", "kxc", "unpol"),  # 2.63x  (33.24 -> 12.62 ns/pt)
    ("lda_c_w20", "lxc", "pol"),  # 3.50x  (84.52 -> 24.17 ns/pt)
    ("lda_c_w20", "lxc", "unpol"),  # 2.99x  (56.43 -> 18.85 ns/pt)
    ("lda_c_wigner", "fxc", "unpol"),  # 1.48x  (3.00 -> 2.03 ns/pt)
    ("lda_c_wigner", "kxc", "pol"),  # 2.22x  (10.99 -> 4.95 ns/pt)
    ("lda_c_wigner", "kxc", "unpol"),  # 1.59x  (5.22 -> 3.29 ns/pt)
    ("lda_k_tf", "fxc", "unpol"),  # 1.24x  (3.17 -> 2.57 ns/pt)
    ("lda_k_tf", "kxc", "pol"),  # 1.85x  (10.92 -> 5.89 ns/pt)
    ("lda_k_tf", "lxc", "pol"),  # 2.42x  (15.88 -> 6.55 ns/pt)
    ("lda_k_zlp", "fxc", "unpol"),  # 1.74x  (4.96 -> 2.85 ns/pt)
    ("lda_k_zlp", "kxc", "pol"),  # 2.12x  (14.10 -> 6.66 ns/pt)
    ("lda_k_zlp", "lxc", "pol"),  # 2.53x  (21.38 -> 8.46 ns/pt)
    ("lda_x", "fxc", "unpol"),  # 1.70x  (3.62 -> 2.13 ns/pt)
    ("lda_x_erf", "kxc", "pol"),  # 2.76x  (88.85 -> 32.21 ns/pt)
    ("lda_x_erf", "kxc", "unpol"),  # 2.09x  (15.75 -> 7.53 ns/pt)
    ("lda_x_erf", "lxc", "pol"),  # 2.87x  (180.71 -> 63.07 ns/pt)
    ("lda_x_erf", "lxc", "unpol"),  # 2.11x  (19.08 -> 9.05 ns/pt)
    ("lda_x_rel", "fxc", "unpol"),  # 1.70x  (6.29 -> 3.71 ns/pt)
    ("lda_x_rel", "kxc", "unpol"),  # 1.28x  (7.40 -> 5.78 ns/pt)
    ("lda_x_rel", "lxc", "unpol"),  # 1.46x  (8.73 -> 5.99 ns/pt)
    ("lda_x_sloc", "fxc", "unpol"),  # 1.69x  (2.93 -> 1.74 ns/pt)
    ("lda_x_sloc", "kxc", "pol"),  # 1.54x  (12.32 -> 7.97 ns/pt)
    ("lda_x_sloc", "lxc", "pol"),  # 2.21x  (22.12 -> 10.01 ns/pt)
    ("lda_x_yukawa", "kxc", "pol"),  # 2.96x  (99.50 -> 33.61 ns/pt)
    ("lda_x_yukawa", "lxc", "pol"),  # 3.22x  (234.28 -> 72.78 ns/pt)
    ("lda_xc_ksdt", "kxc", "pol"),  # 2.89x  (401.75 -> 139.12 ns/pt)
    ("lda_xc_ksdt", "kxc", "unpol"),  # 3.27x  (86.40 -> 26.39 ns/pt)
    ("lda_xc_ksdt", "lxc", "pol"),  # 3.60x  (2042.31 -> 567.64 ns/pt)
    ("lda_xc_ksdt", "lxc", "unpol"),  # 3.82x  (195.41 -> 51.14 ns/pt)
    ("lda_xc_teter93", "fxc", "unpol"),  # 2.01x  (6.11 -> 3.04 ns/pt)
    ("lda_xc_teter93", "kxc", "pol"),  # 2.81x  (23.72 -> 8.44 ns/pt)
    ("lda_xc_teter93", "lxc", "pol"),  # 3.47x  (44.14 -> 12.73 ns/pt)
    ("lda_xc_zlp", "fxc", "unpol"),  # 2.23x  (3.91 -> 1.75 ns/pt)
    ("lda_xc_zlp", "kxc", "pol"),  # 2.00x  (7.33 -> 3.66 ns/pt)
    ("lda_xc_zlp", "kxc", "unpol"),  # 2.30x  (5.85 -> 2.54 ns/pt)
    ("lda_xc_zlp", "lxc", "pol"),  # 2.35x  (8.24 -> 3.51 ns/pt)
    ("lda_xc_zlp", "lxc", "unpol"),  # 2.48x  (6.81 -> 2.75 ns/pt)
    ("mgga_c_cc", "fxc", "unpol"),  # 2.31x  (9.77 -> 4.22 ns/pt)
    ("mgga_c_ccalda", "fxc", "unpol"),  # 2.42x  (14.94 -> 6.16 ns/pt)
    ("mgga_c_cs", "fxc", "unpol"),  # 2.08x  (7.17 -> 3.45 ns/pt)
    ("mgga_k_gea2", "fxc", "unpol"),  # 1.74x  (7.08 -> 4.06 ns/pt)
    ("mgga_k_gea4", "fxc", "unpol"),  # 1.93x  (8.94 -> 4.63 ns/pt)
    ("mgga_k_rda", "fxc", "unpol"),  # 2.70x  (20.61 -> 7.63 ns/pt)
    ("mgga_x_2d_js17", "fxc", "unpol"),  # 2.63x  (8.30 -> 3.16 ns/pt)
    ("mgga_x_edmgga", "fxc", "unpol"),  # 2.61x  (20.31 -> 7.78 ns/pt)
    ("mgga_x_ft98", "fxc", "unpol"),  # 3.08x  (30.03 -> 9.73 ns/pt)
    ("mgga_x_gx", "fxc", "unpol"),  # 2.64x  (15.35 -> 5.82 ns/pt)
    ("mgga_x_jk", "fxc", "unpol"),  # 2.28x  (13.98 -> 6.14 ns/pt)
    ("mgga_x_mbeef", "fxc", "unpol"),  # 4.05x  (83.81 -> 20.67 ns/pt)
    ("mgga_x_mbeefvdw", "fxc", "unpol"),  # 3.41x  (36.59 -> 10.72 ns/pt)
    ("mgga_x_mcml", "fxc", "unpol"),  # 4.11x  (81.92 -> 19.93 ns/pt)
    ("mgga_x_pbe_gx", "fxc", "unpol"),  # 2.63x  (17.19 -> 6.53 ns/pt)
    ("mgga_x_pkzb", "fxc", "unpol"),  # 1.96x  (9.59 -> 4.89 ns/pt)
    ("mgga_x_regtpss", "fxc", "unpol"),  # 2.91x  (35.79 -> 12.28 ns/pt)
    ("mgga_x_rlda", "fxc", "unpol"),  # 1.87x  (8.30 -> 4.43 ns/pt)
    ("mgga_x_rtpss", "fxc", "unpol"),  # 2.88x  (24.87 -> 8.65 ns/pt)
    ("mgga_x_sa_tpss", "fxc", "unpol"),  # 3.21x  (39.94 -> 12.45 ns/pt)
    ("mgga_x_tau_hcth", "fxc", "unpol"),  # 2.43x  (14.81 -> 6.09 ns/pt)
    ("mgga_x_th", "fxc", "unpol"),  # 1.33x  (4.57 -> 3.44 ns/pt)
    ("mgga_x_vcml", "fxc", "unpol"),  # 4.24x  (85.70 -> 20.21 ns/pt)
    ("mgga_xc_cc06", "fxc", "unpol"),  # 2.34x  (11.86 -> 5.06 ns/pt)
    ("mgga_xc_zlp", "fxc", "unpol"),  # 1.91x  (5.74 -> 3.01 ns/pt)
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
