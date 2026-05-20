//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta713 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2312;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2313;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2314;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2315;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta713<F: Float>(t40804: F, t40806: F, t40790: F, t40793: F, t40797: F, t40799: F, t40801: F, t40803: F, t46311: F, t67214: F, t67215: F, t12939: F, t16716: F, t3966: F, t40: F, t12908: F, t20749: F, t12923: F, t4194: F, t5398: F, t20800: F, t262: F, t10143: F, t20778: F, t13115: F, t16586: F, t12950: F, t1430: F, t16558: F, t16637: F, t17635: F, t20217: F, t20234: F, t2291: F, t4104: F, t607: F, t67060: F, t75: F, t767: F, zeta_threshold: F, t52: F, t12961: F, t1431: F, t16649: F, t2298: F, t4111: F, t771: F, t78: F, t12895: F, t193: F, t20756: F, t2522: F, t39549: F, t39563: F, t4314: F, t5527: F, t766: F, t776: F, t868: F, t870: F, t21038: F, t225: F, t10110: F, t1527: F, t1528: F, t17049: F, t17057: F, t17064: F, t17092: F, t21013: F, t21049: F, t21054: F, t259: F, t2597: F, t2713: F, t2718: F, t40890: F, t4147: F, t4273: F, t4300: F, t5636: F, t5657: F, t59466: F, t59537: F, t798: F, t855: F, t865: F, t866: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t67216, t67217, t67218, t67226) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2312::<F>(t40804, t40806, t40790, t40793, t40797, t40799, t40801, t40803, t46311, t67214, t67215, t12939, t16716, t3966);
        let (t67228, t67231, t67235, t67239, t67244, t67262) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2313::<F>(t40, t12908, t20749, t12923, t4194, t5398, t20800, t262, t10143, t20778, t13115, t16586, t12950, t1430, t16558, t16637, t17635, t20217, t20234, t2291, t3966, t4104, t607, t67060, t75, t767, zeta_threshold);
        let t67280 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2314::<F>(t52, t12961, t1431, t16558, t16649, t17635, t20217, t20234, t2298, t3966, t4111, t5398, t607, t67060, t771, t78, zeta_threshold);
        let (t67282, t67286) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2315::<F>(t67262, t67280, t12895, t193, t20756, t2522, t39549, t39563, t4314, t5527, t67226, t67228, t67231, t67235, t67239, t67244, t766, t776, t868, t870);
        let t67322 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2316::<F>(t21038, t225, t10110, t1527, t1528, t17049, t17057, t17064, t17092, t21013, t21049, t21054, t259, t2597, t2713, t2718, t40890, t4147, t4273, t4300, t5636, t5657, t59466, t59537, t798, t855, t865, t866);
    (t67216, t67217, t67218, t67226, t67228, t67231, t67244, t67282, t67286, t67322)
}
