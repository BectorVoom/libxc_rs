//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 891/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk891<F: Float>(t1992: F, t22635: F, t31558: F, t3911: F, t22716: F, t8622: F, t6897: F, t80645: F, t8621: F, t22633: F, t31550: F, t80650: F, t26331: F, t31549: F, t3734: F, t22704: F, t31559: F, t81326: F) -> (F, F, F, F, F, F) {
    let t115303 = t1992 * t22635 * t31558 * t3911;
    let t115305 = t22716 * t8622;
    let t115306 = 0.63969658155208805863e-1 * t115305;
    let t115308 = t6897 * t80645 * t8621;
    let t115311 = t22633 * t80650 * t31550;
    let t115315 = t26331 * t22635 * t31549 * t3734;
    let t115318 = t22704 * t81326 * t31559;
    (t115303, t115306, t115308, t115311, t115315, t115318)
}
