//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 961/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk961<F: Float>(t2289: F, t38638: F, t16043: F, t9964: F, t30344: F, t3351: F, t3352: F, t515: F, t17787: F, t9005: F, t10112: F, t325: F) -> (F, F, F, F, F) {
    let t46026 = t38638 * t2289;
    let t46034 = t16043 * t9964;
    let t46038 = t3351 * t3352 * t515 * t30344;
    let t46040 = t17787 * t9005;
    let t46042 = t10112 * t325;
    (t46026, t46034, t46038, t46040, t46042)
}
