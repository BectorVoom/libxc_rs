//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1087/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1087<F: Float>(t4199: F, t9892: F, t13123: F, t9882: F, t9888: F, t9905: F, t9494: F, t9885: F, t9722: F, t1409: F, t707: F, t9862: F, t9467: F, t9713: F, t1471: F, t31: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t46130 = t4199 * t9892;
    let t46132 = t13123 * t9882;
    let t46134 = t13123 * t9888;
    let t46196 = t4199 * t9905;
    let t46208 = t4199 * t9494;
    let t46278 = t13123 * t9885;
    let t46302 = t4199 * t9722;
    let t46369 = t707 * t9862 * t1409;
    let t46371 = t13123 * t9467;
    let t46376 = t4199 * t9713;
    let t46387 = t31 * t1471;
    (t46130, t46132, t46134, t46196, t46208, t46278, t46302, t46369, t46371, t46376, t46387)
}
