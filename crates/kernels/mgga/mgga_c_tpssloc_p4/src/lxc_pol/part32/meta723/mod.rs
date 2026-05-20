//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta723 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2306;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2307;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2308;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2309;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2310;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2311;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2312;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2313;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2314;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2315;
use chunk10::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2316;
use chunk11::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2317;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta723<F: Float>(t24667: F, t6252: F, t1653: F, t8039: F, t85822: F, t6224: F, t7348: F, t24574: F, t29741: F, t29614: F, t7327: F, t103683: F, t24589: F, t24833: F, t24858: F, t27507: F, t27520: F, t27536: F, t27537: F, t27562: F, t29781: F, t3624: F, t3625: F, t5975: F, t7283: F, t7362: F, t7373: F, t7377: F, t8066: F, t8073: F, t85820: F, t86037: F, t86102: F, t94966: F, t95803: F, t95813: F, t29702: F, t103515: F, t11907: F, t1216: F, t1716: F, t18525: F, t18946: F, t19203: F, t2148: F, t24812: F, t27489: F, t27490: F, t27492: F, t27496: F, t27510: F, t27540: F, t27732: F, t29709: F, t3610: F, t6140: F, t7381: F, t8082: F, t94858: F, t95033: F, t6260: F, t24660: F, t1215: F, t5392: F, t7376: F, t27736: F, t7999: F, t103218: F, t11904: F, t24849: F, t27406: F, t27455: F, t27525: F, t27532: F, t27733: F, t29678: F, t29719: F, t29723: F, t4930: F, t5068: F, t7365: F, t7382: F, t8077: F, t86039: F, t86076: F, t86077: F, t94837: F, t95048: F, t24826: F, t29716: F, t103615: F, t24745: F, t27453: F, t27460: F, t27481: F, t27484: F, t27498: F, t3612: F, t7368: F, t85918: F, t85941: F, t85952: F, t85963: F, t94874: F, t95069: F, t8070: F, t94490: F, t86036: F, t95760: F, t1409: F, t1734: F, t19138: F, t24851: F, t27502: F, t29735: F, t3966: F, t5011: F, t6256: F, t86015: F, t86116: F, t95098: F, t95114: F, t95197: F, t95201: F, t95761: F, t8074: F, t94909: F, t29745: F, t29705: F, t477: F, t6238: F, t1090: F, t17635: F, t19145: F, t24820: F, t24821: F, t27549: F, t27550: F, t27551: F, t29753: F, t85863: F, t85986: F, t86000: F, t95125: F, t95134: F, t95136: F, t1186: F, t11881: F, t1751: F, t19165: F, t24814: F, t24815: F, t27517: F, t27533: F, t29708: F, t29711: F, t29726: F, t3242: F, t3961: F, t5079: F, t6146: F, t94395: F, t95092: F, t95163: F, t95165: F, t95192: F, t95213: F, t29777: F, t7359: F, t29759: F, t1244: F, t1246: F, t15245: F, t19120: F, t19169: F, t2121: F, t2147: F, t24776: F, t27546: F, t27574: F, t27721: F, t462: F, t5971: F, t7375: F, t95714: F, t95722: F, t29790: F, t29763: F, t8067: F, t11914: F, t1201: F, t18572: F, t18940: F, t19153: F, t2144: F, t2152: F, t27466: F, t27474: F, t27478: F, t29773: F, t4733: F, t5064: F, t8054: F, t95726: F, t11888: F, t15032: F, t1729: F, t19156: F, t19179: F, t27465: F, t27516: F, t27722: F, t29664: F, t29712: F, t3604: F, t4964: F, t6168: F, t7389: F, t8083: F, t8085: F, t95747: F, t95751: F, t95758: F, t27604: F, t4993: F, t19095: F, t24733: F, t1207: F, t19024: F, t7337: F, t19046: F, t7338: F, t6169: F, t7344: F, t1218: F, t1232: F, t1737: F, t1748: F, t18307: F, t18943: F, t18959: F, t24716: F, t6221: F, t7339: F, t7345: F, t86164: F, t95242: F, t95244: F, t95276: F, t95440: F, t18375: F, t27599: F, t4997: F, t18360: F, t18364: F, t18397: F, t18401: F, t19002: F, t19016: F, t24741: F, t27617: F, t4950: F, t4980: F, t4984: F, t5014: F, t5030: F, t86324: F, t86327: F, t95566: F, t95623: F, t95627: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t103707, t103733) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2306::<F>(t24667, t6252, t1653, t8039, t85822, t6224, t7348, t24574, t29741, t29614, t7327, t103683, t24589, t24833, t24858, t27507, t27520, t27536, t27537, t27562, t29781, t3624, t3625, t5975, t7283, t7362, t7373, t7377, t8066, t8073, t85820, t86037, t86102, t94966, t95803, t95813);
        let t103766 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2307::<F>(t24574, t29702, t103515, t11907, t1216, t1716, t18525, t18946, t19203, t2148, t24812, t27489, t27490, t27492, t27496, t27507, t27510, t27536, t27540, t27732, t29709, t3610, t6140, t7283, t7373, t7381, t8082, t94858, t95033);
        let (t103779, t103801) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2308::<F>(t6260, t7327, t24660, t6252, t1215, t5392, t7376, t27736, t7999, t103218, t11904, t24849, t27406, t27455, t27525, t27532, t27733, t29678, t29719, t29723, t3610, t4930, t5068, t7283, t7365, t7382, t8077, t86037, t86039, t86076, t86077, t94837, t95048);
        let t103829 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2309::<F>(t24826, t29716, t103218, t103615, t103707, t1216, t24745, t27406, t27453, t27460, t27481, t27484, t27498, t3610, t3612, t7283, t7368, t85918, t85941, t85952, t85963, t94858, t94874, t95069);
        let t103864 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2310::<F>(t8070, t94490, t86036, t95760, t103779, t1409, t1734, t19138, t24849, t24851, t27502, t27507, t27532, t27540, t29735, t3624, t3966, t5011, t6256, t7327, t7376, t8082, t86015, t86116, t95098, t95114, t95197, t95201, t95761);
        let t103889 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2311::<F>(t8074, t94909, t24826, t29745, t24574, t29705, t477, t6238, t1090, t17635, t19145, t24812, t24820, t24821, t27549, t27550, t27551, t29753, t7283, t7362, t85863, t85986, t86000, t95125, t95134, t95136);
        let t103918 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2312::<F>(t1186, t11881, t1751, t19145, t19165, t24812, t24814, t24815, t27517, t27533, t27549, t27550, t29708, t29711, t29719, t29726, t3242, t3610, t3624, t3961, t5068, t5079, t6146, t7283, t7381, t94395, t95092, t95163, t95165, t95192, t95213);
        let t103949 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2313::<F>(t24574, t29777, t29678, t7359, t29759, t1244, t1246, t15245, t1734, t19120, t19169, t2121, t2147, t24776, t24858, t27406, t27546, t27574, t27721, t29711, t3624, t462, t5079, t5971, t7283, t7373, t7375, t7376, t95714, t95722);
        let t103978 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2314::<F>(t24574, t29790, t29763, t8067, t94490, t11914, t1201, t1244, t1246, t18572, t18940, t19153, t2144, t2152, t27406, t27460, t27466, t27474, t27478, t29708, t29773, t4733, t5011, t5064, t7283, t7362, t8054, t95726);
        let t104002 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2315::<F>(t11888, t1215, t1244, t1246, t15032, t1729, t19156, t19179, t24589, t27465, t27516, t27722, t29664, t29708, t29712, t3604, t4964, t6168, t7373, t7375, t7376, t7389, t8083, t8085, t95747, t95751, t95758);
        let t104029 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2316::<F>(t27604, t4993, t19095, t24733, t1207, t19024, t7337, t19046, t7338, t6169, t7344, t1218, t1232, t1737, t1748, t18307, t18943, t18959, t24716, t6221, t7339, t7345, t86164, t95242, t95244, t95276, t95440);
        let t104056 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2317::<F>(t18375, t7339, t27599, t4997, t18360, t18364, t18397, t18401, t19002, t19016, t24741, t27617, t4950, t4980, t4984, t5014, t5030, t86324, t86327, t95566, t95623, t95627);
    (t103733, t103766, t103801, t103829, t103864, t103889, t103918, t103949, t103978, t104002, t104029, t104056)
}
