//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 862/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk862<F: Float>(t120: F, t781: F, t118: F, t123: F, t116: F, t16: F, t2397: F, t9691: F, t693: F, t9694: F, t119: F, t133: F, t625: F) -> (F, F, F, F, F, F) {
    let t9697 = t120 * t781;
    let t9698 = t118 * t9697;
    let t9700 = F::new(1.0)/pow_3_2::<F>(t123);
    let t9701 = t9700 * t116;
    let t9702 = t9701 * t16;
    let t9704 = t2397 * t9691;
    let t9706 = t693 * t9694;
    let t9709 = t133 * t119 * t625;
    (t9697, t9698, t9702, t9704, t9706, t9709)
}
