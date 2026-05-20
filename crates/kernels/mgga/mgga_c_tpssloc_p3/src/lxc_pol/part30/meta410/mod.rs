//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta410 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1550;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1551;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1552;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1553;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1554;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1555;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1556;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta410<F: Float>(t18047: F, t383: F, t4684: F, t5932: F, t3188: F, t4649: F, t1629: F, t4673: F, t1625: F, t1060: F, t1022: F, t5914: F, t17959: F, t381: F, t1003: F, t1058: F, t1063: F, t14608: F, t1610: F, t1632: F, t17876: F, t3180: F, t3186: F, t3200: F, t353: F, t384: F, t4615: F, t4669: F, t4678: F, t4681: F, t4685: F, t4689: F, t4691: F, t5903: F, t5933: F, t5941: F, t18124: F, t1055: F, t1052: F, t1066: F, t14529: F, t14545: F, t14552: F, t14555: F, t1635: F, t18053: F, t18057: F, t18059: F, t18062: F, t18065: F, t18071: F, t18074: F, t388: F, t4660: F, t4665: F, t18050: F, t1068: F, t1070: F, t17194: F, t17197: F, t17198: F, t17202: F, t17209: F, t17301: F, t17303: F, t17306: F, t17372: F, t17374: F, t17377: F, t17379: F, t17425: F, t17427: F, t17561: F, t17563: F, t17568: F, t193: F, t336: F, t4696: F, t4700: F, t4701: F, t17490: F, t17504: F, t17506: F, t17509: F, t17512: F, t17515: F, t17519: F, t17523: F, t17526: F, t17530: F, t17929: F, t17932: F, t17936: F, t17940: F, t17942: F, t17944: F, t17946: F, t17950: F, t17953: F, t17957: F, t25: F, t265: F, t394: F, t17133: F, t1074: F, t1408: F, t1409: F, t1642: F, t16557: F, t16558: F, t17141: F, t396: F, t3966: F, t40: F, t4324: F, t4705: F, t5397: F, t5398: F, t5669: F, t5955: F, t606: F, t607: F, t873: F, dens_threshold: F, rho0: F, zeta_threshold: F, t5972: F, t690: F, t11147: F, t5392: F, t11145: F, t123: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18129, t18131, t18138, t18139, t18142, t18150, t18151, t18154) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1550::<F>(t18047, t383, t4684, t5932, t3188, t4649, t1629, t4673, t1625, t1060, t1022, t5914);
        let (t18161, t18164) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1551::<F>(t1060, t18154, t17959, t381, t1003, t1058, t1063, t14608, t1610, t1632, t17876, t18129, t18131, t18139, t18142, t18151, t3180, t3186, t3200, t353, t384, t4615, t4669, t4678, t4681, t4685, t4689, t4691, t5903, t5933, t5941);
        let (t18165, t18166, t18168) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1552::<F>(t18124, t18164, t1055, t1052, t1066, t14529, t14545, t14552, t14555, t1635, t18053, t18057, t18059, t18062, t18065, t18071, t18074, t388, t4660, t4665);
        let (t18169, t18173) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1553::<F>(t18050, t18168, t1068, t1070, t17194, t17197, t17198, t17202, t17209, t17301, t17303, t17306, t17372, t17374, t17377, t17379, t17425, t17427, t17561, t17563, t17568, t193, t336, t4696, t4700, t4701);
        let t18174 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1554::<F>(t17490, t17504, t17506, t17509, t17512, t17515, t17519, t17523, t17526, t17530, t17929, t17932, t17936, t17940, t17942, t17944, t17946, t17950, t17953, t17957);
        let t18188 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1555::<F>(t25, t265, t394, t17133, t18173, t18174, t1074, t1408, t1409, t1642, t16557, t16558, t17141, t396, t3966, t40, t4324, t4705, t5397, t5398, t5669, t5955, t606, t607, t873, dens_threshold, rho0, zeta_threshold);
        let (t18196, t18203) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1556::<F>(t16557, t5972, t690);
        let (t18206, t18208) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1557::<F>(t11147, t5392, t607, t11145, t123);
    (t18138, t18150, t18154, t18161, t18165, t18166, t18169, t18188, t18196, t18203, t18206, t18208)
}
