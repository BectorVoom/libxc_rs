//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2230/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2230<F: Float>(t40736: F, t10126: F, t12854: F, t1877: F, t2522: F, t2745: F, t40732: F, t4119: F, t4307: F, t46235: F, t46237: F, t46238: F, t46239: F, t46240: F, t46245: F, t46252: F) -> (F, F) {
    let t46256 = F::new(4.0) * t40736;
    let t46257 = F::new(9.0) * t10126 * t2522 * t4119 - F::new(3.0) * t12854 * t1877 * t2745 - F::new(9.0) * t2522 * t4307 * t46240 - F::new(9.0) * t2522 * t4307 * t46252 - t40732 - t46235 + t46237 + t46238 - t46239 + t46245 + t46256;
    (t46256, t46257)
}
