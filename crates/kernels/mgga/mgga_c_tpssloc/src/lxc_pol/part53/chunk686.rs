//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 686/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk686<F: Float>(t23062: F, t6593: F, t229: F, t6546: F, t805: F, t243: F, t598: F, t6584: F, t6604: F, t6606: F, t1891: F, t22822: F, t133: F, t6601: F, t6590: F, t22813: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23063 = t23062 * t6593;
    let t23069 = t6546 * t229;
    let t23070 = t23069 * t805;
    let t23071 = 7.0 / 72.0 * t23070;
    let t23075 = t243 * t243;
    let t23076 = 1.0 / t23075;
    let t23077 = t598 * t23076;
    let t23083 = t6584 * t6604;
    let t23084 = t23083 * t6606;
    let t23093 = t22822 * t1891;
    let t23094 = t23093 * t133;
    let t23095 = t23094 * t6601;
    let t23097 = t6590 * t6604;
    let t23102 = t22813 * t1891;
    (t23063, t23069, t23071, t23077, t23083, t23084, t23094, t23095, t23097, t23102)
}
