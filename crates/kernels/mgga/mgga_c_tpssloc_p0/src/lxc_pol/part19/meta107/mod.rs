//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta107 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk585;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk586;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk587;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk588;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk589;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk590;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk591;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta107<F: Float>(t3036: F, t368: F, t1015: F, t3033: F, t1022: F, t360: F, t1021: F, t248: F, t1030: F, t372: F, t364: F, t354: F, t1043: F, t121: F, t884: F, t1041: F, t1044: F, t2780: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3037, t3038, t3039) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk585::<F>(t3036, t368, t1015, t3033);
        let t3040 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk586::<F>(t1022);
        let t3041 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk587::<F>(t3040, t360);
        let t3043 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk588::<F>(t1021, t248, t3041);
        let (t3047, t3048) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk589::<F>(t1030, t372, t364, t354);
        let t3051 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk590::<F>(t1043, t121);
        let t3053 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk591::<F>(t248, t3051, t884);
        let (t3054, t3057) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk592::<F>(t1041, t3053, t1044, t248, t2780);
    (t3037, t3038, t3039, t3040, t3041, t3043, t3047, t3048, t3051, t3053, t3054, t3057)
}
