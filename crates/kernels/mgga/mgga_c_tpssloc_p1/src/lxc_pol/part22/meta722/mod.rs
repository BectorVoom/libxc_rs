//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta722 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2356;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2357;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2358;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2359;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2360;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2361;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2362;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2363;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2364;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2365;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2366;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2367;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta722<F: Float>(t13065: F, t13176: F, t13384: F, t13390: F, t13397: F, t13433: F, t1528: F, t16673: F, t16753: F, t16758: F, t16759: F, t16815: F, t16816: F, t16820: F, t16823: F, t16830: F, t17030: F, t17031: F, t17034: F, t17037: F, t17041: F, t17046: F, t17050: F, t17057: F, t17064: F, t17090: F, t17092: F, t20867: F, t20870: F, t20871: F, t20873: F, t20986: F, t21014: F, t21028: F, t21033: F, t21050: F, t226: F, t235: F, t25093: F, t255: F, t2597: F, t2617: F, t2718: F, t2732: F, t4162: F, t4166: F, t4182: F, t4234: F, t4268: F, t4273: F, t4280: F, t4281: F, t4283: F, t4288: F, t4290: F, t4291: F, t4292: F, t4295: F, t4301: F, t47374: F, t47386: F, t5575: F, t5585: F, t5617: F, t5637: F, t5648: F, t5653: F, t5655: F, t59498: F, t59519: F, t67339: F, t67344: F, t67350: F, t67358: F, t67392: F, t67403: F, t67405: F, t67429: F, t67582: F, t67596: F, t68144: F, t68211: F, t68217: F, t68256: F, t68299: F, t808: F, t812: F, t829: F, t855: F, t858: F, t865: F, t866: F, t21064: F, t225: F, t13042: F, t13463: F, t17052: F, t17070: F, t21034: F, t252: F, t259: F, t2713: F, t4142: F, t4147: F, t5631: F, t5658: F, t59503: F, t68143: F, t13053: F, t1492: F, t1519: F, t16804: F, t17022: F, t17056: F, t20936: F, t218: F, t25168: F, t4265: F, t46488: F, t5558: F, t58143: F, t852: F, t262: F, t5527: F, t193: F, t202: F, t39585: F, t39590: F, t4119: F, t67322: F, t67457: F, t67458: F, t67461: F, t67464: F, t67466: F, t67472: F, t67475: F, t870: F, t1530: F, t16596: F, t16944: F, t17120: F, t1877: F, t2522: F, t41258: F, t41262: F, t4310: F, t4314: F, t46436: F, t59584: F, t67487: F, t67488: F, t67489: F, t67490: F, t67494: F, t12899: F, t16662: F, t20753: F, t20769: F, t20778: F, t39658: F, t40772: F, t4315: F, t46341: F, t46438: F, t5544: F, t67495: F, t67496: F, t67497: F, t67498: F, t868: F, t16625: F, t25374: F, t4255: F, t59564: F, t67499: F, t67500: F, t67501: F, t67502: F, t67503: F, t67504: F, t67506: F, t67507: F, t67508: F, t1484: F, t16606: F, t16949: F, t17116: F, t20800: F, t2523: F, t25365: F, t39249: F, t39256: F, t39373: F, t39397: F, t39400: F, t39408: F, t39463: F, t39468: F, t39472: F, t39476: F, t39529: F, t39593: F, t40708: F, t40721: F, t40779: F, t40784: F, t41254: F, t46138: F, t46218: F, t46235: F, t46336: F, t57932: F, t67044: F, t67086: F, t67087: F, t67088: F, t67089: F, t67090: F, t67101: F, t67104: F, t67105: F, t67112: F, t67134: F, t67137: F, t67141: F, t67160: F, t67175: F, t67195: F, t67204: F, t67206: F, t67207: F, t67210: F, t67211: F, t67212: F, t67218: F, t67286: F, t67478: F, t67480: F, t67482: F, t67484: F, t67485: F, t67486: F, t776: F, t2: F, t5669: F, t584: F, t21589: F, t2940: F, t21152: F, t690: F, t21155: F, t21146: F, t21149: F) -> (F, F, F, F, F, F, F) {
        let t68305 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2356::<F>(t13065, t13176, t13384, t13390, t13397, t13433, t1528, t16673, t16753, t16758, t16759, t16815, t16816, t16820, t16823, t16830, t17030, t17031, t17034, t17037, t17041, t17046, t17050, t17057, t17064, t17090, t17092, t20867, t20870, t20871, t20873, t20986, t21014, t21028, t21033, t21050, t226, t235, t25093, t255, t2597, t2617, t2718, t2732, t4162, t4166, t4182, t4234, t4268, t4273, t4280, t4281, t4283, t4288, t4290, t4291, t4292, t4295, t4301, t47374, t47386, t5575, t5585, t5617, t5637, t5648, t5653, t5655, t59498, t59519, t67339, t67344, t67350, t67358, t67392, t67403, t67405, t67429, t67582, t67596, t68144, t68211, t68217, t68256, t68299, t808, t812, t829, t855, t858, t865, t866);
        let t68333 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2357::<F>(t21064, t225, t13042, t13463, t1528, t17050, t17052, t17070, t21034, t252, t259, t2713, t4142, t4147, t4268, t4273, t4301, t5631, t5637, t5658, t59503, t68143, t866);
        let t68365 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2358::<F>(t13042, t13053, t13065, t1492, t1519, t1528, t16804, t17022, t17056, t17090, t20936, t21034, t21050, t218, t25168, t259, t2597, t2713, t4265, t4301, t46488, t5558, t5637, t5658, t58143, t68211, t852);
        let t68375 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2359::<F>(t262, t5527, t193, t202, t39585, t39590, t4119, t67322, t67457, t67458, t67461, t67464, t67466, t67472, t67475, t68305, t68333, t68365, t870);
        let t68391 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2360::<F>(t1530, t16596, t16944, t17120, t1877, t2522, t41258, t41262, t4310, t4314, t46436, t59584, t67487, t67488, t67489, t67490, t67494);
        let t68407 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2361::<F>(t12899, t16662, t1877, t20753, t20769, t20778, t39658, t40772, t4314, t4315, t46341, t46438, t5544, t67495, t67496, t67497, t67498, t868);
        let t68414 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2362::<F>(t16625, t1877, t25374, t4255, t4314, t59564, t67499, t67500, t67501, t67502, t67503, t67504, t67506, t67507, t67508);
        let t68418 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2363::<F>(t1484, t16606, t16625, t16949, t17116, t20800, t2522, t2523, t25365, t39249, t39256, t39373, t39397, t39400, t39408, t39463, t39468, t39472, t39476, t39529, t39593, t40708, t40721, t40779, t40784, t4119, t41254, t4310, t4314, t46138, t46218, t46235, t46336, t57932, t67044, t67086, t67087, t67088, t67089, t67090, t67101, t67104, t67105, t67112, t67134, t67137, t67141, t67160, t67175, t67195, t67204, t67206, t67207, t67210, t67211, t67212, t67218, t67286, t67478, t67480, t67482, t67484, t67485, t67486, t68375, t68391, t68407, t68414, t776);
        let (t68427, t68441, t68442) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2364::<F>(t2, t5669, t584, t21589, t2940, t21152, t690);
        let t68444 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2365::<F>(t21155, t690);
        let t68446 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2366::<F>(t21146, t690);
        let t68448 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2367::<F>(t21149, t690);
    (t68418, t68427, t68441, t68442, t68444, t68446, t68448)
}
