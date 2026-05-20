//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta673 (260520-c91 hierarchical CSE).
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
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2024;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2025;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2026;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2027;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2028;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2029;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2030;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2031;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2032;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2033;
use chunk10::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2034;
use chunk11::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta673<F: Float>(t84533: F, t91305: F, t91312: F, t91314: F, t91323: F, t91346: F, t93720: F, t93722: F, t93731: F, t93736: F, t93742: F, t93743: F, t93745: F, t97378: F, t97380: F, t97382: F, t97387: F, t97389: F, t84536: F, t91383: F, t91394: F, t93753: F, t97394: F, t97398: F, t97400: F, t97402: F, t97404: F, t97407: F, t97410: F, t97412: F, t97414: F, t97416: F, t97419: F, t97423: F, t97427: F, t97431: F, t84555: F, t84558: F, t91398: F, t91400: F, t91406: F, t93762: F, t93763: F, t97435: F, t97437: F, t97439: F, t97444: F, t97447: F, t97450: F, t97453: F, t97456: F, t97459: F, t97461: F, t97463: F, t102647: F, t102663: F, t102679: F, t102694: F, t102705: F, t544: F, t553: F, t6378: F, t7211: F, t90993: F, t91000: F, t91002: F, t93618: F, t97119: F, t97124: F, t97129: F, t97135: F, t97137: F, t97142: F, t97146: F, t97148: F, t97152: F, t97158: F, t97161: F, t102587: F, t1336: F, t1825: F, t19654: F, t19732: F, t24116: F, t27075: F, t27086: F, t27098: F, t29343: F, t29349: F, t3777: F, t5234: F, t5250: F, t5334: F, t6415: F, t6420: F, t7208: F, t84595: F, t84597: F, t91018: F, t91043: F, t91045: F, t93607: F, t97179: F, t97200: F, t1338: F, t29286: F, t2085: F, t6387: F, t1352: F, t16047: F, t16060: F, t19744: F, t19815: F, t27097: F, t27103: F, t29339: F, t29345: F, t5287: F, t5344: F, t6388: F, t7209: F, t7932: F, t84577: F, t91078: F, t91081: F, t93792: F, t93794: F, t97488: F, t97491: F, t97494: F, t102558: F, t102580: F, t102597: F, t102614: F, t102629: F, t1375: F, t1378: F, t27068: F, t27115: F, t29372: F, t3758: F, t5215: F, t5354: F, t84423: F, t90706: F, t93461: F, t93467: F, t97529: F, t97537: F, t97548: F, t16030: F, t20022: F, t20050: F, t20060: F, t2091: F, t2092: F, t26224: F, t27132: F, t29361: F, t3882: F, t3887: F, t5321: F, t5353: F, t56640: F, t7214: F, t7936: F, t7937: F, t90743: F, t93319: F, t93824: F, t97571: F, t97573: F, t97577: F, t97583: F, t97588: F, t97599: F, t97604: F, t97611: F, t97616: F, t1385: F, t16439: F, t1843: F, t20023: F, t20044: F, t26996: F, t27062: F, t29360: F, t6460: F, t7194: F, t7213: F, t7925: F, t93341: F, t97640: F, t97644: F, t97647: F, t225: F, t29290: F, t29293: F, t1386: F, t16022: F, t16460: F, t20026: F, t24082: F, t26990: F, t56434: F, t56596: F, t6461: F, t97626: F, t97705: F, t29287: F, t19647: F, t19648: F, t20029: F, t24095: F, t26989: F, t5210: F, t56607: F, t568: F, t7199: F, t7918: F, t84705: F, t91548: F, t97766: F, t102401: F, t102403: F, t102432: F, t102475: F, t102493: F, t102523: F, t12725: F, t1323: F, t1390: F, t1459: F, t1807: F, t1842: F, t19456: F, t19534: F, t19804: F, t1983: F, t20025: F, t20051: F, t2040: F, t2075: F, t22574: F, t2314: F, t25988: F, t27009: F, t27051: F, t27114: F, t27180: F, t27188: F, t28002: F, t29214: F, t29219: F, t29299: F, t29311: F, t33899: F, t4028: F, t4034: F, t4077: F, t510: F, t5107: F, t5326: F, t533: F, t539: F, t5493: F, t55943: F, t56422: F, t56580: F, t574: F, t6361: F, t6440: F, t652: F, t7050: F, t7156: F, t7191: F, t7687: F, t7801: F, t7806: F, t84400: F, t84659: F, t90521: F, t90617: F, t90659: F, t90663: F, t90670: F, t91496: F, t91531: F, t92090: F, t93286: F, t93316: F, t93337: F, t93344: F, t93350: F, t93353: F, t93388: F, t93404: F, t93407: F, t93452: F, t93873: F, t96683: F, t96893: F, t96896: F, t96900: F, t96905: F, t97503: F, t97509: F, t97524: F, t97527: F, t97619: F, t97624: F, t97652: F, t97658: F, t97664: F, t97724: F, t97729: F, t97732: F, t97740: F, t97750: F) -> F {
        let t102715 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2024::<F>(t84533, t91305, t91312, t91314, t91323, t91346, t93720, t93722, t93731, t93736, t93742, t93743, t93745, t97378, t97380, t97382, t97387, t97389);
        let t102732 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2025::<F>(t84536, t91383, t91394, t93753, t97394, t97398, t97400, t97402, t97404, t97407, t97410, t97412, t97414, t97416, t97419, t97423, t97427, t97431);
        let t102746 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2026::<F>(t84555, t84558, t91398, t91400, t91406, t93762, t93763, t97435, t97437, t97439, t97444, t97447, t97450, t97453, t97456, t97459, t97461, t97463);
        let (t102749, t102765) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2027::<F>(t102647, t102663, t102679, t102694, t102705, t102715, t102732, t102746, t544, t553, t6378, t7211, t90993, t91000, t91002, t93618, t97119, t97124, t97129, t97135, t97137, t97142, t97146, t97148, t97152, t97158, t97161);
        let t102790 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2028::<F>(t102587, t1336, t1825, t19654, t19732, t24116, t27075, t27086, t27098, t29343, t29349, t3777, t5234, t5250, t5334, t6415, t6420, t7208, t84595, t84597, t91018, t91043, t91045, t93607, t97179, t97200);
        let t102822 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2029::<F>(t1338, t29286, t2085, t6387, t1336, t1352, t16047, t16060, t19744, t19815, t27097, t27103, t29339, t29345, t3777, t5234, t5250, t5287, t5334, t5344, t6388, t7209, t7932, t84577, t91078, t91081, t93792, t93794, t97488, t97491, t97494);
        let t102828 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2030::<F>(t102558, t102580, t102597, t102614, t102629, t102765, t102790, t102822, t1375, t1378, t27068, t27115, t29372, t3758, t5215, t5354, t84423, t90706, t93461, t93467, t97529, t97537, t97548);
        let t102861 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2031::<F>(t1375, t16030, t20022, t20050, t20060, t2091, t2092, t26224, t27115, t27132, t29361, t3882, t3887, t5321, t5353, t56640, t7214, t7936, t7937, t90743, t93319, t93824, t97571, t97573, t97577, t97583, t97588, t97599, t97604, t97611, t97616);
        let t102900 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2032::<F>(t1375, t1385, t16439, t1843, t20023, t20044, t26996, t27062, t29360, t3887, t5321, t6460, t7194, t7213, t7214, t7925, t93341, t97640, t97644, t97647);
        let t102936 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2033::<F>(t225, t29290, t29293, t1386, t16022, t16460, t20026, t2092, t24082, t26990, t27062, t5215, t56434, t56596, t6461, t7194, t7925, t7937, t97626, t97705);
        let (t102948, t102972) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2034::<F>(t225, t29287, t16439, t19647, t19648, t20029, t2092, t24095, t26224, t26989, t29361, t3758, t5210, t56607, t568, t6461, t7194, t7199, t7918, t7937, t84705, t91548, t97766);
        let t102988 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2035::<F>(t102401, t102403, t102432, t102475, t102493, t102523, t102749, t102828, t102861, t102900, t102936, t102948, t102972, t12725, t1323, t1375, t1386, t1390, t1459, t16030, t16460, t1807, t1842, t1843, t19456, t19534, t19804, t1983, t20025, t20044, t20051, t2040, t2075, t2085, t2092, t22574, t2314, t24095, t25988, t26224, t26989, t26990, t26996, t27009, t27051, t27068, t27114, t27132, t27180, t27188, t28002, t29214, t29219, t29286, t29299, t29311, t33899, t3758, t3882, t3887, t4028, t4034, t4077, t510, t5107, t5215, t5326, t533, t5354, t539, t5493, t55943, t56422, t56580, t568, t574, t6361, t6440, t652, t7050, t7156, t7191, t7194, t7199, t7687, t7801, t7806, t7925, t84400, t84659, t90521, t90617, t90659, t90663, t90670, t91496, t91531, t92090, t93286, t93316, t93337, t93344, t93350, t93353, t93388, t93404, t93407, t93452, t93873, t96683, t96893, t96896, t96900, t96905, t97503, t97509, t97524, t97527, t97619, t97624, t97652, t97658, t97664, t97724, t97729, t97732, t97740, t97750);
    t102988
}
