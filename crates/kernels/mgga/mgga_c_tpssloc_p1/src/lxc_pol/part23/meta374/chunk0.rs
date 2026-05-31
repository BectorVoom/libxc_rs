//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1175/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1175<F: Float>(t35577: F, t1454: F, t2585: F, t1406: F, t9238: F, t4199: F, t9919: F, t9892: F, t13123: F, t9882: F, t9888: F, t9905: F) -> (F, F, F, F, F, F, F, F) {
    let t45496 = F::cast_from(1.0_f64) / t35577;
    let t45656 = t2585 * t1454;
    let t45844 = t1406 * t9238;
    let t46125 = t4199 * t9919;
    let t46130 = t4199 * t9892;
    let t46132 = t13123 * t9882;
    let t46134 = t13123 * t9888;
    let t46196 = t4199 * t9905;
    (t45496, t45656, t45844, t46125, t46130, t46132, t46134, t46196)
}
