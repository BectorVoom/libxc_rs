//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta648 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2238;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2239;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2240;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2241;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2242;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2243;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2244;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2245;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2246;
use chunk9::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2247;
use chunk10::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2248;
use chunk11::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2249;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta648<F: Float>(t7611: F, t82713: F, t82716: F, t3040: F, t7593: F, t25550: F, t82822: F, t23384: F, t25476: F, t1058: F, t1060: F, t13940: F, t14488: F, t14618: F, t1945: F, t1953: F, t23701: F, t25499: F, t25516: F, t25535: F, t2776: F, t3186: F, t3200: F, t3201: F, t4615: F, t4673: F, t6687: F, t6784: F, t6797: F, t6813: F, t7610: F, t82592: F, t986: F, t4541: F, t984: F, t25467: F, t25459: F, t11037: F, t13933: F, t14526: F, t1615: F, t1920: F, t1948: F, t1949: F, t23346: F, t23571: F, t23670: F, t25541: F, t25558: F, t25713: F, t25718: F, t3076: F, t3188: F, t345: F, t7622: F, t88941: F, t7604: F, t82632: F, t25723: F, t88810: F, t1409: F, t1539: F, t6746: F, t82655: F, t14220: F, t7581: F, t11034: F, t1599: F, t1629: F, t23518: F, t23604: F, t23620: F, t23633: F, t25567: F, t25659: F, t25708: F, t82382: F, t82653: F, t82789: F, t83233: F, t83245: F, t83265: F, t89106: F, t25555: F, t25529: F, t6680: F, t2966: F, t7614: F, t14622: F, t14651: F, t1610: F, t23478: F, t23635: F, t23685: F, t23707: F, t25712: F, t4684: F, t61774: F, t6800: F, t6811: F, t7619: F, t82566: F, t82799: F, t82806: F, t25471: F, t82431: F, t7607: F, t25490: F, t82514: F, t7577: F, t1014: F, t1023: F, t1049: F, t12648: F, t12652: F, t23327: F, t23601: F, t23602: F, t23605: F, t23705: F, t23714: F, t25429: F, t25470: F, t25485: F, t25491: F, t25492: F, t25510: F, t25554: F, t25721: F, t3041: F, t3121: F, t4669: F, t4677: F, t6743: F, t82513: F, t82809: F, t89194: F, t89205: F, t83244: F, t974: F, t985: F, t3030: F, t343: F, t25483: F, t25486: F, t1022: F, t23678: F, t25479: F, t25705: F, t4680: F, t82668: F, t82823: F, t82828: F, t82830: F, t83246: F, t88155: F, t4547: F, t82573: F, t11051: F, t11054: F, t14608: F, t23654: F, t23662: F, t25493: F, t25518: F, t25549: F, t3016: F, t353: F, t383: F, t4649: F, t6768: F, t6786: F, t7620: F, t82534: F, t82625: F, t88728: F, t10165: F, t1052: F, t1055: F, t13736: F, t1634: F, t1956: F, t23378: F, t23721: F, t23722: F, t25400: F, t25743: F, t25797: F, t3026: F, t3174: F, t3175: F, t4557: F, t4660: F, t50625: F, t6771: F, t7624: F, t83358: F, t83364: F, t83368: F, t83420: F, t88954: F, t89001: F, t89042: F, t89066: F, t89101: F, t89143: F, t89181: F, t89225: F, t89265: F, t89297: F, t25766: F, t968: F, t25739: F, t11010: F, t14552: F, t14555: F, t1603: F, t23329: F, t25423: F, t25430: F, t25755: F, t25767: F, t3020: F, t3169: F, t3207: F, t388: F, t50632: F, t6776: F, t6816: F, t7625: F, t25751: F, t4657: F, t6703: F, t7554: F, t7561: F, t225: F, t25789: F, t1066: F, t13742: F, t1635: F, t23394: F, t23588: F, t25407: F, t25732: F, t4542: F, t50653: F, t50690: F, t6704: F, t6706: F, t82402: F, t83398: F, t83408: F, t25802: F, t23587: F, t7560: F, t25410: F, t14548: F, t23341: F, t25436: F, t3206: F, t83435: F, t83441: F, t83444: F, t25798: F, t25822: F, t7557: F, t10160: F, t23353: F, t23365: F, t25403: F, t25453: F, t25738: F, t7600: F, t82442: F, t82499: F, t83457: F, t83459: F) -> (F, F, F, F, F) {
        let (t89312, t89330) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2238::<F>(t7611, t82713, t82716, t3040, t7593, t25550, t82822, t23384, t25476, t1058, t1060, t13940, t14488, t14618, t1945, t1953, t23701, t25499, t25516, t25535, t2776, t3186, t3200, t3201, t4615, t4673, t6687, t6784, t6797, t6813, t7610, t82592, t986);
        let (t89349, t89363) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2239::<F>(t4541, t984, t23384, t25467, t25459, t1058, t1060, t11037, t13933, t14526, t1615, t1920, t1948, t1949, t23346, t23571, t23670, t25541, t25558, t25713, t25718, t3076, t3186, t3188, t345, t6687, t7622, t88941, t89312);
        let (t89375, t89402) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2240::<F>(t7604, t82632, t25723, t88810, t1409, t3040, t1539, t6746, t82655, t14220, t7581, t11034, t1599, t1629, t23346, t23518, t23604, t23620, t23633, t25467, t25567, t25659, t25708, t3186, t4673, t6687, t82382, t82653, t82789, t83233, t83245, t83265, t89106);
        let t89433 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2241::<F>(t25555, t82822, t25529, t6680, t1920, t2966, t7614, t14622, t14651, t1539, t1610, t23478, t23633, t23635, t23685, t23707, t25567, t25712, t3200, t4684, t61774, t6687, t6784, t6800, t6811, t7619, t82566, t82799, t82806);
        let t89477 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2242::<F>(t25471, t82431, t7607, t82632, t25490, t82514, t23518, t7577, t1014, t1023, t1049, t12648, t12652, t23327, t23601, t23602, t23605, t23633, t23705, t23714, t25429, t25470, t25485, t25491, t25492, t25510, t25554, t25721, t3041, t3121, t4669, t4677, t6743, t82513, t82809, t89194, t89205);
        let t89515 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2243::<F>(t83244, t974, t985, t3030, t343, t25483, t25486, t25490, t25492, t1022, t1058, t1060, t23633, t23670, t23678, t25479, t25499, t25554, t25555, t25705, t25713, t3200, t4680, t4684, t6687, t6743, t82668, t82823, t82828, t82830, t83245, t83246, t88155, t89375);
        let t89547 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2244::<F>(t23478, t4547, t7607, t82573, t1058, t1060, t11051, t11054, t14608, t23327, t23346, t23633, t23654, t23662, t25493, t25518, t25549, t3016, t3186, t353, t383, t4649, t4669, t6687, t6768, t6786, t7614, t7619, t7620, t82382, t82534, t82625, t88728);
        let t89556 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2245::<F>(t10165, t1052, t1055, t13736, t1599, t1634, t1956, t23346, t23378, t23721, t23722, t25400, t25743, t25797, t3026, t3174, t3175, t4557, t4660, t50625, t6687, t6771, t7624, t83358, t83364, t83368, t83420, t88941, t88954, t89001, t89042, t89066, t89101, t89143, t89181, t89225, t89265, t89297, t89330, t89363, t89402, t89433, t89477, t89515, t89547);
        let t89590 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2246::<F>(t1920, t25766, t968, t23384, t25739, t11010, t12652, t14552, t14555, t1603, t1956, t23327, t23329, t23571, t25423, t25429, t25430, t25743, t25755, t25767, t3020, t3169, t3207, t388, t50632, t6680, t6687, t6776, t6816, t7593, t7625, t986);
        let t89623 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2247::<F>(t25751, t82431, t4657, t6703, t7554, t82573, t1920, t2966, t7561, t225, t25789, t1066, t13742, t1635, t1956, t23346, t23394, t23588, t25407, t25732, t3169, t4542, t50653, t50690, t6687, t6704, t6706, t82402, t83398, t83408);
        let t89658 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2248::<F>(t23384, t25802, t23587, t7560, t25410, t1052, t14548, t23341, t23346, t23394, t25436, t25797, t3016, t3174, t3206, t4557, t6687, t6704, t7561, t7624, t83435, t83441, t83444, t89349, t986);
        let t89690 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2249::<F>(t23384, t25798, t225, t25822, t7557, t82632, t10160, t1066, t14555, t1599, t1635, t23346, t23353, t23365, t23378, t25403, t25453, t25738, t3169, t4557, t6687, t6816, t7600, t82442, t82499, t83457, t83459);
    (t89556, t89590, t89623, t89658, t89690)
}
