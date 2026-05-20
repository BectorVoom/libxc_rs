//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta839 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3001;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3002;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3003;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3004;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3005;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3006;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3007;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3008;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3009;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3010;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3011;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta839<F: Float>(t60391: F, t60394: F, t60398: F, t60400: F, t60429: F, t60434: F, t60568: F, t60570: F, t60946: F, t60953: F, t60955: F, t60958: F, t60961: F, t1581: F, t49541: F, t60887: F, t14473: F, t4498: F, t60332: F, t942: F, t951: F, t959: F, t10623: F, t5808: F, t17954: F, t2907: F, t60741: F, t60744: F, t60748: F, t60750: F, t60752: F, t60787: F, t60966: F, t60970: F, t62729: F, t62730: F, t62732: F, t62733: F, t62736: F, t62737: F, t17152: F, t42972: F, t973: F, t10876: F, t13969: F, t17983: F, t13995: F, t14501: F, t1020: F, t1021: F, t10214: F, t10403: F, t10408: F, t1041: F, t14164: F, t14211: F, t1539: F, t17701: F, t17732: F, t18014: F, t248: F, t2979: F, t3040: F, t3070: F, t3071: F, t3120: F, t360: F, t42388: F, t42546: F, t42861: F, t43343: F, t4338: F, t4582: F, t4650: F, t48612: F, t50337: F, t5875: F, t59706: F, t59711: F, t59719: F, t61910: F, t10422: F, t18020: F, t10883: F, t17979: F, t17620: F, t2960: F, t10390: F, t17649: F, t17980: F, t17984: F, t3146: F, t42565: F, t43211: F, t43307: F, t43325: F, t43336: F, t43341: F, t50343: F, t50361: F, t50378: F, t50384: F, t55723: F, t974: F, t5893: F, t698: F, t17615: F, t3131: F, t5866: F, t1022: F, t5872: F, t10263: F, t10413: F, t14213: F, t14215: F, t14220: F, t14228: F, t14230: F, t42483: F, t43352: F, t43354: F, t4342: F, t4575: F, t49929: F, t50324: F, t50425: F, t50429: F, t5677: F, t5894: F, t61775: F, t18015: F, t1036: F, t18010: F, t14025: F, t14508: F, t13970: F, t14511: F, t13546: F, t14222: F, t1616: F, t17156: F, t17637: F, t17643: F, t3048: F, t3088: F, t3151: F, t378: F, t43382: F, t49934: F, t50438: F, t50442: F, t5885: F, t5890: F, t5904: F, t61686: F, t61717: F, t61760: F, t61803: F, t61835: F, t61876: F, t61921: F, t61965: F, t62007: F, t62042: F, t62101: F, t62145: F, t62185: F, t62225: F, t62258: F, t62296: F, t62333: F, t62362: F, t62398: F, t62427: F, t62475: F, t62512: F, t62544: F, t62576: F, t62616: F, t62648: F, t62680: F, t62722: F, t5914: F, t3166: F, t1023: F, t11034: F, t11054: F, t11059: F, t14596: F, t14651: F, t18080: F, t18083: F, t18088: F, t18094: F, t18099: F, t18104: F, t18111: F, t18161: F, t3186: F, t3188: F, t3200: F, t3201: F, t43470: F, t43562: F, t4649: F, t4669: F, t4673: F, t4689: F, t50509: F, t50610: F, t5932: F, t381: F, t61719: F, t1058: F, t1060: F, t11046: F, t14488: F, t14577: F, t14630: F, t1629: F, t18089: F, t18100: F, t18112: F, t18139: F, t18142: F, t18151: F, t3180: F, t43473: F, t4678: F, t5936: F, t1057: F, t61729: F, t3199: F, t61734: F, t1061: F, t11037: F, t11051: F, t11065: F, t14574: F, t14581: F, t14590: F, t14591: F, t14608: F, t14618: F, t14623: F, t14627: F, t18131: F, t18138: F, t3202: F, t43553: F, t43554: F, t4677: F, t47857: F, t5928: F, t5933: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t62739 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3001::<F>(t60391, t60394, t60398, t60400, t60429, t60434, t60568, t60570, t60946, t60953, t60955, t60958, t60961);
        let (t62742, t62744, t62748, t62750, t62753) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3002::<F>(t1581, t49541, t60887, t14473, t4498, t60332, t942, t951, t959, t10623, t5808, t17954, t2907);
        let t62754 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3003::<F>(t60741, t60744, t60748, t60750, t60752, t60787, t60966, t60970, t62742, t62744, t62748, t62750, t62753);
        let (t62757, t62766, t62778) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3004::<F>(t62729, t62730, t62732, t62733, t62736, t62737, t62739, t62754, t17152, t42972, t973, t10876, t13969, t17983);
        let t62803 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3005::<F>(t13995, t14501, t1020, t1021, t10214, t10403, t10408, t1041, t14164, t14211, t1539, t17701, t17732, t18014, t248, t2979, t3040, t3070, t3071, t3120, t360, t42388, t42546, t42861, t43343, t4338, t4582, t4650, t48612, t50337, t5875, t59706, t59711, t59719, t61910, t62757, t62766, t62778, t973);
        let t62829 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3006::<F>(t10422, t18020, t3070, t10883, t13969, t17979, t17620, t2960, t10390, t17649, t17980, t17984, t3146, t42565, t43211, t43307, t43325, t43336, t43341, t50343, t50361, t50378, t50384, t55723, t973, t974);
        let t62871 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3007::<F>(t5893, t698, t973, t17615, t2960, t3131, t5866, t1022, t5872, t10263, t10403, t10413, t13995, t14213, t14215, t14220, t14228, t14230, t3070, t3071, t42483, t43352, t43354, t4342, t4575, t49929, t50324, t50425, t50429, t5677, t5894, t61775);
        let t62909 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3008::<F>(t10403, t10422, t18015, t1036, t18010, t14025, t14508, t13970, t14511, t10263, t10408, t13546, t14222, t14228, t1616, t17156, t17637, t17643, t3048, t3070, t3071, t3088, t3151, t378, t43382, t49934, t50438, t50442, t55723, t5885, t5890, t5904, t973, t974);
        let t62914 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3009::<F>(t61686, t61717, t61760, t61803, t61835, t61876, t61921, t61965, t62007, t62042, t62101, t62145, t62185, t62225, t62258, t62296, t62333, t62362, t62398, t62427, t62475, t62512, t62544, t62576, t62616, t62648, t62680, t62722, t62803, t62829, t62871, t62909);
        let (t62925, t62945, t62953) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3010::<F>(t3040, t5914, t3166, t5872, t1023, t11034, t11054, t11059, t14596, t14651, t18080, t18083, t18088, t18094, t18099, t18104, t18111, t18161, t3186, t3188, t3200, t3201, t43470, t43562, t4649, t4669, t4673, t4689, t50509, t50610, t5932);
        let (t62984, t62988) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3011::<F>(t381, t61719, t1058, t1060, t11034, t11046, t11059, t14488, t14577, t14630, t14651, t1629, t18089, t18100, t18112, t18139, t18142, t18151, t3166, t3180, t3186, t3188, t3200, t3201, t43473, t4678, t5866, t5932, t5936, t62945);
        let t63022 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3012::<F>(t1057, t61729, t3199, t61734, t1061, t11037, t11051, t11065, t14574, t14581, t14590, t14591, t14608, t14618, t14623, t14627, t18131, t18138, t3040, t3186, t3202, t43553, t43554, t4677, t47857, t5928, t5932, t5933, t5936);
    (t62742, t62744, t62748, t62750, t62753, t62757, t62914, t62925, t62953, t62984, t62988, t63022)
}
