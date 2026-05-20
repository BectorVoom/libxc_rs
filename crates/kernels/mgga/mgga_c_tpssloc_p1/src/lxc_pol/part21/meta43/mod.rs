//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta43 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk318;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk319;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk320;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk321;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk322;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk323;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk324;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk325;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk326;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk327;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta43<F: Float>(t218: F, t852: F, t225: F, t253: F, t257: F, t68: F, t252: F, t814: F, t829: F, t235: F, t226: F, t255: F, t808: F, t812: F, t259: F, t799: F, t261: F, t193: F, t202: F, t680: F, t705: F, t710: F, t719: F, t752: F, t755: F, t760: F, t765: F, t766: F, t776: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t853, t855) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk318::<F>(t218, t852, t225, t253);
        let (t856, t858) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk319::<F>(t257, t68);
        let t860 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk320::<F>(t252, t814);
        let t861 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk321::<F>(t829, t860);
        let t863 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk322::<F>(t235, t852);
        let t865 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk323::<F>(t226, t255, t808, t812, t861, t863);
        let t866 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk324::<F>(t858, t865);
        let t868 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk325::<F>(t259, t799, t853, t855, t866);
        let t870 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk326::<F>(t261);
        let t873 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk327::<F>(t193, t202, t680, t705, t710, t719, t752, t755, t760, t765, t766, t776, t868, t870);
    (t853, t855, t856, t858, t860, t861, t863, t865, t866, t868, t870, t873)
}
