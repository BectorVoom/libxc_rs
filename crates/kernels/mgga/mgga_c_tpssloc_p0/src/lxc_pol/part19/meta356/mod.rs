//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta356 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1287;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1288;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1289;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1290;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1291;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1292;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1293;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta356<F: Float>(t2793: F, t10661: F, t913: F, t2836: F, t2792: F, t2842: F, t2844: F, t2880: F, t2897: F, t2904: F, t10701: F, t888: F, t10705: F, t275: F, t2790: F, t2840: F, t10704: F, t41642: F, t41656: F, t41658: F, t41660: F, t41662: F, t41669: F, t41673: F, t41675: F, t41831: F, t41833: F, t41836: F, t41839: F, t41842: F, t41845: F, t41678: F, t41682: F, t41684: F, t41690: F, t41699: F, t41703: F, t41711: F, t41863: F, t41865: F, t41868: F, t41870: F, t41872: F, t41874: F, t41876: F, t41646: F, t41651: F, t41680: F, t41695: F, t41707: F, t41713: F, t41717: F, t41882: F, t41885: F, t41887: F, t41889: F, t41892: F, t41927: F, t41929: F, t41654: F, t41961: F, t41937: F, t41940: F, t41943: F, t41945: F, t41948: F, t41951: F, t41954: F, t41957: F, t41964: F, t41967: F, t41970: F, t41973: F, t893: F, t2843: F, t10619: F, t942: F, t2928: F, t315: F, t2931: F, t10843: F, t923: F, t10744: F, t10750: F, t10760: F, t10765: F, t10771: F, t10825: F, t2861: F, t2881: F, t2886: F, t2888: F, t2907: F, t41827: F, t41987: F, t932: F, t933: F, t952: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t41995, t41998, t42002, t42005, t42011, t42020, t42023) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1287::<F>(t2793, t10661, t913, t2836, t2792, t2842, t2844, t2880, t2897, t2904, t10701, t888);
        let (t42025, t42031, t42046) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1288::<F>(t10705, t42023, t275, t2790, t2840, t10704, t41995, t41642, t41656, t41658, t41660, t41662, t41669, t41673, t41675, t41831, t41833, t41836, t41839, t41842, t41845);
        let t42061 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1289::<F>(t41678, t41682, t41684, t41690, t41699, t41703, t41711, t41863, t41865, t41868, t41870, t41872, t41874, t41876);
        let t42077 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1290::<F>(t41646, t41651, t41680, t41695, t41707, t41713, t41717, t41882, t41885, t41887, t41889, t41892, t41927, t41929);
        let t42092 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1291::<F>(t41654, t41961, t41937, t41940, t41943, t41945, t41948, t41951, t41954, t41957, t41964, t41967, t41970, t41973);
        let (t42097, t42105, t42106) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1292::<F>(t42046, t42061, t42077, t42092, t893, t913, t2840, t275, t2843, t41995, t10619, t942);
        let (t42110, t42113, t42122) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1293::<F>(t2928, t315, t2931, t10843, t923, t10744, t10750, t10760, t10765, t10771, t10825, t2861, t2881, t2886, t2888, t2907, t41827, t41987, t41998, t42002, t42005, t42011, t42020, t42025, t42031, t42097, t42105, t42106, t932, t933, t952);
    (t41995, t41998, t42002, t42005, t42025, t42031, t42097, t42105, t42110, t42113, t42122)
}
