//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2105/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2105<F: Float>(t67: F, t792: F, t9558: F, t133: F, t1484: F, t41214: F, t6600: F, t213: F, t221: F, t13004: F, t782: F, t131: F, t205: F, t41160: F) -> (F, F, F, F, F) {
    let t46799 = t792 * t9558 * t67;
    let t46806 = t41214 * t133 * t6600 * t1484;
    let t46838 = t221 * t213;
    let t46843 = t782 * t13004;
    let t46847 = t205 * t41160 * t131;
    (t46799, t46806, t46838, t46843, t46847)
}
