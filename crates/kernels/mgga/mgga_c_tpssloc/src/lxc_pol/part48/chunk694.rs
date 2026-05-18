//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 694/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk694<F: Float>(t22724: F, t6973: F, t6982: F, t794: F, t6897: F, t6883: F, t6983: F, t1307: F, t562: F, t1352: F, t6976: F, t22633: F) -> (F, F, F, F, F, F) {
    let t22725 = t22724 * t6973;
    let t22726 = F::new(0.26044789391763585244e-1) * t22725;
    let t22727 = t794 * t6982;
    let t22728 = t6897 * t22727;
    let t22730 = t6883 * t6983;
    let t22731 = F::new(0.38381794893125283518e-1) * t22730;
    let t22732 = t562 * t1307;
    let t22733 = t22732 * t1352;
    let t22734 = t6976 * t22733;
    let t22735 = t22633 * t22734;
    (t22725, t22726, t22728, t22730, t22731, t22735)
}
