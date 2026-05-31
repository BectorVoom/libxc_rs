//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2096/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2096<F: Float>(t16: F, t39031: F, t39: F, t9287: F, t51: F, t9300: F, t39033: F, t39035: F, t39037: F, t39039: F, t2239: F, t3951: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t45869 = F::cast_from(12.0_f64) * t16;
    let t45870 = F::cast_from(24.0_f64) * t39031;
    let t45970 = t39 * t9287;
    let t45974 = t51 * t9300;
    let t46085 = F::cast_from(12.0_f64) * t16;
    let t46086 = F::cast_from(0.1248e2_f64) * t39031;
    let t46087 = F::cast_from(0.7092e3_f64) * t39033;
    let t46088 = F::cast_from(0.27744e4_f64) * t39035;
    let t46089 = F::cast_from(420.0_f64) * t39037;
    let t46090 = F::cast_from(0.911232e4_f64) * t39039;
    let t46104 = t3951 * t2239;
    (t45869, t45870, t45970, t45974, t46085, t46086, t46087, t46088, t46089, t46090, t46104)
}
