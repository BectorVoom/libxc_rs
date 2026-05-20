//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2788/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2788<F: Float>(t46376: F, t16710: F, t2663: F, t41255: F, t41259: F, t46433: F, t46435: F, t46437: F, t46439: F, t16717: F, t47176: F, t157: F, t46387: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t58983 = F::cast_from(0.11696447245269292414e1_f64) * t46376;
    let t58984 = t16710 * t2663;
    let t58985 = F::cast_from(0.24415263074675393405e-3_f64) * t58984;
    let t58986 = F::cast_from(0.5848223622634646207e0_f64) * t41255;
    let t58987 = F::cast_from(0.11696447245269292414e1_f64) * t41259;
    let t58988 = F::cast_from(0.11393789434848516923e-2_f64) * t46433;
    let t58989 = F::cast_from(0.97661052298701573622e-3_f64) * t46435;
    let t58990 = F::new(4.0) * t46437;
    let t58991 = F::new(2.0) * t46439;
    let t58993 = F::new(48.0) * t47176 * t16717;
    let t58994 = t46387 * t157;
    (t58983, t58985, t58986, t58987, t58988, t58989, t58990, t58991, t58993, t58994)
}
