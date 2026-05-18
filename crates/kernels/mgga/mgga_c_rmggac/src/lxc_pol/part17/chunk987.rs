//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 987/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk987<F: Float>(t3839: F, t46278: F, t41165: F, t6387: F, t41262: F, t6382: F, t41176: F, t3814: F, t46184: F, t46121: F, t854: F, t6444: F, t9872: F) -> (F, F, F, F, F, F, F) {
    let t46279 = t3839 * t46278;
    let t46281 = t41165 * t6387;
    let t46283 = t41262 * t6382;
    let t46285 = t41176 * t6387;
    let t46287 = t3814 * t46184;
    let t46289 = t854 * t46121;
    let t46291 = t6444 * t9872;
    (t46279, t46281, t46283, t46285, t46287, t46289, t46291)
}
