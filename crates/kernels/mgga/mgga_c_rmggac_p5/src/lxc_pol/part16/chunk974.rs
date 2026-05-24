//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 974/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk974<F: Float>(t3851: F, t45726: F, t45730: F, t25640: F, t45568: F, t333: F, t9876: F, t3814: F, t9872: F, t3810: F, t45418: F, t3826: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t46252 = t3851 * t45726;
    let t46254 = t3851 * t45730;
    let t46256 = t25640 * t45568;
    let t46258 = t9876 * t333;
    let t46259 = t3814 * t46258;
    let t46261 = t9872 * t333;
    let t46262 = t3851 * t46261;
    let t46266 = t3810 * t46258;
    let t46268 = t3851 * t45418;
    let t46270 = t3826 * t46261;
    (t46252, t46254, t46256, t46258, t46259, t46261, t46262, t46266, t46268, t46270)
}
