//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2220/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2220<F: Float>(t16: F, t39031: F, t39033: F, t39035: F, t39037: F, t39039: F, t12566: F, t604: F, t2239: F, t3951: F, t12571: F, t12582: F, t12719: F, t1437: F, t2240: F, t2241: F, t39043: F, t39049: F, t39054: F, t3953: F, t3958: F, t4021: F, t45986: F, t46022: F, t46050: F, t46080: F, t605: F, t645: F, t86: F, t9239: F, t9243: F, t9342: F) -> F {
    let t46085 = F::new(12.0) * t16;
    let t46086 = F::new(0.1248e2) * t39031;
    let t46087 = F::new(0.7092e3) * t39033;
    let t46088 = F::new(0.27744e4) * t39035;
    let t46089 = F::new(420.0) * t39037;
    let t46090 = F::new(0.911232e4) * t39039;
    let t46099 = t12566 * t604;
    let t46104 = t3951 * t2239;
    let t46114 = F::new(60.0) * t12571 * t9243 - F::new(4.0) * t605 * (t45986 + t46022 + t46050 + t46080) + (-t46085 + t46086 - t46087 + t46088 + t46089 - t46090 + t39043) * t86 - F::new(360.0) * t9239 * t4021 * t2241 + F::new(20.0) * t2240 * t1437 * t9342 - F::new(12.0) * t46099 * t645 - F::new(360.0) * t39054 * t12582 + F::new(60.0) * t46104 * t2241 + F::new(60.0) * t39049 * t3958 - F::new(4.0) * t3953 * t9342 + F::new(60.0) * t2240 * t12719 * t645;
    t46114
}
