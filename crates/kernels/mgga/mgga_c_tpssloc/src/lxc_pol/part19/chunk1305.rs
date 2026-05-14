//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1305/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1305<F: Float>(t3355: F, t427: F, t3358: F, t11176: F, t1147: F, t3368: F, t3400: F, t11285: F, t11300: F, t11307: F, t11353: F, t11356: F, t11361: F, t11365: F, t1137: F, t11400: F, t11415: F, t11420: F, t1156: F, t1157: F, t3332: F, t3357: F, t3359: F, t3371: F, t3396: F, t3401: F, t3403: F, t3404: F, t43679: F, t44142: F, t44146: F, t44155: F, t44161: F, t44164: F, t44167: F, t44168: F, t44172: F) -> (F,) {
    let t44175 = t3355 * t3355;
    let t44177 = t427 / t44175;
    let t44178 = t3358 * t3358;
    let t44179 = 1.0 / t44178;
    let t44183 = t11176 * t1147;
    let t44188 = t3368 * t3400;
    let t44198 = 24.0 * t11415 * t11307 - 24.0 * t11420 * t44142 * t1137 - 6.0 * t3332 * t44146 * t1137 + 0.96491876992155210402e2 * t3357 * t44146 * t3359 - 0.12304822629859687989e5 * t44155 * t43679 * t11285 + t44161 + t44164 - t44167 + 0.51947577317044391277e2 * t3401 * t44168 * t3403 + 0.82761620670837440481e4 * t44172 * t11353 + 0.19964560303604640732e6 * t44177 * t44142 * t44179 + 0.23392894490538584828e1 * t44183 * t1157 + 0.35089341735807877242e1 * t11356 * t3396 + 0.10389515463408878255e3 * t44188 * t3404 + 0.23392894490538584828e1 * t3371 * t11400 + 0.14035736694323150897e2 * t11361 * t11300 - 0.14035736694323150897e2 * t11365 * t43679 * t1156;
    (t44198,)
}
