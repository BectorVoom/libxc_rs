//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta439 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1683;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1684;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1685;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1686;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1687;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta439<F: Float>(t19804: F, t562: F, t1372: F, t6361: F, t225: F, t6435: F, t1323: F, t6434: F, t1385: F, t6439: F, t12021: F, t6362: F, t1375: F, t1386: F, t16022: F, t16460: F, t1843: F, t3758: F, t3882: F, t5215: F, t5326: F, t5354: F, t568: F, t6440: F, t6461: F, t20034: F, t1390: F, t6463: F, t12044: F, t12048: F, t12057: F, t12059: F, t1297: F, t1307: F, t1388: F, t15898: F, t15911: F, t15916: F, t15917: F, t15923: F, t193: F, t19596: F, t19599: F, t19603: F, t19631: F, t19677: F, t3918: F, t5126: F, t5160: F, t5161: F, t533: F, t5356: F, t571: F, t6330: F, t9780: F, t9789: F, t3701: F, t6324: F, t6347: F, t12461: F, t12087: F, t12094: F, t12103: F, t12105: F, t12109: F, t12114: F, t16497: F, t1799: F, t19678: F, t19683: F, t19684: F, t19685: F, t19686: F, t19687: F, t9793: F, t9797: F, t9820: F, t9824: F, t3919: F, t12116: F, t12118: F, t12123: F, t12130: F, t12133: F, t12141: F, t15976: F, t16171: F, t19689: F, t19690: F, t19691: F, t19693: F, t19694: F, t19695: F, t19696: F, t19697: F, t19698: F, t9853: F, t9859: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20038, t20040, t20044, t20048, t20050, t20051, t20060) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1683::<F>(t19804, t562, t1372, t6361, t225, t6435, t1323, t6434, t1385, t6439, t12021, t6362);
        let t20062 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1684::<F>(t1375, t1386, t16022, t16460, t1843, t20038, t20040, t20044, t20048, t20051, t20060, t3758, t3882, t5215, t5326, t5354, t568, t6440, t6461);
        let (t20063, t20075) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1685::<F>(t20034, t20062, t1390, t6463, t12044, t12048, t12057, t12059, t1297, t1307, t1388, t15898, t15911, t15916, t15917, t15923, t193, t19596, t19599, t19603, t19631, t19677, t3918, t5126, t5160, t5161, t533, t5356, t571, t6330, t9780, t9789);
        let (t20085, t20092) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1686::<F>(t3701, t6324, t571, t6347, t12461, t12087, t12094, t12103, t12105, t12109, t12114, t1307, t1388, t16497, t1799, t19678, t19683, t19684, t19685, t19686, t19687, t3918, t5126, t5160, t9793, t9797, t9820, t9824);
        let t20096 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1687::<F>(t3919, t6330, t12116, t12118, t12123, t12130, t12133, t12141, t15976, t16171, t19689, t19690, t19691, t19693, t19694, t19695, t19696, t19697, t19698, t5126, t9853, t9859);
    (t20038, t20040, t20044, t20048, t20050, t20051, t20060, t20063, t20075, t20085, t20092, t20096)
}
