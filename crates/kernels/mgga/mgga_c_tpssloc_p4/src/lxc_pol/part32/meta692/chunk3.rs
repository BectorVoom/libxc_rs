//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2145/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2145<F: Float>(t19893: F, t90914: F, t90915: F, t1799: F, t1834: F, t1352: F, t22633: F, t6976: F, t96951: F, t19743: F, t3807: F, t1992: F, t20014: F) -> (F, F, F, F, F, F) {
    let t96962 = t90914 * t90915 * t19893;
    let t96964 = t1834 * t1799;
    let t96967 = t22633 * t6976 * t96964 * t1352;
    let t96972 = t22633 * t6976 * t96951 * t1352;
    let t96976 = t22633 * t6976 * t19743 * t3807;
    let t96979 = t1992 * t6976 * t20014;
    (t96962, t96964, t96967, t96972, t96976, t96979)
}
