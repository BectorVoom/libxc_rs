//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1180/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1180<F: Float>(t152: F, t185: F, t75836: F, t46125: F, t46130: F, t46132: F, t46134: F, t5398: F) -> (F, F, F, F, F, F) {
    let t75839 = 24.0 * t75836 * t152 * t185;
    let t75840 = 0.14035736694323150897e2 * t46125;
    let t75844 = 0.20779030926817756511e3 * t46130;
    let t75845 = 0.1301229756036208781e0 * t46132;
    let t75846 = 0.19263893255070628431e1 * t46134;
    let t75847 = t5398 * t5398;
    (t75839, t75840, t75844, t75845, t75846, t75847)
}
