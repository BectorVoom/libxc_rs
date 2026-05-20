//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2488/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2488<F: Float>(t13036: F, t225: F, t13336: F, t68: F, t1496: F, t41083: F, t4257: F, t9601: F, t13193: F, t2697: F, t13204: F, t2563: F) -> (F, F, F, F, F, F) {
    let t46508 = t13036 * t225;
    let t46528 = t13336 * t68;
    let t46546 = t41083 * t1496;
    let t46549 = t9601 * t4257;
    let t46551 = t2697 * t13193;
    let t46558 = t2563 * t13204;
    (t46508, t46528, t46546, t46549, t46551, t46558)
}
