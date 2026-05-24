//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 976/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk976<F: Float>(t41176: F, t6387: F, t3814: F, t46184: F, t46121: F, t854: F, t6444: F, t9872: F, t46116: F, t793: F, t46176: F, t797: F) -> (F, F, F, F, F, F) {
    let t46285 = t41176 * t6387;
    let t46287 = t3814 * t46184;
    let t46289 = t854 * t46121;
    let t46291 = t6444 * t9872;
    let t46293 = t793 * t46116;
    let t46300 = t797 * t46176;
    (t46285, t46287, t46289, t46291, t46293, t46300)
}
