//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1093/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1093<F: Float>(t39277: F, t9118: F, t1910: F, t3351: F, t352: F, t515: F, t7231: F, t7720: F, t9790: F, t46420: F, t7204: F, t46424: F, t7192: F) -> (F, F, F, F, F) {
    let t47876 = t39277 * t9118;
    let t47881 = t3351 * t7231 * t515 * t1910 * t352;
    let t47883 = t7720 * t9790;
    let t47885 = t7204 * t46420;
    let t47887 = t7192 * t46424;
    (t47876, t47881, t47883, t47885, t47887)
}
