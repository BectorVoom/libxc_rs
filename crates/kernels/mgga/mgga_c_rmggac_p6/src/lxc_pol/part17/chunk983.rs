//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 983/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk983<F: Float>(t30526: F, t9708: F, t25525: F, t321: F, t9884: F, t333: F, t25529: F, t3826: F, t45730: F, t25518: F, t45568: F, t25636: F, t45572: F) -> (F, F, F, F, F, F, F) {
    let t46232 = t30526 * t9708;
    let t46235 = t25525 * t9884 * t321;
    let t46237 = t9884 * t333;
    let t46238 = t25529 * t46237;
    let t46242 = t3826 * t45730;
    let t46244 = t25518 * t45568;
    let t46246 = t25636 * t45572;
    (t46232, t46235, t46237, t46238, t46242, t46244, t46246)
}
