//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 843/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk843<F: Float>(t641: F, t31: F, t607: F, t645: F, t608: F, t6504: F, t8308: F, t79: F, t8306: F, t31005: F, t8513: F, t22633: F, t22635: F, t31090: F, t90506: F, t22642: F, t22643: F, t8458: F) -> (F, F, F, F, F, F, F, F, F) {
    let t113836 = t641 * t641;
    let t113864 = t645 * t31 * t607;
    let t113871 = t8308 * t608 * t6504;
    let t113875 = t8306 * t79;
    let t113876 = t608 * t641;
    let t113890 = t8513 * t31005 * t6504;
    let t113907 = t8513 * t79 * t6504 * t641;
    let t113931 = 0.13159472534785811492e0 * t22633 * t22635 * t31090 * t90506;
    let t113934 = 0.16449340668482264365e-1 * t22642 * t22643 * t8458;
    (t113836, t113864, t113871, t113875, t113876, t113890, t113907, t113931, t113934)
}
