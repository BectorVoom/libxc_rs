//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2246/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2246<F: Float>(t10109: F, t1527: F, t13036: F, t225: F, t2678: F, t829: F, t828: F, t9632: F, t1519: F, t9971: F, t13336: F, t68: F) -> (F, F, F, F, F, F) {
    let t46488 = t10109 * t1527;
    let t46508 = t13036 * t225;
    let t46511 = t829 * t2678;
    let t46519 = t9632 * t828;
    let t46524 = t9971 * t1519;
    let t46528 = t13336 * t68;
    (t46488, t46508, t46511, t46519, t46524, t46528)
}
