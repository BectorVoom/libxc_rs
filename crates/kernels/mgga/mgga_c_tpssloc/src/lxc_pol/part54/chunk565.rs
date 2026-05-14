//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 565/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk565<F: Float>(t1088: F, t4733: F, t123: F, t3237: F, t3238: F, t4721: F, t4726: F, t4731: F, t423: F, t1098: F, t1657: F, t1119: F, t1671: F, t3259: F, t1117: F, t3264: F) -> (F, F, F, F, F) {
    let t4734 = t1088 * t4733;
    let t4735 = t123 * t4734;
    let t4737 = t3237 - 0.5936111111111111111e-2 * t3238 - 0.5936111111111111111e-2 * t4721 - 0.11872222222222222222e-1 * t4726 + 0.35616666666666666666e-1 * t4731 + 0.17808333333333333333e-1 * t4735;
    let t4739 = 0.621814e-1 * t4737 * t423;
    let t4740 = t1657 * t1098;
    let t4742 = 1.0 * t4740 * t1119;
    let t4744 = 1.0 * t3259 * t1671;
    let t4745 = t1671 * t1117;
    let t4747 = 2.0 * t3264 * t4745;
    (t4735, t4739, t4742, t4744, t4747)
}
