//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2067;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2068;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta477<F: Float>(t1842: F, t3911: F, t3887: F, t3888: F, t12021: F, t12033: F, t1375: F, t1386: F, t16453: F, t16458: F, t16460: F, t16463: F, t16465: F, t16468: F, t1843: F, t3758: F, t3882: F, t3889: F, t5215: F, t5326: F, t5354: F, t568: F, t16451: F, t3734: F, t571: F, t1390: F, t5356: F, t12127: F, t12133: F, t12141: F, t12466: F, t1297: F, t1307: F, t15983: F, t15985: F, t15987: F, t15988: F, t16018: F, t16165: F, t16166: F, t16167: F, t16168: F, t16171: F, t16172: F, t1799: F, t193: F, t3918: F, t533: F, t9853: F, t9859: F, t15903: F, t15929: F, t15981: F, t113: F, t1266: F, t1271: F, t12724: F, t12728: F, t12835: F, t12841: F, t1393: F, t15857: F, t1774: F, t1778: F, t2312: F, t2314: F, t2320: F, t3929: F, t4026: F, t4037: F, t4077: F, t510: F, t5107: F, t5118: F, t513: F, t5361: F, t650: F, t652: F) -> (F, F, F, F, F, F, F) {
        let (t16471, t16475, t16485) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2067::<F>(t1842, t3911, t3887, t3888, t12021, t12033, t1375, t1386, t16453, t16458, t16460, t16463, t16465, t16468, t1843, t3758, t3882, t3889, t5215, t5326, t5354, t568);
        let (t16486, t16490, t16497, t16501) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2068::<F>(t16451, t16485, t3734, t571, t1390, t5356, t12127, t12133, t12141, t12466, t1297, t1307, t15983, t15985, t15987, t15988, t16018, t16165, t16166, t16167, t16168, t16171, t16172, t1799, t193, t3918, t533, t9853, t9859);
        let (t16503, t16505) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2069::<F>(t15903, t15929, t15981, t16501, t113, t1266, t1271, t12724, t12728, t12835, t12841, t1393, t15857, t1774, t1778, t2312, t2314, t2320, t3929, t4026, t4037, t4077, t510, t5107, t5118, t513, t5361, t650, t652);
    (t16471, t16475, t16486, t16490, t16497, t16503, t16505)
}
