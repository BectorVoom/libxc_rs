//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta742 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2452;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2453;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2454;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2455;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2456;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2457;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2458;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2459;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2460;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2461;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2462;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2463;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta742<F: Float>(t17863: F, t2986: F, t48279: F, t10231: F, t21409: F, t973: F, t21462: F, t2970: F, t10186: F, t1597: F, t17841: F, t21410: F, t21419: F, t21444: F, t21463: F, t2960: F, t343: F, t4518: F, t4540: F, t4546: F, t48067: F, t5836: F, t61288: F, t61291: F, t61294: F, t67060: F, t68458: F, t68554: F, t977: F, t978: F, t984: F, t17635: F, t4337: F, t10254: F, t21510: F, t13769: F, t13835: F, t13839: F, t1409: F, t17748: F, t17800: F, t17804: F, t2988: F, t2989: F, t4531: F, t5681: F, t5685: F, t61082: F, t61103: F, t61279: F, t61307: F, t61310: F, t61313: F, t61322: F, t61327: F, t61365: F, t6733: F, t21472: F, t13822: F, t21452: F, t21468: F, t42972: F, t21453: F, t21469: F, t21473: F, t48293: F, t48321: F, t61383: F, t61387: F, t61391: F, t61394: F, t61397: F, t61405: F, t61408: F, t61422: F, t61427: F, t21456: F, t28565: F, t48329: F, t48336: F, t48339: F, t48374: F, t48379: F, t48382: F, t48397: F, t61447: F, t61472: F, t61489: F, t61495: F, t61557: F, t61597: F, t61600: F, t61602: F, t69533: F, t69574: F, t69665: F, t69695: F, t1049: F, t1052: F, t1065: F, t1625: F, t1635: F, t17583: F, t17588: F, t17875: F, t18071: F, t18166: F, t21480: F, t21662: F, t21663: F, t3026: F, t3174: F, t381: F, t388: F, t4557: F, t4660: F, t4665: F, t4694: F, t61058: F, t21682: F, t225: F, t1009: F, t1057: F, t10482: F, t5866: F, t1022: F, t1058: F, t1060: F, t1061: F, t11059: F, t14618: F, t18083: F, t18100: F, t18111: F, t18138: F, t18162: F, t21594: F, t21618: F, t21637: F, t21643: F, t23508: F, t3180: F, t3186: F, t360: F, t43503: F, t43576: F, t43577: F, t4669: F, t50508: F, t50509: F, t5932: F, t68441: F, t68706: F, t68708: F, t68710: F, t68715: F, t68717: F, t68760: F, t68762: F, t68764: F, t68767: F, t68769: F, t68771: F, t68773: F, t68775: F, t68883: F, t68885: F, t68887: F, t68891: F, t68894: F, t68896: F, t68905: F, t68910: F, t68912: F, t68916: F, t68918: F, t68920: F, t68923: F, t68926: F, t68930: F, t68934: F, t68936: F, t68938: F, t68940: F, t68943: F, t68947: F, t68949: F, t68951: F, t68954: F, t68995: F, t68998: F, t69003: F, t69005: F, t69011: F, t69014: F, t69018: F, t69021: F, t69023: F, t69025: F, t69027: F, t69030: F, t69036: F, t69253: F, t69255: F, t69257: F, t69259: F, t69261: F, t69453: F, t69456: F, t69459: F, t69461: F, t69313: F, t69335: F, t69337: F, t69340: F, t69343: F, t69346: F, t69350: F, t69353: F, t69357: F, t69469: F, t69471: F, t69263: F, t69288: F, t69291: F, t69294: F, t69297: F, t69299: F, t69302: F, t69305: F, t69307: F, t69310: F, t4657: F, t5872: F, t1063: F, t11034: F, t11060: F, t1610: F, t18086: F, t18089: F, t18094: F, t18129: F, t21481: F, t21614: F, t21626: F, t21657: F, t3200: F, t3201: F, t43553: F, t43554: F, t4649: F, t4673: F, t4678: F, t47853: F, t5928: F, t21390: F, t11065: F, t14608: F, t1632: F, t17876: F, t18103: F, t18117: F, t18131: F, t21617: F, t21644: F, t21647: F, t21650: F, t3188: F, t43470: F, t43473: F, t4684: F, t5914: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t69741 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2452::<F>(t17863, t2986, t48279, t10231, t21409, t973, t21462, t2970, t10186, t1597, t17841, t21410, t21419, t21444, t21463, t2960, t343, t4518, t4540, t4546, t48067, t5836, t61288, t61291, t61294, t67060, t68458, t68554, t977, t978, t984);
        let (t69742, t69791) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2453::<F>(t17635, t4337, t10254, t21510, t13769, t13835, t13839, t1409, t17748, t17800, t17804, t2986, t2988, t2989, t4518, t4531, t4540, t5681, t5685, t61082, t61103, t61279, t61307, t61310, t61313, t61322, t61327, t61365, t6733);
        let t69817 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2454::<F>(t21472, t2970, t973, t13822, t21452, t21468, t42972, t21453, t21469, t21473, t2960, t48293, t48321, t61383, t61387, t61391, t61394, t61397, t61405, t61408, t61422, t61427);
        let t69837 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2455::<F>(t21456, t28565, t343, t4540, t4546, t48329, t48336, t48339, t48374, t48379, t48382, t48397, t61447, t61472, t61489, t61495, t61557, t61597, t61600, t61602, t973, t984);
        let (t69840, t69860) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2456::<F>(t69533, t69574, t69665, t69695, t69741, t69791, t69817, t69837, t1049, t1052, t1065, t1625, t1635, t17583, t17588, t17875, t18071, t18166, t21480, t21662, t21663, t3026, t3174, t381, t388, t4557, t4660, t4665, t4694, t61058);
        let (t69871, t69923, t69935, t69942) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2457::<F>(t21682, t225, t1009, t21480, t1057, t10482, t5866, t1022, t1049, t1058, t1060, t1061, t11059, t14618, t18083, t18100, t18111, t18138, t18162, t21594, t21618, t21637, t21643, t23508, t3180, t3186, t360, t43503, t43576, t43577, t4669, t50508, t50509, t5932);
        let t69958 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2458::<F>(t68441, t68706, t68708, t68710, t68715, t68717, t68760, t68762, t68764, t68767, t68769, t68771, t68773, t68775, t68883, t68885, t68887, t68891, t68894, t68896);
        let t69959 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2459::<F>(t68905, t68910, t68912, t68916, t68918, t68920, t68923, t68926, t68930, t68934, t68936, t68938, t68940, t68943, t68947, t68949, t68951, t68954, t68995, t68998);
        let t69961 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2460::<F>(t69003, t69005, t69011, t69014, t69018, t69021, t69023, t69025, t69027, t69030, t69036, t69253, t69255, t69257, t69259, t69261, t69453, t69456, t69459, t69461);
        let t69966 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2461::<F>(t69313, t69335, t69337, t69340, t69343, t69346, t69350, t69353, t69357, t69469, t69471, t69263, t69288, t69291, t69294, t69297, t69299, t69302, t69305, t69307, t69310, t69958, t69959, t69961);
        let (t69996, t70009) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2462::<F>(t4657, t5872, t1022, t1058, t1060, t1063, t11034, t11059, t11060, t1610, t18086, t18089, t18094, t18129, t21481, t21614, t21626, t21637, t21657, t3186, t3200, t3201, t43553, t43554, t4649, t4669, t4673, t4678, t47853, t5928);
        let (t70012, t70014, t70068) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2463::<F>(t225, t69840, t1049, t21390, t1058, t1060, t11034, t11059, t11060, t11065, t14608, t1632, t17876, t18103, t18117, t18131, t21617, t21643, t21644, t21647, t21650, t3186, t3188, t3200, t43470, t43473, t4649, t4684, t5914, t69996);
    (t69742, t69860, t69871, t69923, t69935, t69942, t69966, t70009, t70012, t70014, t70068)
}
