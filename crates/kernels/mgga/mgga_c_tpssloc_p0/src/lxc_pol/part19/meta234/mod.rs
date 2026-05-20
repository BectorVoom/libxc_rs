//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta234 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk947;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk948;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk949;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk950;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk951;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk952;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta234<F: Float>(t11129: F, t1156: F, t3400: F, t1164: F, t268: F, t405: F, t6546: F, t1091: F, t2394: F, t3244: F, t690: F, t3249: F, t3253: F, t154: F, t3584: F, t3241: F, t636: F, t9288: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11131, t11133, t11135) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk947::<F>(t11129, t1156, t3400, t1164, t268, t405, t6546);
        let (t11136, t11137) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk948::<F>(t11135, t1091, t2394);
        let t11139 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk949::<F>(t3244, t690);
        let t11141 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk950::<F>(t3249, t690);
        let t11143 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk951::<F>(t3253, t690);
        let (t11145, t11147) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk952::<F>(t154, t3584, t3241, t636);
        let t11148 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk953::<F>(t11147, t9288);
    (t11131, t11133, t11135, t11136, t11137, t11139, t11141, t11143, t11145, t11147, t11148)
}
