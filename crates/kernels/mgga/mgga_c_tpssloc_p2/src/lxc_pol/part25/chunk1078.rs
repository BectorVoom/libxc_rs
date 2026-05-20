//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1078/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1078<F: Float>(t89: F, t9416: F, t88: F, t2745: F, t776: F, t2553: F, t868: F, t2379: F, t2749: F, t2678: F, t829: F, t828: F, t9632: F) -> (F, F, F, F, F, F, F, F, F) {
    let t45640 = t89 * t9416;
    let t45814 = t88 * t9416;
    let t46240 = t2745 * t776;
    let t46252 = t2553 * t868;
    let t46298 = t2379 * t868;
    let t46320 = t776 * t2749;
    let t46362 = t2745 * t868;
    let t46511 = t829 * t2678;
    let t46519 = t9632 * t828;
    (t45640, t45814, t46240, t46252, t46298, t46320, t46362, t46511, t46519)
}
