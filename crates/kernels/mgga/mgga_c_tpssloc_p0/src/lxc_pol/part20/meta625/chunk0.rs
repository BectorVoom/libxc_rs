//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2249/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2249<F: Float>(t13210: F, t9638: F, t120: F, t13170: F, t2553: F, t828: F, t13231: F, t13258: F, t41107: F, t4250: F, t13244: F, t242: F, t812: F, t841: F) -> (F, F, F, F, F, F, F) {
    let t46595 = t9638 * t13210;
    let t46597 = t120 * t13170;
    let t46606 = t2553 * t828;
    let t46611 = t13258 * t13231;
    let t46616 = t41107 * t4250;
    let t46618 = t13258 * t13244;
    let t46628 = t812 * t841 * t242;
    (t46595, t46597, t46606, t46611, t46616, t46618, t46628)
}
