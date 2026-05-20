//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1294;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1295;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1296;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1297;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1298;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta357<F: Float>(t2853: F, t2885: F, t10523: F, t938: F, t10660: F, t888: F, t10663: F, t10702: F, t2844: F, t41995: F, t10810: F, t919: F, t2859: F, t2884: F, t302: F, t41642: F, t41656: F, t41658: F, t41660: F, t41662: F, t41669: F, t41673: F, t41675: F, t41831: F, t41833: F, t41836: F, t41839: F, t41842: F, t41845: F, t41678: F, t41682: F, t41684: F, t41690: F, t41699: F, t41703: F, t41711: F, t41863: F, t41865: F, t41868: F, t41870: F, t41872: F, t41874: F, t41876: F, t41646: F, t41651: F, t41680: F, t41695: F, t41707: F, t41713: F, t41717: F, t41882: F, t41885: F, t41887: F, t41889: F, t41892: F, t41927: F, t41929: F, t41654: F, t41961: F, t41937: F, t41940: F, t41943: F, t41945: F, t41948: F, t41951: F, t41954: F, t41957: F, t41964: F, t41967: F, t41970: F, t41973: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t42123, t42128, t42145, t42148, t42149) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1294::<F>(t2853, t2885, t10523, t938, t10660, t888, t10663, t10702, t2844, t41995, t10810, t919);
        let (t42154, t42172) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1295::<F>(t2859, t2884, t302, t41642, t41656, t41658, t41660, t41662, t41669, t41673, t41675, t41831, t41833, t41836, t41839, t41842, t41845);
        let t42187 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1296::<F>(t41678, t41682, t41684, t41690, t41699, t41703, t41711, t41863, t41865, t41868, t41870, t41872, t41874, t41876);
        let t42203 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1297::<F>(t41646, t41651, t41680, t41695, t41707, t41713, t41717, t41882, t41885, t41887, t41889, t41892, t41927, t41929);
        let t42218 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1298::<F>(t41654, t41961, t41937, t41940, t41943, t41945, t41948, t41951, t41954, t41957, t41964, t41967, t41970, t41973);
    (t42123, t42128, t42145, t42148, t42149, t42154, t42172, t42187, t42203, t42218)
}
