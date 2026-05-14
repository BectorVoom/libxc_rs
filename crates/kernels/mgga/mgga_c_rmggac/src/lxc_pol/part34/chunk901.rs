//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 901/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk901<F: Float>(t76046: F, t76050: F, t76054: F, t70358: F, t76066: F, t70365: F, t70369: F, t70373: F, t15675: F, t4965: F, t70381: F, t76079: F, t70330: F, t71789: F, t71802: F, t76064: F, t76075: F, t76084: F) -> (F,) {
    let t78473 = 0.20455996240684006298e-1 * t76046;
    let t78474 = 0.20455996240684006298e-1 * t76050;
    let t78475 = 0.20455996240684006298e-1 * t76054;
    let t78476 = 0.86737941314158990619e-4 * t70358;
    let t78477 = 0.14967802127329760705e-1 * t76066;
    let t78478 = 0.30487649791575028312e-3 * t70365;
    let t78479 = 0.43368970657079495308e-4 * t70369;
    let t78480 = 0.30487649791575028312e-3 * t70373;
    let t78482 = 0.11974241701863808564e0 * t4965 * t15675;
    let t78483 = 0.16263363996404810741e-4 * t70381;
    let t78484 = 0.13637330827122670865e-1 * t76079;
    let t78485 = -t71789 - 0.40878380883436523436e-5 * t70330 - t78473 - t78474 - t78475 - t76064 + t78476 - t78477 - t78478 + t78479 - t78480 + t71802 - t78482 + t76075 + t78483 + t78484 - t76084;
    (t78485,)
}
