//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 883/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk883<F: Float>(t75312: F, t75314: F, t75316: F, t75319: F, t75322: F, t75328: F, t75332: F, t75334: F, t75337: F, t75340: F, t75344: F, t75347: F, t71277: F, t71278: F, t75326: F, t75330: F) -> (F,) {
    let t78132 = 0.39828462315181744018e-2 * t75312;
    let t78133 = 0.39828462315181744018e-2 * t75314;
    let t78134 = 0.69699809051568052031e-2 * t75316;
    let t78135 = 0.11571889285499841527e-2 * t75319;
    let t78136 = 0.11571889285499841527e-2 * t75322;
    let t78138 = 0.36366215538993788972e-1 * t75328;
    let t78140 = 0.16566831523319392755e-1 * t75332;
    let t78141 = 0.27611385872198987925e-1 * t75334;
    let t78142 = 0.49892673757765869017e-2 * t75337;
    let t78143 = 0.31062809106223861416e-1 * t75340;
    let t78144 = 0.1814407727691612783e-2 * t75344;
    let t78145 = 0.31752135234603223702e-2 * t75347;
    let t78146 = -t78132 - t71277 + t71278 - t78133 + t78134 - t78135 - t78136 + 0.1735783392824976229e-2 * t75326 + t78138 - 0.72732431077987577941e-1 * t75330 - t78140 + t78141 + t78142 + t78143 - t78144 + t78145;
    (t78146,)
}
