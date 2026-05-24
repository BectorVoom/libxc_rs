//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1028/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1028<F: Float>(t10090: F, t16043: F, t1818: F, t236: F, t321: F, t3351: F, t35312: F, t333: F, t511: F, t9210: F, t352: F, t515: F) -> (F, F, F, F) {
    let t46906 = t16043 * t10090;
    let t46911 = t3351 * t35312 * t236 * t1818 * t321;
    let t46916 = t3351 * t9210 * t511 * t1818 * t333;
    let t46921 = t3351 * t9210 * t515 * t1818 * t352;
    (t46906, t46911, t46916, t46921)
}
