//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2203;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta660<F: Float>(t16558: F, t707: F, t751: F, t16586: F, t9929: F, t16579: F, t172: F, t763: F, t67: F, t758: F, t16957: F, t41011: F, t212: F, t5544: F, t12998: F, t686: F, t776: F, t13012: F, t16798: F, t16773: F, t46843: F, t16777: F, t5527: F, t46799: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t59037, t59039, t59045, t59048, t59100) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2203::<F>(t16558, t707, t751, t16586, t9929, t16579, t172, t763, t67, t758, t16957, t41011);
        let (t59135, t59138, t59140, t59154, t59156, t59162, t59165) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2204::<F>(t212, t5544, t12998, t686, t776, t13012, t16798, t16773, t46843, t16777, t5527, t46799);
    (t59037, t59039, t59045, t59048, t59100, t59135, t59138, t59140, t59154, t59156, t59162, t59165)
}
