//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 997/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk997<F: Float>(t262: F, t46258: F, t7829: F, t352: F, t9876: F, t7782: F, t10122: F, t321: F, t7788: F, t333: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t46412 = t262 * t46258;
    let t46413 = t7829 * t46412;
    let t46415 = t9876 * t352;
    let t46416 = t262 * t46415;
    let t46417 = t7782 * t46416;
    let t46419 = t10122 * t321;
    let t46420 = t262 * t46419;
    let t46421 = t7788 * t46420;
    let t46423 = t10122 * t333;
    let t46424 = t262 * t46423;
    let t46425 = t7782 * t46424;
    let t46427 = t10122 * t352;
    (t46412, t46413, t46415, t46416, t46417, t46419, t46420, t46421, t46423, t46424, t46425, t46427)
}
