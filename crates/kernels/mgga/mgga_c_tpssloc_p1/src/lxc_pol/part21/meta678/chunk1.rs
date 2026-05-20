//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2486/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2486<F: Float>(t1409: F, t707: F, t9862: F, t13123: F, t9467: F, t4199: F, t9713: F, t1471: F, t31: F, t4211: F, t9874: F, t13119: F, t2663: F) -> (F, F, F, F, F, F) {
    let t46369 = t707 * t9862 * t1409;
    let t46371 = t13123 * t9467;
    let t46376 = t4199 * t9713;
    let t46387 = t31 * t1471;
    let t46433 = t4211 * t9874;
    let t46435 = t13119 * t2663;
    (t46369, t46371, t46376, t46387, t46433, t46435)
}
