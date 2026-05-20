//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta712 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2233;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2234;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2235;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2236;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2237;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2238;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2239;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2240;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2241;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta712<F: Float>(t17037: F, t1888: F, t22996: F, t232: F, t58204: F, t6646: F, t2632: F, t58166: F, t28423: F, t6579: F, t28427: F, t1902: F, t5611: F, t16830: F, t25255: F, t25262: F, t2617: F, t28413: F, t4234: F, t4291: F, t5585: F, t812: F, t81679: F, t829: F, t87154: F, t92516: F, t98461: F, t98464: F, t98467: F, t98471: F, t98475: F, t25038: F, t25248: F, t25249: F, t4119: F, t28419: F, t23035: F, t23153: F, t5527: F, t6637: F, t22893: F, t28341: F, t81640: F, t1484: F, t6552: F, t87586: F, t1509: F, t7510: F, t16815: F, t22986: F, t2647: F, t22992: F, t25269: F, t25297: F, t4166: F, t4182: F, t4281: F, t5612: F, t81615: F, t87166: F, t87521: F, t87523: F, t87534: F, t92543: F, t5584: F, t58226: F, t23110: F, t23185: F, t28418: F, t59331: F, t23168: F, t28330: F, t13397: F, t16816: F, t25261: F, t81633: F, t87536: F, t87545: F, t87547: F, t87566: F, t87582: F, t87584: F, t87602: F, t5631: F, t828: F, t25319: F, t16935: F, t17034: F, t25281: F, t4162: F, t5575: F, t6660: F, t7535: F, t81689: F, t81717: F, t82011: F, t87604: F, t87613: F, t87619: F, t87635: F, t87669: F, t87680: F, t92781: F, t92794: F, t28406: F, t814: F, t234: F, t776: F, t16758: F, t5593: F, t81865: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t98478, t98482, t98486, t98488, t98490, t98494) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2233::<F>(t17037, t1888, t22996, t232, t58204, t6646, t2632, t58166, t28423, t6579, t28427, t1902, t5611);
        let t98497 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2234::<F>(t16830, t25255, t25262, t2617, t28413, t4234, t4291, t5585, t812, t81679, t829, t87154, t92516, t98461, t98464, t98467, t98471, t98475, t98478, t98482, t98486, t98488, t98490, t98494);
        let (t98502, t98505, t98513, t98516) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2235::<F>(t25038, t25248, t25249, t4119, t28419, t6579, t23035, t23153, t5527, t6637, t22893, t28341, t81640);
        let (t98520, t98524, t98530, t98534) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2236::<F>(t1484, t6552, t6637, t87586, t1509, t7510, t1888, t232, t58166, t6646, t16815, t22986, t2647);
        let t98536 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2237::<F>(t22992, t25269, t25297, t4166, t4182, t4281, t5612, t812, t81615, t87166, t87521, t87523, t87534, t92543, t98502, t98505, t98513, t98516, t98520, t98524, t98530, t98534);
        let (t98541, t98546, t98549, t98553, t98564) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2238::<F>(t1902, t5584, t1888, t232, t58226, t6646, t23110, t23185, t28418, t59331, t23168, t28330);
        let t98566 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2239::<F>(t13397, t16816, t25261, t4182, t4234, t4281, t4291, t81633, t829, t87536, t87545, t87547, t87566, t87582, t87584, t87602, t98494, t98541, t98546, t98549, t98553, t98564);
        let t98587 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2240::<F>(t1888, t232, t5631, t6646, t828, t25319, t4119, t6552, t6637, t16935, t17034, t25261, t25281, t4162, t4281, t5575, t6660, t7535, t81689, t81717, t82011, t87604, t87613, t87619, t87635, t87669, t87680, t92781, t92794);
        let (t98592, t98601, t98608, t98610) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2241::<F>(t28406, t814, t234, t5631, t6552, t6637, t776, t16758, t22986, t2647, t6646, t5593, t81865);
    (t98497, t98524, t98536, t98566, t98587, t98592, t98601, t98608, t98610)
}
