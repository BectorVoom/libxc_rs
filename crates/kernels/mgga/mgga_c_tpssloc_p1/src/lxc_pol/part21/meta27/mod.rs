//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta27 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk203;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk204;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk205;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk206;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk207;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk208;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk209;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk210;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta27<F: Float>(t531: F, t532: F, t154: F, t205: F, t215: F, t131: F, t221: F, t225: F, t144: F, t523: F, t525: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t533 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk203::<F>(t531, t532);
        let (t534, t535) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk204::<F>(t532, t154);
        let t539 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk205::<F>(t205, t215, t535);
        let t541 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk206::<F>(t131, t534, t221);
        let t544 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk207::<F>(t225, t539);
        let t546 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk208::<F>(t144, t225, t523, t525);
        let (t547, t548) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk209::<F>(t533, t68);
        let t550 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk210::<F>(t546, t548);
    (t533, t534, t535, t539, t541, t544, t546, t547, t548, t550)
}
