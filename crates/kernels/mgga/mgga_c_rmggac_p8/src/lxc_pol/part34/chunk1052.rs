//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1052/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1052<F: Float>(t75312: F, t75314: F, t75316: F, t75319: F, t75322: F, t75328: F, t75332: F, t75334: F, t75337: F, t75340: F, t75344: F, t75347: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t78132 = F::cast_from(0.39828462315181744018e-2_f64) * t75312;
    let t78133 = F::cast_from(0.39828462315181744018e-2_f64) * t75314;
    let t78134 = F::cast_from(0.69699809051568052031e-2_f64) * t75316;
    let t78135 = F::cast_from(0.11571889285499841527e-2_f64) * t75319;
    let t78136 = F::cast_from(0.11571889285499841527e-2_f64) * t75322;
    let t78138 = F::cast_from(0.36366215538993788972e-1_f64) * t75328;
    let t78140 = F::cast_from(0.16566831523319392755e-1_f64) * t75332;
    let t78141 = F::cast_from(0.27611385872198987925e-1_f64) * t75334;
    let t78142 = F::cast_from(0.49892673757765869017e-2_f64) * t75337;
    let t78143 = F::cast_from(0.31062809106223861416e-1_f64) * t75340;
    let t78144 = F::cast_from(0.1814407727691612783e-2_f64) * t75344;
    let t78145 = F::cast_from(0.31752135234603223702e-2_f64) * t75347;
    (t78132, t78133, t78134, t78135, t78136, t78138, t78140, t78141, t78142, t78143, t78144, t78145)
}
