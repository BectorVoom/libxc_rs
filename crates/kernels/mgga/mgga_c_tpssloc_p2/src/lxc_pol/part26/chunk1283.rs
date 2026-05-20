//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1283/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1283<F: Float>(t23047: F, t2617: F, t2635: F, t2690: F, t6612: F, t812: F, t831: F, t23041: F, t2686: F, t6614: F, t9663: F, t23048: F, t9983: F) -> (F, F, F, F, F) {
    let t81803 = t2617 * t23047;
    let t81804 = t81803 * t2635;
    let t81807 = t812 * t6612 * t2690;
    let t81808 = t81807 * t831;
    let t81810 = t23041 * t2686;
    let t81812 = t6614 * t9663;
    let t81814 = t23048 * t9983;
    (t81804, t81808, t81810, t81812, t81814)
}
