//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta840 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3013;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3014;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3015;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3016;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3017;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3018;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3019;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3020;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3021;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3022;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3023;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta840<F: Float>(t1058: F, t1060: F, t11051: F, t14526: F, t14600: F, t14618: F, t1615: F, t18086: F, t18099: F, t18155: F, t23508: F, t3040: F, t3180: F, t3197: F, t3200: F, t3201: F, t360: F, t43503: F, t43515: F, t43516: F, t43576: F, t43577: F, t4594: F, t4649: F, t4674: F, t4684: F, t4685: F, t50465: F, t50509: F, t50516: F, t50592: F, t5928: F, t5937: F, t62925: F, t1022: F, t11034: F, t11046: F, t11054: F, t11059: F, t11065: F, t11066: F, t14577: F, t14587: F, t14630: F, t14651: F, t18047: F, t18080: F, t18093: F, t18107: F, t18121: F, t18162: F, t3120: F, t3186: F, t3193: F, t43480: F, t4669: F, t4677: F, t4681: F, t5929: F, t5932: F, t5936: F, t1049: F, t14488: F, t14578: F, t14606: F, t14622: F, t14640: F, t14645: F, t1610: F, t1625: F, t1630: F, t17959: F, t18103: F, t18161: F, t381: F, t4657: F, t47841: F, t50535: F, t5914: F, t62757: F, t1003: F, t11037: F, t13940: F, t14608: F, t14615: F, t14648: F, t1629: F, t1632: F, t18088: F, t18117: F, t18129: F, t18150: F, t3188: F, t353: F, t383: F, t43536: F, t43558: F, t4673: F, t5939: F, t62914: F, t62984: F, t6739: F, t3185: F, t61734: F, t1063: F, t14572: F, t14631: F, t14654: F, t17671: F, t17876: F, t18081: F, t18108: F, t3076: F, t3189: F, t3204: F, t384: F, t4615: F, t4691: F, t47853: F, t50508: F, t5903: F, t5941: F, t62604: F, t18053: F, t225: F, t4693: F, t10160: F, t1052: F, t1055: F, t1066: F, t11010: F, t14529: F, t14552: F, t14555: F, t14659: F, t17588: F, t17875: F, t18166: F, t3020: F, t3026: F, t3174: F, t3207: F, t349: F, t388: F, t4557: F, t4665: F, t4694: F, t5920: F, t61646: F, t62953: F, t62988: F, t63022: F, t990: F, t1070: F, t193: F, t336: F, t60741: F, t60744: F, t60748: F, t60750: F, t60752: F, t60787: F, t60966: F, t60970: F, t61010: F, t61048: F, t61643: F, t62742: F, t62744: F, t62748: F, t62750: F, t62753: F, t25: F, t265: F, t394: F, t59618: F, t60840: F, t60878: F, t60904: F, t60909: F, t60924: F, t60939: F, t60962: F, t1074: F, t12606: F, t13493: F, t1408: F, t1409: F, t14675: F, t1642: F, t16557: F, t16558: F, t17133: F, t18176: F, t2249: F, t2250: F, t2756: F, t3220: F, t396: F, t3966: F, t40: F, t4705: F, t47676: F, t5397: F, t5398: F, t55677: F, t5669: F, t5955: F, t59627: F, t59629: F, t59631: F, t606: F, t607: F, t873: F, dens_threshold: F, rho0: F, zeta_threshold: F, t15051: F, t51667: F, t4857: F, t4781: F, t1118: F, t3264: F, t18238: F, t690: F, t3247: F, t1088: F, t123: F, t18236: F, t18231: F, t2244: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t63058 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3013::<F>(t1058, t1060, t11051, t14526, t14600, t14618, t1615, t18086, t18099, t18155, t23508, t3040, t3180, t3197, t3200, t3201, t360, t43503, t43515, t43516, t43576, t43577, t4594, t4649, t4674, t4684, t4685, t50465, t50509, t50516, t50592, t5928, t5937, t62925);
        let t63095 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3014::<F>(t1022, t1058, t1060, t11034, t11046, t11054, t11059, t11065, t11066, t14577, t14587, t14630, t14651, t18047, t18080, t18086, t18093, t18107, t18121, t18162, t3120, t3180, t3186, t3193, t3200, t43480, t4669, t4677, t4681, t5928, t5929, t5932, t5936);
        let t63133 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3015::<F>(t1049, t1058, t1060, t11065, t14488, t14578, t14606, t14622, t14640, t14645, t1610, t1625, t1630, t17959, t18080, t18103, t18161, t3120, t3200, t381, t4649, t4657, t4669, t4684, t47841, t50535, t5914, t5932, t62757);
        let t63168 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3016::<F>(t1003, t11037, t11046, t11059, t13940, t14488, t14608, t14615, t14618, t14648, t1629, t1632, t18088, t18117, t18129, t18150, t3120, t3186, t3188, t3200, t353, t360, t383, t43536, t43558, t4673, t4684, t5928, t5939, t62914, t62984, t6739);
        let t63198 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3017::<F>(t3185, t61734, t1063, t11037, t14572, t14618, t14622, t14631, t14654, t17671, t17876, t18081, t18108, t18150, t3076, t3189, t3200, t3204, t384, t4615, t4649, t4669, t4684, t4691, t47853, t50508, t50509, t5903, t5936, t5941, t62604);
        let t63235 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3018::<F>(t18053, t225, t4693, t10160, t1049, t1052, t1055, t1066, t11010, t14529, t14552, t14555, t14659, t17588, t17875, t18047, t18166, t3020, t3026, t3174, t3207, t349, t388, t4557, t4665, t4694, t5914, t5920, t61646, t62914, t62953, t62988, t63022, t63058, t63095, t63133, t63168, t63198, t990);
        let t63241 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3019::<F>(t1070, t193, t336, t60741, t60744, t60748, t60750, t60752, t60787, t60966, t60970, t61010, t61048, t61643, t62742, t62744, t62748, t62750, t62753, t63235);
        let t63261 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3020::<F>(t25, t265, t394, t59618, t60840, t60878, t60904, t60909, t60924, t60939, t60962, t63241, t1074, t12606, t13493, t1408, t1409, t14675, t1642, t16557, t16558, t17133, t18176, t2249, t2250, t2756, t3220, t396, t3966, t40, t4705, t47676, t5397, t5398, t55677, t5669, t5955, t59627, t59629, t59631, t606, t607, t873, dens_threshold, rho0, zeta_threshold);
        let (t63280, t63283, t63287, t63290, t63291) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3021::<F>(t15051, t51667, t4857, t4781, t1118, t3264, t18238, t690);
        let (t63294, t63296) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3022::<F>(t16558, t3247, t607, t1088, t123);
        let (t63298, t63300) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3023::<F>(t18236, t2250, t1088, t123);
        let (t63302, t63304) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3024::<F>(t18231, t2244, t1088, t123);
    (t63261, t63280, t63283, t63287, t63290, t63291, t63294, t63296, t63298, t63300, t63302, t63304)
}
