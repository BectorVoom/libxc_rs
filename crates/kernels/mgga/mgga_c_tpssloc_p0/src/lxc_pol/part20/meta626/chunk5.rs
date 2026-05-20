//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2261/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2261<F: Float>(t12984: F, t12998: F, t2553: F, t686: F, t12990: F, t13012: F, t12994: F, t213: F, t221: F, t13196: F, t776: F, t13004: F, t782: F) -> (F, F, F, F, F, F) {
    let t46828 = t12998 * t686 * t12984 * t2553;
    let t46830 = t13012 * t12990;
    let t46836 = t13012 * t12994;
    let t46838 = t221 * t213;
    let t46839 = t13196 * t776;
    let t46843 = t782 * t13004;
    (t46828, t46830, t46836, t46838, t46839, t46843)
}
