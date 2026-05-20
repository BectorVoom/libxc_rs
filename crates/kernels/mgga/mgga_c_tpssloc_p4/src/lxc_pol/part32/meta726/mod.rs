//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta726 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2341;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2342;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2343;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2344;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2345;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2346;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2347;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2348;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2349;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2350;
use chunk10::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2351;
use chunk11::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2352;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta726<F: Float>(t225: F, t29665: F, t8006: F, t94490: F, t11606: F, t1190: F, t1238: F, t1252: F, t15797: F, t15820: F, t1716: F, t1720: F, t19208: F, t19213: F, t19219: F, t24615: F, t27721: F, t27784: F, t27785: F, t29536: F, t29664: F, t3593: F, t498: F, t6243: F, t7283: F, t7300: F, t7301: F, t7391: F, t8014: F, t8061: F, t8088: F, t86501: F, t94391: F, t94558: F, t95912: F, t29827: F, t3640: F, t103164: F, t103213: F, t103258: F, t103279: F, t103303: F, t103341: F, t103377: F, t103415: F, t103457: F, t103488: F, t104508: F, t104534: F, t104564: F, t104596: F, t104631: F, t1254: F, t1256: F, t1763: F, t19262: F, t193: F, t24905: F, t24909: F, t27838: F, t27843: F, t336: F, t4700: F, t5091: F, t6270: F, t6274: F, t7398: F, t86517: F, t86524: F, t95921: F, t95925: F, t28: F, t265: F, t504: F, t100624: F, t100805: F, t1409: F, t16558: F, t2161: F, t27850: F, t29840: F, t3966: F, t52: F, t5398: F, t607: F, t7402: F, t8097: F, dens_threshold: F, rho1: F, zeta_threshold: F, t103125: F, t113: F, t1393: F, t1849: F, t19289: F, t19450: F, t20098: F, t2114: F, t2165: F, t2167: F, t27903: F, t29497: F, t33690: F, t4073: F, t96355: F, t96358: F, t96360: F, t96738: F, t96740: F, t96746: F, t96755: F, t96758: F, t96760: F, t96763: F, t96765: F, t96767: F, t5456: F, t7263: F, t2109: F, t96461: F, t96469: F, t96425: F, t22549: F, t24514: F, t24517: F, t26016: F, t27298: F, t83717: F, t85501: F, t90098: F, t90101: F, t90104: F, t96135: F, t96138: F, t96418: F, t96422: F, t96466: F, t96473: F, t2110: F, t26009: F, t27937: F, t27979: F, t7256: F, t7259: F, t90114: F, t96102: F, t96110: F, t96115: F, t96120: F, t96383: F, t96443: F, t96646: F, t26012: F, t7974: F, t1860: F, t26024: F, t26028: F, t27303: F, t27308: F, t27365: F, t27956: F, t29481: F, t6486: F, t7255: F, t7428: F, t7975: F, t7978: F, t96045: F, t96379: F, t96458: F, t5415: F, t55: F, t17635: F, t17686: F, t17691: F, t1864: F, t24498: F, t26090: F, t27311: F, t27332: F, t27356: F, t27364: F, t29474: F, t29475: F, t29478: F, t3961: F, t6495: F, t6509: F, t67: F, t7246: F, t7251: F, t7432: F, t7445: F, t83803: F, t85539: F, t96025: F, t96157: F, t96393: F, t26063: F, t26067: F, t27341: F, t27966: F, t95981: F, t96028: F, t96072: F, t96406: F, t96479: F, t96482: F, t2108: F, t2240: F, t5392: F, t605: F, t1410: F, t24520: F, t24526: F, t27972: F, t27976: F, t6492: F, t9239: F, t96502: F, t96506: F, t96517: F, t96521: F, t96553: F, t96556: F, t26070: F, t26073: F, t26076: F, t27961: F, t27982: F, t7435: F, t85480: F, t85536: F, t96403: F, t96559: F, t96562: F, t55921: F, t7245: F, t12571: F, t27331: F, t29473: F, t33: F, t26055: F, t96535: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t104669 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2341::<F>(t225, t29665, t8006, t94490, t11606, t1190, t1238, t1252, t15797, t15820, t1716, t1720, t19208, t19213, t19219, t24615, t27721, t27784, t27785, t29536, t29664, t3593, t498, t6243, t7283, t7300, t7301, t7391, t8014, t8061, t8088, t86501, t94391, t94558, t95912);
        let t104708 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2342::<F>(t29827, t3640, t103164, t103213, t103258, t103279, t103303, t103341, t103377, t103415, t103457, t103488, t104508, t104534, t104564, t104596, t104631, t104669, t1254, t1256, t1763, t19262, t193, t24905, t24909, t27838, t27843, t336, t4700, t5091, t6270, t6274, t7398, t86517, t86524, t95921, t95925);
        let t104721 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2343::<F>(t28, t265, t504, t100624, t104708, t100805, t1409, t16558, t2161, t27850, t29840, t3966, t52, t5398, t607, t7402, t8097, dens_threshold, rho1, zeta_threshold);
        let t104727 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2344::<F>(t103125, t104721, t113, t1393, t1849, t19289, t19450, t20098, t2114, t2165, t2167, t27903, t29497, t33690, t4073, t96355, t96358, t96360, t96738, t96740, t96746, t96755, t96758, t96760, t96763, t96765, t96767);
        let (t104729, t104758) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2345::<F>(t5456, t7263, t2109, t96461, t96469, t96425, t22549, t24514, t24517, t26016, t27298, t83717, t85501, t90098, t90101, t90104, t96135, t96138, t96418, t96422, t96466, t96473);
        let t104783 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2346::<F>(t2110, t24517, t26009, t26016, t27298, t27937, t27979, t7256, t7259, t90114, t96102, t96110, t96115, t96120, t96383, t96443, t96646);
        let t104813 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2347::<F>(t26012, t7974, t1860, t2109, t22549, t24514, t26009, t26024, t26028, t27303, t27308, t27365, t27956, t29481, t6486, t7255, t7428, t7975, t7978, t96045, t96379, t96458);
        let t104858 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2348::<F>(t5415, t55, t16558, t17635, t17686, t17691, t1860, t1864, t24498, t26090, t27311, t27332, t27356, t27364, t29474, t29475, t29478, t29481, t3961, t3966, t607, t6486, t6495, t6509, t67, t7246, t7251, t7428, t7432, t7445, t83803, t85539, t96025, t96157, t96393);
        let t104885 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2349::<F>(t2110, t26063, t26067, t27332, t27341, t27966, t7256, t7259, t7432, t95981, t96028, t96072, t96406, t96479, t96482);
        let t104916 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2350::<F>(t2108, t2240, t5392, t1409, t605, t1410, t2110, t24520, t24526, t26009, t27972, t27976, t6492, t7246, t9239, t96502, t96506, t96517, t96521, t96553, t96556);
        let t104942 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2351::<F>(t2110, t24514, t26070, t26073, t26076, t27303, t27365, t27961, t27982, t7256, t7259, t7435, t7975, t85480, t85536, t96403, t96559, t96562);
        let t104971 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2352::<F>(t55921, t7245, t12571, t27331, t2240, t29473, t33, t2110, t26055, t26070, t26073, t26076, t26090, t27308, t27311, t27341, t6492, t7435, t7975, t7978, t96535);
    (t104727, t104729, t104758, t104783, t104813, t104858, t104885, t104916, t104942, t104971)
}
