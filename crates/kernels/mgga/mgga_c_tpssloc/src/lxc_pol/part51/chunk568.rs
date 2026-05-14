//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 568/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk568<F: Float>(t1409: F, t3242: F, t607: F, t3240: F, t123: F, t3247: F, t1088: F, t1089: F, t3966: F, t3237: F, t3238: F, t4721: F, t423: F, t1098: F, t1657: F, t1119: F) -> (F, F, F, F, F, F, F, F) {
    let t4723 = t3242 * t1409;
    let t4724 = t4723 * t607;
    let t4725 = t3240 * t4724;
    let t4726 = t123 * t4725;
    let t4728 = t3247 * t1409;
    let t4729 = t4728 * t607;
    let t4730 = t1088 * t4729;
    let t4731 = t123 * t4730;
    let t4733 = t1089 * t3966;
    let t4734 = t1088 * t4733;
    let t4735 = t123 * t4734;
    let t4737 = t3237 - 0.5936111111111111111e-2 * t3238 - 0.5936111111111111111e-2 * t4721 - 0.11872222222222222222e-1 * t4726 + 0.35616666666666666666e-1 * t4731 + 0.17808333333333333333e-1 * t4735;
    let t4739 = 0.621814e-1 * t4737 * t423;
    let t4740 = t1657 * t1098;
    let t4742 = 1.0 * t4740 * t1119;
    (t4724, t4726, t4729, t4731, t4733, t4735, t4739, t4742)
}
