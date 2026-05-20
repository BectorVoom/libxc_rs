//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta421 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1699;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1700;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1701;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1702;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta421<F: Float>(t3793: F, t3805: F, t5301: F, t3802: F, t5234: F, t3788: F, t836: F, t1336: F, t5252: F, t3777: F, t5245: F, t12419: F, t12420: F, t5249: F, t12215: F, t12335: F, t12340: F, t12346: F, t12356: F, t12358: F, t12366: F, t12386: F, t12388: F, t12395: F, t12429: F, t16366: F, t16370: F, t16379: F, t16383: F, t16387: F, t3803: F, t3809: F, t5246: F, t5303: F, t16253: F, t16319: F, t16361: F, t553: F, t3901: F, t5287: F, t1352: F, t16036: F, t3856: F, t5348: F, t1834: F, t3787: F, t1380: F, t16206: F, t12267: F, t1383: F, t16133: F, t16136: F, t1814: F, t1838: F, t1840: F, t3773: F, t3898: F, t3905: F, t3907: F, t3909: F, t5230: F, t5339: F, t5341: F, t5344: F, t544: F, t16131: F, t1378: F, t225: F, t5319: F, t1372: F, t5210: F, t12030: F, t12444: F, t1375: F, t1386: F, t16022: F, t16028: F, t16030: F, t1843: F, t3758: F, t3889: F, t3912: F, t5215: F, t5321: F, t5354: F, t568: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16391, t16394, t16400, t16401, t16405) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1699::<F>(t3793, t3805, t5301, t3802, t5234, t3788, t836, t1336, t5252, t3777, t5245, t12419, t12420, t5249);
        let t16411 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1700::<F>(t12215, t12335, t12340, t12346, t12356, t12358, t12366, t12386, t12388, t12395, t12429, t16366, t16370, t16379, t16383, t16387, t16391, t16394, t16400, t16401, t16405, t3803, t3809, t5246, t5252, t5303);
        let (t16413, t16414, t16416, t16419, t16423, t16428) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1701::<F>(t16253, t16319, t16361, t16411, t553, t3901, t5287, t1352, t16036, t3856, t5348, t1834, t3787);
        let t16435 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1702::<F>(t16428, t3793, t1380, t16206, t12267, t1336, t1383, t16133, t16136, t16414, t16416, t16419, t16423, t1814, t1838, t1840, t3773, t3777, t3898, t3905, t3907, t3909, t5230, t5234, t5339, t5341, t5344, t544);
        let (t16436, t16437, t16439, t16448, t16451) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1703::<F>(t16131, t16435, t1378, t225, t5319, t1372, t5210, t12030, t12444, t1375, t1386, t16022, t16028, t16030, t1843, t3758, t3889, t3912, t5215, t5321, t5354, t568);
    (t16391, t16405, t16413, t16419, t16436, t16437, t16439, t16448, t16451)
}
