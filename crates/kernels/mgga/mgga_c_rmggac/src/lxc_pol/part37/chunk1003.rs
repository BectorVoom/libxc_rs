//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1003/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1003<F: Float>(t69261: F, t75300: F, t69274: F, t75308: F, t75312: F, t75314: F, t75316: F, t75319: F, t75322: F, t75332: F, t75334: F, t75337: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t78126 = F::cast_from(0.10286123809333192469e-2_f64) * t69261;
    let t78127 = F::cast_from(0.77145928569998943515e-3_f64) * t75300;
    let t78129 = F::cast_from(0.26609426004141796809e-1_f64) * t69274;
    let t78130 = F::cast_from(0.19914231157590872009e-2_f64) * t75308;
    let t78132 = F::cast_from(0.39828462315181744018e-2_f64) * t75312;
    let t78133 = F::cast_from(0.39828462315181744018e-2_f64) * t75314;
    let t78134 = F::cast_from(0.69699809051568052031e-2_f64) * t75316;
    let t78135 = F::cast_from(0.11571889285499841527e-2_f64) * t75319;
    let t78136 = F::cast_from(0.11571889285499841527e-2_f64) * t75322;
    let t78140 = F::cast_from(0.16566831523319392755e-1_f64) * t75332;
    let t78141 = F::cast_from(0.27611385872198987925e-1_f64) * t75334;
    let t78142 = F::cast_from(0.49892673757765869017e-2_f64) * t75337;
    (t78126, t78127, t78129, t78130, t78132, t78133, t78134, t78135, t78136, t78140, t78141, t78142)
}
