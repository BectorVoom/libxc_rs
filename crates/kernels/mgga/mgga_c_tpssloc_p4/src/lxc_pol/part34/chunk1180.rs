//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1180/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1180<F: Float>(t1530: F, t5544: F, t22960: F, t5527: F, t28248: F, t86721: F, t5660: F, t25373: F, t193: F, t20756: F, t5397: F, t21066: F, t25: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t105758 = t5544 * t1530;
    let t105759 = t22960 * t105758;
    let t105762 = t5527 * t1530;
    let t105763 = t22960 * t105762;
    let t105766 = t86721 * t28248;
    let t105769 = t1530 * t5660;
    let t105770 = t25373 * t105769;
    let t105773 = t193 * t20756;
    let t105780 = t5397 * t1530;
    let t105787 = t25 * t21066;
    (t105758, t105759, t105762, t105763, t105766, t105769, t105770, t105773, t105780, t105787)
}
