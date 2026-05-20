//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2253/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2253<F: Float>(t13278: F, t2681: F, t4236: F, t9674: F, t13186: F, t2697: F, t13289: F, t41011: F, t4179: F, t820: F, t1509: F, t2678: F) -> (F, F, F, F, F, F) {
    let t46675 = t13278 * t2681;
    let t46677 = t9674 * t4236;
    let t46679 = t2697 * t13186;
    let t46686 = t41011 * t13289;
    let t46692 = t4179 * t820;
    let t46693 = t1509 * t2678;
    (t46675, t46677, t46679, t46686, t46692, t46693)
}
