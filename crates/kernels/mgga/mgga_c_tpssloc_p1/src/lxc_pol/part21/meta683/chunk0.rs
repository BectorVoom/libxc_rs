//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2495/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2495<F: Float>(t41189: F, t4134: F, t118: F, t12971: F, t2576: F, t794: F, t13025: F, t9546: F, t13017: F, t2563: F, t1489: F, t41083: F) -> (F, F, F, F, F) {
    let t46772 = t41189 * t4134;
    let t46780 = t2576 * t118 * t794 * t12971;
    let t46782 = t9546 * t13025;
    let t46788 = t2563 * t13017;
    let t46790 = t41083 * t1489;
    (t46772, t46780, t46782, t46788, t46790)
}
