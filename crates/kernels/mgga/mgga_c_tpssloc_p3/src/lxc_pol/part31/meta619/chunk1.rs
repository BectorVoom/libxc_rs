//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1870/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1870<F: Float>(t1352: F, t22633: F, t6976: F, t96964: F, t96951: F, t19743: F, t3807: F, t1992: F, t20014: F, t1351: F, t550: F, t6434: F) -> (F, F, F, F, F) {
    let t96967 = t22633 * t6976 * t96964 * t1352;
    let t96972 = t22633 * t6976 * t96951 * t1352;
    let t96976 = t22633 * t6976 * t19743 * t3807;
    let t96979 = t1992 * t6976 * t20014;
    let t96986 = t1992 * t6976 * t6434 * t1351 * t550;
    (t96967, t96972, t96976, t96979, t96986)
}
