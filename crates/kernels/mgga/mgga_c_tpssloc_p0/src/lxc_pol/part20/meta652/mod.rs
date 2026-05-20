//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta652 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2400;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2401;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2402;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2403;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2404;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2405;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2406;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta652<F: Float>(t41662: F, t41675: F, t41678: F, t41682: F, t41684: F, t41863: F, t41865: F, t41870: F, t41872: F, t41874: F, t41876: F, t48982: F, t47761: F, t47765: F, t47769: F, t48112: F, t48114: F, t48116: F, t48119: F, t48122: F, t48125: F, t48128: F, t48131: F, t41887: F, t41889: F, t48134: F, t48137: F, t48142: F, t48145: F, t48148: F, t49009: F, t49012: F, t49015: F, t49018: F, t49021: F, t48155: F, t41680: F, t41713: F, t47777: F, t48153: F, t48157: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F, t49040: F, t41959: F, t41962: F, t47781: F, t47785: F, t47787: F, t49043: F, t49049: F, t49052: F, t49054: F, t49056: F, t49058: F, t49060: F, t49127: F, t49140: F, t49154: F, t1556: F, t2842: F, t10727: F, t10702: F, t10704: F, t2836: F, t912: F, t10655: F, t14422: F, t2793: F, t4396: F, t10662: F, t4399: F, t10828: F, t1580: F, t10524: F, t10724: F, t10740: F, t10743: F, t10771: F, t10811: F, t10825: F, t14329: F, t14425: F, t14429: F, t14435: F, t14463: F, t1581: F, t2861: F, t2862: F, t2880: F, t4434: F, t4437: F, t931: F, t943: F, t951: F, t13515: F, t2837: F, t2841: F, t4351: F, t2845: F, t10697: F, t4354: F, t10701: F, t1543: F, t10705: F, t1557: F, t41618: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t49167 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2400::<F>(t41662, t41675, t41678, t41682, t41684, t41863, t41865, t41870, t41872, t41874, t41876, t48982);
        let (t49181, t49194) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2401::<F>(t47761, t47765, t47769, t48112, t48114, t48116, t48119, t48122, t48125, t48128, t48131, t41887, t41889, t48134, t48137, t48142, t48145, t48148, t49009, t49012, t49015, t49018, t49021);
        let t49208 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2402::<F>(t48155, t41680, t41713, t47777, t48153, t48157, t48159, t48161, t48163, t48165, t48167, t49040);
        let t49219 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2403::<F>(t41959, t41962, t47781, t47785, t47787, t49043, t49049, t49052, t49054, t49056, t49058, t49060);
        let (t49222, t49228, t49240) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2404::<F>(t49127, t49140, t49154, t49167, t49181, t49194, t49208, t49219, t1556, t2842, t10727, t10702);
        let (t49244, t49256, t49259, t49262) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2405::<F>(t10704, t2836, t49240, t912, t10655, t14422, t2793, t2842, t4396, t10662, t10702, t4399);
        let t49266 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2406::<F>(t10828, t1580, t10524, t10724, t10740, t10743, t10771, t10811, t10825, t14329, t14425, t14429, t14435, t14463, t1581, t2861, t2862, t2880, t4434, t4437, t49222, t49228, t49244, t49256, t49259, t49262, t931, t943, t951);
        let (t49268, t49271, t49273, t49276, t49278) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2407::<F>(t13515, t2837, t2841, t4351, t2845, t10697, t4354, t10701, t1543, t10705, t1557, t41618);
    (t49222, t49228, t49244, t49256, t49259, t49262, t49266, t49268, t49271, t49273, t49276, t49278)
}
