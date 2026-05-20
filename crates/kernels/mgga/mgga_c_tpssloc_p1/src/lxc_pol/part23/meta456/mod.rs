//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta456 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1319;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1320;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1321;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1322;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1323;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1324;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1325;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1326;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1327;
use chunk9::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1328;
use chunk10::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1329;
use chunk11::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1330;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta456<F: Float>(t1484: F, t5611: F, t13222: F, t13350: F, t1510: F, t16891: F, t20947: F, t20972: F, t20993: F, t210: F, t2571: F, t2643: F, t46876: F, t5544: F, t5567: F, t58723: F, t58744: F, t67880: F, t67882: F, t67884: F, t67920: F, t67937: F, t9559: F, t9646: F, t232: F, t76001: F, t2632: F, t76085: F, t13283: F, t20963: F, t20969: F, t20981: F, t2630: F, t2645: F, t41096: F, t4167: F, t4178: F, t5527: F, t58809: F, t67607: F, t67644: F, t67976: F, t67978: F, t67980: F, t76090: F, t817: F, t819: F, t820: F, t843: F, t9607: F, t9974: F, t119: F, t16872: F, t20800: F, t20904: F, t20949: F, t2701: F, t41139: F, t41349: F, t4172: F, t46957: F, t47047: F, t5614: F, t5619: F, t68021: F, t75978: F, t76002: F, t76074: F, t76086: F, t787: F, t13005: F, t16771: F, t214: F, t221: F, t41155: F, t41161: F, t41185: F, t41200: F, t4127: F, t4128: F, t46764: F, t46772: F, t46790: F, t68073: F, t68110: F, t76056: F, t76063: F, t41209: F, t41212: F, t46806: F, t59195: F, t59204: F, t59206: F, t59218: F, t59221: F, t59224: F, t68116: F, t68118: F, t68122: F, t68131: F, t225: F, t13228: F, t1512: F, t20953: F, t237: F, t249: F, t59259: F, t59263: F, t59276: F, t59288: F, t67872: F, t68148: F, t68195: F, t68197: F, t68199: F, t68201: F, t76132: F, t76167: F, t76193: F, t76227: F, t10080: F, t1499: F, t16673: F, t17027: F, t20857: F, t20858: F, t21014: F, t226: F, t235: F, t255: F, t40932: F, t4166: F, t46524: F, t5585: F, t5612: F, t5617: F, t5653: F, t59355: F, t812: F, t13416: F, t1525: F, t20853: F, t20854: F, t20861: F, t20870: F, t20871: F, t20876: F, t20937: F, t2728: F, t4281: F, t4295: F, t5575: F, t5645: F, t5655: F, t67392: F, t5636: F, t13397: F, t1492: F, t1523: F, t1528: F, t16758: F, t16815: F, t16830: F, t17034: F, t17052: F, t17090: F, t17092: F, t20806: F, t20862: F, t20867: F, t20873: F, t20986: F, t21013: F, t21025: F, t21028: F, t21034: F, t21050: F, t259: F, t40890: F, t4147: F, t4268: F, t4291: F, t5637: F, t5648: F, t5651: F, t5658: F, t67305: F, t67339: F, t67344: F, t67405: F, t67429: F, t67441: F, t68246: F, t855: F, t858: F, t860: F, t5657: F, t10110: F, t1519: F, t1527: F, t20936: F, t21033: F, t21054: F, t218: F, t252: F, t2718: F, t5558: F, t5631: F, t68322: F, t193: F, t202: F, t2522: F, t39593: F, t41254: F, t4310: F, t67112: F, t75950: F, t75951: F, t75952: F, t76017: F, t76018: F, t76020: F, t76024: F, t76025: F, t766: F, t870: F, t16606: F, t16625: F, t2378: F, t39658: F, t41258: F, t41262: F, t4314: F, t68371: F, t76026: F, t76027: F, t76030: F, t76031: F, t76034: F, t76035: F, t76037: F) -> (F, F) {
        let (t76250, t76259) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1319::<F>(t1484, t5611, t13222, t13350, t1510, t16891, t20947, t20972, t20993, t210, t2571, t2643, t46876, t5544, t5567, t58723, t58744, t67880, t67882, t67884, t67920, t67937, t9559, t9646);
        let (t76274, t76290, t76295) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1320::<F>(t232, t76001, t2632, t76085, t13283, t1510, t20963, t20969, t20981, t2630, t2643, t2645, t41096, t4167, t4178, t5527, t5544, t58809, t67607, t67644, t67976, t67978, t67980, t76090, t817, t819, t820, t843, t9607, t9974);
        let (t76327, t76333) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1321::<F>(t232, t76085, t119, t1484, t16872, t20800, t20904, t20949, t210, t2630, t2701, t41139, t41349, t4172, t46957, t47047, t5614, t5619, t68021, t75978, t76002, t76074, t76086, t787, t817, t819, t820, t843);
        let t76359 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1322::<F>(t13005, t16771, t20800, t210, t214, t221, t2571, t41155, t41161, t41185, t41200, t4127, t4128, t46764, t46772, t46790, t5544, t68073, t68110, t75978, t76056, t76063, t787);
        let t76371 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1323::<F>(t41209, t41212, t46806, t59195, t59204, t59206, t59218, t59221, t59224, t68116, t68118, t68122, t68131);
        let (t76372, t76373, t76394) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1324::<F>(t76359, t76371, t225, t13222, t13228, t1512, t20953, t237, t249, t4167, t4178, t59259, t59263, t59276, t59288, t67872, t68148, t68195, t68197, t68199, t68201, t76250);
        let (t76397, t76414) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1325::<F>(t76132, t76167, t76193, t76227, t76259, t76295, t76333, t76394, t10080, t1499, t16673, t17027, t20857, t20858, t21014, t226, t235, t255, t40932, t4166, t46524, t5585, t5612, t5617, t5653, t59355, t76086, t76090, t76373, t812);
        let t76467 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1326::<F>(t13228, t13416, t1525, t16673, t20853, t20854, t20861, t20870, t20871, t20876, t20937, t2728, t4166, t4281, t4295, t5575, t5645, t5655, t67392, t76290, t812);
        let t76497 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1327::<F>(t5636, t13397, t1492, t1510, t1523, t1528, t16673, t16758, t16815, t16830, t17034, t17052, t17090, t17092, t20806, t20862, t20867, t20873, t20986, t21013, t21025, t21028, t21034, t21050, t259, t2728, t40890, t4147, t4166, t4268, t4281, t4291, t5612, t5637, t5648, t5651, t5658, t67305, t67339, t67344, t67392, t67405, t67429, t67441, t68246, t76002, t76074, t76274, t76327, t76414, t76467, t812, t855, t858, t860);
        let t76532 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1328::<F>(t5657, t10110, t1519, t1527, t1528, t17052, t17090, t20936, t21033, t21050, t21054, t218, t252, t259, t2718, t4147, t4268, t5558, t5631, t5636, t5637, t5658, t68322, t76372, t76397, t855);
        let t76543 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1329::<F>(t1484, t193, t202, t20800, t2522, t39593, t41254, t4310, t67112, t75950, t75951, t75952, t75978, t76017, t76018, t76020, t76024, t76025, t76497, t76532, t766, t870);
        let t76556 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1330::<F>(t16606, t16625, t193, t2378, t2522, t39658, t41258, t41262, t4314, t5527, t5544, t68371, t76026, t76027, t76030, t76031, t76034, t76035, t76037, t76063);
    (t76543, t76556)
}
