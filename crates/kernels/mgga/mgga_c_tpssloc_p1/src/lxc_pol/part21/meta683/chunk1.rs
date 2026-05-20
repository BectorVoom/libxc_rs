//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2496/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2496<F: Float>(t2559: F, t4126: F, t4130: F, t12997: F, t13000: F, t2566: F, t67: F, t792: F, t9558: F, t12984: F, t2379: F, t686: F) -> (F, F, F, F) {
    let t46793 = t2559 * t4126 * t4130;
    let t46796 = t2566 * t12997 * t13000;
    let t46799 = t792 * t9558 * t67;
    let t46802 = t46799 * t686 * t12984 * t2379;
    (t46793, t46796, t46799, t46802)
}
