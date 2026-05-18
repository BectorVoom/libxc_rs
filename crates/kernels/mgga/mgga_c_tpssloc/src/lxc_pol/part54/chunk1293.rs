//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1293/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1293<F: Float>(t31628: F, t6914: F, t22704: F, t22705: F, t31627: F, t31632: F, t6883: F, t22724: F, t31623: F, t22716: F, t8631: F, t31631: F, t6897: F, t794: F) -> (F, F, F, F, F, F) {
    let t115415 = t6914 * t31628;
    let t115423 = t22704 * t22705 * t31627;
    let t115430 = t6883 * t31632;
    let t115432 = t22724 * t31623;
    let t115433 = F::new(0.26044789391763585244e-1) * t115432;
    let t115434 = t22716 * t8631;
    let t115435 = F::new(0.63969658155208805863e-1) * t115434;
    let t115439 = t6897 * t794 * t31631;
    (t115415, t115423, t115430, t115433, t115435, t115439)
}
