//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta675 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2544;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2545;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2546;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2547;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2548;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta675<F: Float>(t43959: F, t4786: F, t11424: F, t15051: F, t11185: F, t15061: F, t1117: F, t14914: F, t3264: F, t11350: F, t1682: F, t1136: F, t15118: F, t3332: F, t44131: F, t51453: F, t51456: F, t51459: F, t51463: F, t51466: F, t51470: F, t51472: F, t51474: F, t51476: F, t11352: F, t4819: F, t11303: F, t11306: F, t11344: F, t11415: F, t11420: F, t11430: F, t15117: F, t15136: F, t15156: F, t15159: F, t15164: F, t15165: F, t15168: F, t15171: F, t15172: F, t1683: F, t3333: F, t3351: F, t3357: F, t3359: F, t44172: F, t44177: F, t44179: F, t44214: F, t44361: F, t4820: F, t4823: F, t11190: F, t11191: F, t1671: F, t50826: F, t50919: F, t43727: F, t43729: F, t43748: F, t43750: F, t50828: F, t50832: F, t50834: F, t50897: F, t50900: F, t50903: F, t50905: F, t50907: F, t50912: F, t50917: F, t50921: F, t50926: F, t50931: F, t50934: F, t50948: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43816: F, t44348: F, t50937: F, t50940: F, t50946: F, t50950: F, t50952: F, t50954: F, t50957: F, t50961: F, t50966: F, t50994: F, t51000: F, t51004: F, t423: F, t1128: F, t15204: F, t3356: F, t4794: F, t11349: F, t1675: F, t14829: F, t3403: F, t11297: F, t11345: F, t11353: F, t1138: F, t11434: F, t1155: F, t15126: F, t15141: F, t15179: F, t15182: F, t15185: F, t3352: F, t3360: F, t3401: F, t44202: F, t44205: F, t44295: F, t44300: F, t4797: F, t4824: F, t4840: F, t11275: F, t1670: F, t43976: F, t11285: F, t4857: F, t11129: F, t11310: F, t11365: F, t11399: F, t11437: F, t11441: F, t15133: F, t15146: F, t15153: F, t15207: F, t15218: F, t15225: F, t1694: F, t1695: F, t3376: F, t3377: F, t3395: F, t43692: F, t44155: F, t44223: F, t4858: F, t4861: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51478, t51480, t51482, t51485, t51493) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2544::<F>(t43959, t4786, t11424, t15051, t11185, t15061, t1117, t14914, t3264, t11350, t1682, t1136, t15118, t3332, t44131, t51453, t51456, t51459, t51463, t51466, t51470, t51472, t51474, t51476);
        let t51538 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2545::<F>(t11352, t4819, t11303, t11306, t11344, t11350, t1136, t11415, t11420, t11430, t15117, t15136, t15156, t15159, t15164, t15165, t15168, t15171, t15172, t1682, t1683, t3332, t3333, t3351, t3357, t3359, t44172, t44177, t44179, t44214, t44361, t4820, t4823);
        let (t51549, t51570) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2546::<F>(t11190, t11191, t1671, t50826, t50919, t43727, t43729, t43748, t43750, t50828, t50832, t50834, t50897, t50900, t50903, t50905, t50907, t50912, t50917, t50921, t50926, t50931, t50934);
        let t51590 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2547::<F>(t50948, t43780, t43782, t43784, t43786, t43788, t43816, t44348, t50937, t50940, t50946, t50950, t50952, t50954, t50957, t50961, t50966, t50994, t51000, t51004);
        let (t51593, t51617) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2548::<F>(t423, t51570, t51590, t1128, t15204, t3356, t4794, t11349, t1675, t14829, t3403, t11297, t11345, t11353, t1138, t11434, t1155, t15126, t15141, t15179, t15182, t15185, t1683, t3352, t3360, t3401, t44202, t44205, t44295, t44300, t4797, t4824, t4840, t51549);
        let (t51641, t51664) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2549::<F>(t11275, t1670, t1117, t43976, t11285, t4857, t11129, t11303, t11310, t11365, t11399, t11437, t11441, t1155, t15133, t15146, t15153, t15207, t15218, t15225, t1694, t1695, t3376, t3377, t3395, t3401, t43692, t44155, t44223, t4858, t4861);
    (t51478, t51480, t51482, t51485, t51493, t51538, t51549, t51593, t51617, t51641, t51664)
}
