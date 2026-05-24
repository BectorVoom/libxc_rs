//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1042/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1042<F: Float>(t46530: F, t7192: F, t34938: F, t46534: F, t34944: F, t46538: F, t41738: F, t46542: F, t4044: F, t6400: F, t645: F, t4601: F, t9739: F) -> (F, F, F, F, F, F) {
    let t47723 = t7192 * t46530;
    let t47725 = t34938 * t46534;
    let t47727 = t34944 * t46538;
    let t47729 = t41738 * t46542;
    let t47735 = t4044 * t645 * t6400;
    let t47737 = t4601 * t9739;
    (t47723, t47725, t47727, t47729, t47735, t47737)
}
