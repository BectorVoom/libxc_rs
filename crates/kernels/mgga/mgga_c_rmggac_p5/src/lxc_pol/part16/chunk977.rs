//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 977/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk977<F: Float>(t265: F, t9908: F, t46128: F, t851: F, t46176: F, t854: F, t3810: F, t46184: F, t3839: F, t46180: F, t2068: F, t46129: F) -> (F, F, F, F, F, F) {
    let t46302 = t9908 * t265;
    let t46305 = t851 * t46128;
    let t46307 = t854 * t46176;
    let t46309 = t3810 * t46184;
    let t46311 = t3839 * t46180;
    let t46320 = t2068 * t46129;
    (t46302, t46305, t46307, t46309, t46311, t46320)
}
