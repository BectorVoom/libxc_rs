//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2103/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2103<F: Float>(t4179: F, t820: F, t812: F, t836: F, t9972: F, t12985: F, t9577: F, t212: F, t4119: F, t2586: F, t9523: F, t4138: F, t9541: F) -> (F, F, F, F, F) {
    let t46692 = t4179 * t820;
    let t46741 = t812 * t9972 * t836;
    let t46764 = t9577 * t12985;
    let t46766 = t212 * t4119;
    let t46768 = t2586 * t9523 * t46766;
    let t46769 = F::cast_from(0.49999999999999999998e-2_f64) * t46768;
    let t46770 = t9541 * t4138;
    (t46692, t46741, t46764, t46769, t46770)
}
