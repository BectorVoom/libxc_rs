//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1011/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1011<F: Float>(t44736: F, t5259: F, t46181: F, t7844: F, t1763: F, t262: F, t265: F, t7835: F, t46185: F, t7829: F, t2068: F, t46117: F) -> (F, F, F, F, F) {
    let t46634 = t5259 * t44736;
    let t46642 = t7844 * t46181;
    let t46646 = t7835 * t262 * t265 * t1763;
    let t46648 = t7829 * t46185;
    let t46650 = t2068 * t46117;
    (t46634, t46642, t46646, t46648, t46650)
}
