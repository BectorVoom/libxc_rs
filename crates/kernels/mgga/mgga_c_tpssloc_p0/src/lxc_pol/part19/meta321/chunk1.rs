//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1139/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1139<F: Float>(t39609: F, t1285: F, t9218: F, t16: F, t185: F, t520: F, t1284: F, t17: F, t9861: F, t3719: F) -> (F, F, F, F, F) {
    let t39610 = F::new(960.0) * t39609;
    let t39611 = t9218 * t1285;
    let t39612 = F::new(480.0) * t39611;
    let t39615 = F::new(24.0) * t16 * t520 * t185;
    let t39620 = t17 * t1284 * t9861;
    let t39621 = F::new(4.0) * t39620;
    let t39622 = t3719 * t3719;
    (t39610, t39612, t39615, t39621, t39622)
}
