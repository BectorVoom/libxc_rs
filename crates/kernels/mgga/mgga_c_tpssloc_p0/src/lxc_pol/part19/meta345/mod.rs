//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta345 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1234;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1235;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1236;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1237;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1238;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1239;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1240;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1241;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1242;
use chunk9::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1243;
use chunk10::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1244;
use chunk11::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta345<F: Float>(t2617: F, t9973: F, t236: F, t40931: F, t240: F, t812: F, t2638: F, t9612: F, t831: F, t10021: F, t815: F, t2686: F, t9671: F, t2678: F, t2632: F, t2681: F, t2628: F, t2690: F, t2635: F, t232: F, t40925: F, t2379: F, t2553: F, t2630: F, t40934: F, t40938: F, t817: F, t819: F, t820: F, t843: F, t9607: F, t9613: F, t9967: F, t9974: F, t9978: F, t9983: F, t9674: F, t2697: F, t9618: F, t40904: F, t816: F, t2629: F, t835: F, t9972: F, t9667: F, t9666: F, t2639: F, t9663: F, t2701: F, t40926: F, t776: F, t9516: F, t13258: F, t9634: F, t9629: F, t9975: F, t6589: F, t67: F, t246: F, t9458: F, t10007: F, t119: F, t120: F, t13262: F, t210: F, t2571: F, t2643: F, t2645: F, t2646: F, t2647: F, t40972: F, t40977: F, t41039: F, t41072: F, t41161: F, t4178: F, t829: F, t9621: F, t9626: F, t9642: F, t9646: F, t9647: F, t9653: F, t40995: F, t41037: F, t41077: F, t41120: F, t41343: F, t10069: F, t10077: F, t10080: F, t10091: F, t10098: F, t13390: F, t13397: F, t226: F, t22997: F, t235: F, t2728: F, t2732: F, t40932: F, t40951: F, t40955: F, t4291: F, t9958: F, t10046: F, t814: F, t10016: F, t10058: F, t10073: F, t10081: F, t10094: F, t13453: F, t255: F, t2613: F, t2740: F, t41231: F, t41333: F, t808: F, t860: F, t863: F, t9661: F, t225: F, t9520: F, t10049: F, t10054: F, t10055: F, t10076: F, t10084: F, t10097: F, t10101: F, t10103: F, t10104: F, t10112: F, t10116: F, t218: F, t23175: F, t252: F, t259: F, t2597: F, t2633: F, t2679: F, t2684: F, t2718: F, t2720: F, t2729: F, t2733: F, t2736: F, t2738: F, t2743: F, t40890: F, t40891: F, t40895: F, t40909: F, t40917: F, t41230: F, t4182: F, t4281: F, t852: F, t855: F, t858: F, t861: F, t865: F, t866: F, t9584: F, t9590: F, t9632: F, t9976: F, t9981: F, t12935: F, t193: F, t202: F, t2522: F, t39585: F, t39590: F, t39593: F, t40848: F, t40887: F, t41252: F, t41254: F, t41256: F, t41258: F, t41260: F, t41262: F, t41266: F, t766: F, t870: F, t9470: F, t2378: F, t262: F, t39658: F, t41270: F, t41273: F, t41275: F, t41278: F, t41281: F, t41283: F, t41286: F, t41289: F, t41292: F, t41296: F, t4314: F, t868: F, t40672: F, t40705: F, t40724: F, t40756: F, t40791: F, t40819: F, t10647: F, t892: F, t914: F, t10650: F, t2837: F) -> (F, F, F) {
        let (t41344, t41349, t41355, t41363, t41365) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1234::<F>(t2617, t9973, t236, t40931, t240, t812, t2638, t9612, t831, t10021, t815, t2686, t9671);
        let (t41367, t41368, t41388, t41393) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1235::<F>(t2678, t2632, t2681, t9671, t2628, t2690, t812, t2635, t232, t40925, t2379, t2553, t2630, t2686, t40934, t40938, t41344, t41349, t41355, t41363, t41365, t817, t819, t820, t843, t9607, t9613, t9967, t9974, t9978, t9983);
        let (t41395, t41397, t41399, t41404, t41410, t41414) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1236::<F>(t2686, t9674, t2697, t9618, t40904, t816, t2681, t2629, t9612, t812, t835, t9972);
        let (t41429, t41434) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1237::<F>(t41414, t9978, t9667, t9983, t2617, t9666, t2635, t2639, t9663, t232, t41367, t2630, t2681, t2701, t40926, t41395, t41397, t41399, t41404, t41410, t776, t817, t819, t820, t831, t843, t9516, t9613);
        let (t41435, t41437, t41448, t41453, t41467, t41468) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1238::<F>(t13258, t9634, t9629, t2379, t2632, t776, t9975, t6589, t67, t246, t232, t9458);
        let t41487 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1239::<F>(t10007, t119, t120, t13262, t210, t2571, t2643, t2645, t2646, t2647, t40972, t40977, t41039, t41072, t41161, t41435, t41437, t41448, t41453, t41467, t41468, t4178, t829, t9516, t9621, t9626, t9642, t9646, t9647, t9653);
        let (t41490, t41495) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1240::<F>(t40995, t41037, t41077, t41120, t41343, t41393, t41434, t41487, t10069, t10077, t10080, t10091, t10098, t13390, t13397, t226, t22997, t235, t2617, t2728, t2732, t40926, t40932, t40934, t40938, t40951, t40955, t4291, t812, t829, t9958);
        let (t41520, t41549) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1241::<F>(t10046, t814, t10016, t10058, t10073, t10081, t10094, t13453, t255, t2613, t2617, t2728, t2732, t2740, t41231, t41333, t41368, t41429, t808, t812, t860, t863, t9661);
        let t41580 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1242::<F>(t225, t9520, t10049, t10054, t10055, t10076, t10084, t10097, t10101, t10103, t10104, t10112, t10116, t218, t22997, t23175, t252, t259, t2597, t2617, t2633, t2679, t2684, t2718, t2720, t2729, t2733, t2736, t2738, t2743, t40890, t40891, t40895, t40904, t40909, t40917, t41230, t41388, t41490, t41495, t41520, t41549, t4182, t4281, t4291, t812, t829, t852, t855, t858, t860, t861, t865, t866, t9584, t9590, t9612, t9632, t9976, t9981);
        let t41591 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1243::<F>(t12935, t193, t202, t2522, t2553, t39585, t39590, t39593, t40848, t40887, t41252, t41254, t41256, t41258, t41260, t41262, t41266, t41580, t766, t870, t9470);
        let t41603 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1244::<F>(t193, t2378, t262, t39658, t40977, t41270, t41273, t41275, t41278, t41281, t41283, t41286, t41289, t41292, t41296, t4314, t776, t868, t870, t9458, t9516);
        let (t41606, t41620, t41622) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1245::<F>(t40672, t40705, t40724, t40756, t40791, t40819, t41591, t41603, t10647, t892, t914, t10650, t2837);
    (t41606, t41620, t41622)
}
