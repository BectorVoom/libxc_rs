//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta350 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1272;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1273;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1274;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1275;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1276;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta350<F: Float>(t2904: F, t41733: F, t951: F, t959: F, t41654: F, t41642: F, t41646: F, t41651: F, t41656: F, t41658: F, t41660: F, t41662: F, t41669: F, t41673: F, t41675: F, t41678: F, t41680: F, t41682: F, t41684: F, t41690: F, t41695: F, t41699: F, t41703: F, t41707: F, t41711: F, t41713: F, t41717: F, t324: F, t10603: F, t2932: F, t10717: F, t10720: F, t10724: F, t10734: F, t10740: F, t10747: F, t10753: F, t10756: F, t10765: F, t10771: F, t10825: F, t10828: F, t14259: F, t2880: F, t2889: F, t2905: F, t2924: F, t2930: F, t2933: F, t41620: F, t41622: F, t41625: F, t41627: F, t41635: F, t41639: F, t41722: F, t950: F, t2794: F, t2836: F, t2842: F, t2784: F, t2791: F, t2897: F, t2929: F, t10629: F, t938: F, t2903: F, t2928: F, t315: F, t2906: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t41737, t41749) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1272::<F>(t2904, t41733, t951, t959, t41654, t41642, t41646, t41651, t41656, t41658, t41660, t41662, t41669, t41673, t41675);
        let t41762 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1273::<F>(t41678, t41680, t41682, t41684, t41690, t41695, t41699, t41703, t41707, t41711, t41713, t41717);
        let (t41764, t41790) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1274::<F>(t324, t41749, t41762, t10603, t2932, t10717, t10720, t10724, t10734, t10740, t10747, t10753, t10756, t10765, t10771, t10825, t10828, t14259, t2880, t2889, t2905, t2924, t2930, t2933, t41620, t41622, t41625, t41627, t41635, t41639, t41722, t950);
        let (t41804, t41813, t41816, t41821, t41825, t41826) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1275::<F>(t2794, t2836, t2842, t2784, t2791, t2897, t2929, t10629, t938, t2903, t2928, t315);
        let t41827 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1276::<F>(t2906);
    (t41737, t41764, t41790, t41804, t41813, t41816, t41821, t41825, t41826, t41827)
}
