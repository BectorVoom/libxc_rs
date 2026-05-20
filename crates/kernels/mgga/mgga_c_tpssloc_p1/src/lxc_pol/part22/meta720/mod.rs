//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta720 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2334;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2335;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2336;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2337;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2338;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2339;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2340;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2341;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2342;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2343;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta720<F: Float>(t20949: F, t2697: F, t20882: F, t9638: F, t13258: F, t20988: F, t13251: F, t16853: F, t16946: F, t16949: F, t16976: F, t17013: F, t2643: F, t2645: F, t41467: F, t4172: F, t4248: F, t4257: F, t46550: F, t46628: F, t5591: F, t58461: F, t58472: F, t58474: F, t58495: F, t9642: F, t20887: F, t13242: F, t13254: F, t16839: F, t16903: F, t16935: F, t20972: F, t20974: F, t20983: F, t20986: F, t2632: F, t4119: F, t4178: F, t4180: F, t58480: F, t58482: F, t58504: F, t58528: F, t67607: F, t9627: F, t9646: F, t20969: F, t2639: F, t16752: F, t120: F, t13222: F, t13228: F, t13262: F, t13350: F, t13351: F, t1512: F, t16836: F, t16918: F, t16932: F, t16937: F, t17017: F, t20756: F, t41453: F, t4181: F, t4255: F, t46574: F, t5612: F, t58557: F, t58765: F, t67578: F, t829: F, t1484: F, t4233: F, t5544: F, t828: F, t1510: F, t16944: F, t2618: F, t46577: F, t5585: F, t5611: F, t58550: F, t58569: F, t58574: F, t67568: F, t776: F, t817: F, t819: F, t820: F, t13278: F, t5619: F, t59281: F, t16662: F, t16872: F, t16951: F, t20800: F, t20904: F, t20953: F, t2623: F, t2701: F, t41344: F, t4236: F, t46650: F, t46878: F, t5527: F, t5587: F, t58576: F, t843: F, t9607: F, t67441: F, t816: F, t20978: F, t20938: F, t838: F, t13177: F, t16859: F, t16912: F, t20963: F, t4167: F, t46692: F, t47285: F, t5614: F, t58616: F, t58668: F, t58670: F, t58853: F, t831: F, t9967: F, t20994: F, t2563: F, t16816: F, t16845: F, t16893: F, t16969: F, t20908: F, t4182: F, t46875: F, t46876: F, t58705: F, t58709: F, t58723: F, t58731: F, t58735: F, t20944: F, t41011: F, t119: F, t13365: F, t1516: F, t20943: F, t210: F, t2571: F, t41084: F, t41161: F, t4158: F, t4261: F, t46887: F, t46912: F, t46929: F, t5567: F, t5624: F, t58744: F, t58834: F, t67282: F, t787: F, t847: F, t9559: F, t9667: F, t46881: F, t16888: F, t20947: F, t20993: F, t2647: F, t4240: F, t46952: F, t46954: F, t58642: F, t58688: F, t58759: F, t58761: F, t58763: F, t67620: F, t13005: F, t13223: F, t16907: F, t16985: F, t20885: F, t221: F, t41096: F, t4191: F, t5617: F, t5628: F, t58791: F, t58797: F, t58809: F, t58845: F, t58847: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t67696 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2334::<F>(t20949, t2697, t20882, t9638, t13258, t20988, t13251, t16853, t16946, t16949, t16976, t17013, t2643, t2645, t41467, t4172, t4248, t4257, t46550, t46628, t5591, t58461, t58472, t58474, t58495, t9642);
        let t67732 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2335::<F>(t20887, t9638, t13242, t13251, t13254, t16839, t16903, t16935, t20972, t20974, t20983, t20986, t20988, t2632, t2643, t2645, t4119, t4178, t4180, t58480, t58482, t58504, t58528, t67607, t9627, t9642, t9646);
        let (t67739, t67777) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2336::<F>(t20969, t2639, t16752, t2632, t120, t13222, t13228, t13251, t13262, t13350, t13351, t1512, t16836, t16839, t16918, t16932, t16937, t17017, t20756, t20986, t2643, t2645, t41453, t41467, t4178, t4180, t4181, t4255, t46574, t5612, t58557, t58765, t67578, t67607, t829);
        let t67826 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2337::<F>(t1484, t4233, t5544, t828, t13222, t13228, t13350, t13351, t1510, t16944, t16949, t20969, t2618, t2643, t4178, t4255, t46577, t5585, t5591, t5611, t58550, t58569, t58574, t67568, t776, t817, t819, t820);
        let t67865 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2338::<F>(t13278, t5619, t1512, t59281, t1484, t16662, t16872, t16951, t20800, t20904, t20949, t20953, t2618, t2623, t2701, t4119, t41344, t4172, t4236, t46650, t46878, t5527, t5544, t5587, t58576, t776, t820, t843, t9607);
        let t67898 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2339::<F>(t67441, t816, t20978, t9638, t20938, t838, t20953, t2639, t13177, t13222, t13262, t13351, t16839, t16859, t16912, t20963, t2643, t2645, t4167, t46692, t47285, t5614, t58569, t58616, t58668, t58670, t58853, t831, t9642, t9967);
        let t67926 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2340::<F>(t20994, t2563, t13251, t13262, t16816, t16836, t16845, t16893, t16969, t20908, t2623, t4178, t4180, t4182, t46875, t46876, t58705, t58709, t58723, t58731, t58735, t67607);
        let t67957 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2341::<F>(t20944, t41011, t119, t13365, t1516, t16976, t20943, t210, t2571, t41084, t41161, t4119, t4158, t4261, t46887, t46912, t46929, t5544, t5567, t5624, t58744, t58834, t67282, t776, t787, t820, t843, t847, t9559);
        let t67988 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2342::<F>(t13278, t5614, t20963, t9667, t46881, t5587, t13222, t13251, t13350, t16888, t20947, t20993, t210, t2571, t2643, t2645, t2647, t4240, t46952, t46954, t5591, t58642, t58688, t58759, t58761, t58763, t67620, t776, t829);
        let (t68010, t68018) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2343::<F>(t20947, t776, t13005, t13222, t13223, t13251, t13350, t13365, t16907, t16985, t20885, t20972, t221, t2643, t41096, t4172, t4191, t4255, t5617, t5628, t58642, t58791, t58797, t58809, t58845, t58847);
    (t67696, t67732, t67739, t67777, t67826, t67865, t67898, t67926, t67957, t67988, t68010, t68018)
}
