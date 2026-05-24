//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 979/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk979<F: Float>(t1652: F, t2347: F, t262: F, t7788: F, t45731: F, t7785: F, t1734: F, t2064: F, t793: F, t5267: F, t25820: F, t5888: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46357 = t2347 * t1652;
    let t46358 = t262 * t46357;
    let t46359 = t7788 * t46358;
    let t46361 = t7785 * t45731;
    let t46369 = t2064 * t1734;
    let t46370 = t793 * t46369;
    let t46385 = t2347 * t5267;
    let t46386 = t25820 * t46385;
    let t46388 = t2347 * t5888;
    (t46357, t46358, t46359, t46361, t46369, t46370, t46385, t46386, t46388)
}
