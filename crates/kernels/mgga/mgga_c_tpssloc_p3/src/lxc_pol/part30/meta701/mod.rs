//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta701 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2265;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2266;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2267;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2268;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2269;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2270;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2271;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2272;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta701<F: Float>(t23164: F, t7479: F, t86893: F, t17063: F, t23278: F, t25168: F, t5637: F, t82294: F, t87748: F, t87902: F, t87911: F, t87927: F, t87932: F, t92954: F, t92961: F, t99033: F, t10109: F, t13042: F, t13463: F, t1528: F, t16804: F, t17050: F, t17057: F, t17064: F, t17069: F, t17070: F, t17090: F, t17092: F, t1902: F, t1912: F, t23281: F, t25169: F, t25170: F, t25184: F, t25188: F, t25200: F, t25233: F, t25348: F, t259: F, t2713: F, t2718: F, t28307: F, t28311: F, t28431: F, t4147: F, t4268: F, t4272: F, t4273: F, t4301: F, t5558: F, t5657: F, t5658: F, t59498: F, t59503: F, t59537: F, t6624: F, t6627: F, t6632: F, t6662: F, t6663: F, t7517: F, t7537: F, t7538: F, t82087: F, t82099: F, t855: F, t865: F, t866: F, t86903: F, t86941: F, t86943: F, t87758: F, t87777: F, t87787: F, t87797: F, t87810: F, t87836: F, t87837: F, t87874: F, t92402: F, t92863: F, t98160: F, t98164: F, t98166: F, t98172: F, t98181: F, t98208: F, t98213: F, t98222: F, t98227: F, t98258: F, t98264: F, t98277: F, t98279: F, t98309: F, t98913: F, t98921: F, t98923: F, t98947: F, t98963: F, t98966: F, t98999: F, t99003: F, t99010: F, t99019: F, t99022: F, t870: F, t16596: F, t86721: F, t1484: F, t584: F, t86753: F, t22959: F, t16949: F, t25014: F, t1408: F, t4255: F, t193: F, t200: F, t7540: F, t16557: F, t1877: F, t1915: F, t23295: F, t25: F, t25013: F, t25015: F, t25021: F, t2522: F, t25354: F, t25366: F, t25372: F, t25385: F, t7541: F, t86736: F, t98091: F, t98094: F, t98103: F, t98112: F, t97989: F, t98039: F, t98090: F, t1634: F, t607: F, t1065: F, t5392: F, t17686: F, t1927: F, t23327: F, t23329: F, t25424: F, t25429: F, t25430: F, t25442: F, t25738: F, t25815: F, t28701: F, t28702: F, t4337: F, t7553: F, t82342: F, t82402: F, t82417: F, t88004: F, t88050: F, t88069: F, t88075: F, t88083: F, t88089: F, t88112: F, t1625: F, t7577: F, t14552: F, t1604: F, t17691: F, t254: F, t25423: F, t25431: F, t25750: F, t25759: F, t25801: F, t4342: F, t6691: F, t7625: F, t82502: F, t88058: F, t88096: F, t88162: F, t23384: F, t28481: F, t14529: F, t1539: F, t17582: F, t18061: F, t1956: F, t23346: F, t23394: F, t25784: F, t28705: F, t4548: F, t4664: F, t60971: F, t61058: F, t6687: F, t6704: F, t88100: F, t88102: F, t88152: F, t88772: F, t5837: F, t984: F, t28691: F, t82431: F, t14545: F, t1635: F, t18070: F, t23336: F, t23372: F, t25420: F, t25797: F, t28491: F, t4557: F, t5944: F, t61646: F, t7565: F, t7600: F, t82481: F, t88167: F, t88194: F, t88744: F, t89598: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t99038 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2265::<F>(t23164, t7479, t86893, t17063, t23278, t25168, t5637, t82294, t87748, t87902, t87911, t87927, t87932, t92954, t92961, t99033);
        let t99042 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2266::<F>(t10109, t13042, t13463, t1528, t16804, t17050, t17057, t17064, t17069, t17070, t17090, t17092, t1902, t1912, t23278, t23281, t25168, t25169, t25170, t25184, t25188, t25200, t25233, t25348, t259, t2713, t2718, t28307, t28311, t28431, t4147, t4268, t4272, t4273, t4301, t5558, t5637, t5657, t5658, t59498, t59503, t59537, t6624, t6627, t6632, t6662, t6663, t7517, t7537, t7538, t82087, t82099, t855, t865, t866, t86903, t86941, t86943, t87758, t87777, t87787, t87797, t87810, t87836, t87837, t87874, t92402, t92863, t98160, t98164, t98166, t98172, t98181, t98208, t98213, t98222, t98227, t98258, t98264, t98277, t98279, t98309, t98913, t98921, t98923, t98947, t98963, t98966, t98999, t99003, t99010, t99019, t99022, t99038);
        let (t99043, t99049, t99055, t99056, t99060) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2267::<F>(t870, t99042, t16596, t86721, t1484, t584, t86753, t22959, t16949, t25014, t1408, t4255);
        let (t99064, t99067) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2268::<F>(t193, t200, t7540, t1408, t16557, t1877, t1915, t22959, t23295, t25, t25013, t25015, t25021, t2522, t25354, t25366, t25372, t25385, t7541, t86736, t98091, t98094, t98103, t98112, t99043, t99049, t99055, t99056, t99060);
        let (t99069, t99070, t99099, t99104) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2269::<F>(t97989, t98039, t98090, t99067, t1634, t607, t1065, t5392, t17686, t1927, t23327, t23329, t25424, t25429, t25430, t25442, t25738, t25815, t28701, t28702, t4337, t7553, t82342, t82402, t82417, t88004, t88050, t88069, t88075, t88083, t88089, t88112);
        let t99143 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2270::<F>(t1625, t7577, t14552, t1604, t17691, t23327, t23329, t254, t25423, t25424, t25429, t25431, t25442, t25750, t25759, t25801, t25815, t28701, t4342, t6691, t7553, t7625, t82502, t88050, t88058, t88096, t88112, t88162, t99070);
        let t99172 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2271::<F>(t23384, t28481, t14529, t1539, t17582, t18061, t1927, t1956, t23327, t23346, t23394, t25784, t28705, t4548, t4664, t60971, t61058, t6687, t6704, t7625, t82402, t88100, t88102, t88152, t88772);
        let (t99180, t99202) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2272::<F>(t5837, t984, t23384, t28691, t28705, t82431, t14545, t1635, t18070, t1956, t23327, t23336, t23372, t25420, t25429, t25750, t25797, t28491, t4557, t5944, t61646, t6687, t6704, t7565, t7600, t82481, t88162, t88167, t88194, t88744, t89598);
    (t99042, t99043, t99055, t99064, t99069, t99099, t99104, t99143, t99172, t99180, t99202)
}
