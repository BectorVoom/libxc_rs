//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 990/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk990<F: Float>(t46185: F, t7829: F, t2068: F, t46117: F, t2073: F, t46122: F, t2079: F, t262: F, t36: F, t6463: F, t27041: F, t45568: F) -> (F, F, F, F, F) {
    let t46648 = t7829 * t46185;
    let t46650 = t2068 * t46117;
    let t46652 = t2073 * t46122;
    let t46656 = t2079 * t262 * t36 * t6463;
    let t46658 = t27041 * t45568;
    (t46648, t46650, t46652, t46656, t46658)
}
