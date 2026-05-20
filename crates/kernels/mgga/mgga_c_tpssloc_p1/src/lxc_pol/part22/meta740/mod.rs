//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta740 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2437;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2438;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2439;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2440;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2441;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2442;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta740<F: Float>(t13520: F, t17507: F, t13727: F, t17510: F, t10661: F, t4395: F, t5730: F, t21303: F, t42028: F, t912: F, t21300: F, t2792: F, t21299: F, t2844: F, t2842: F, t10702: F, t17527: F, t21252: F, t42100: F, t42102: F, t10811: F, t14271: F, t14276: F, t17492: F, t17544: F, t17548: F, t17551: F, t21115: F, t2930: F, t4416: F, t4438: F, t4471: F, t59920: F, t60407: F, t931: F, t21194: F, t2888: F, t41684: F, t48799: F, t48800: F, t48809: F, t59657: F, t68442: F, t68444: F, t68446: F, t68448: F, t68479: F, t68483: F, t68486: F, t68489: F, t68492: F, t68494: F, t68498: F, t68571: F, t68577: F, t68580: F, t68583: F, t42245: F, t47787: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t59700: F, t59702: F, t59704: F, t59759: F, t59761: F, t68586: F, t68589: F, t68592: F, t68596: F, t68599: F, t68602: F, t68605: F, t68608: F, t10771: F, t14442: F, t17366: F, t17554: F, t17555: F, t21114: F, t21195: F, t21198: F, t21207: F, t21239: F, t21242: F, t2861: F, t2886: F, t2900: F, t311: F, t41821: F, t42128: F, t42154: F, t42226: F, t42228: F, t4433: F, t4449: F, t49285: F, t49411: F, t5758: F, t5762: F, t5794: F, t68702: F, t943: F, t951: F, t300: F, t69050: F, t69180: F, t69218: F, t69249: F, t69286: F, t69326: F, t14459: F, t17947: F, t959: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t69335, t69337, t69340, t69343, t69346) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2437::<F>(t13520, t17507, t13727, t17510, t10661, t4395, t5730, t21303, t42028, t912, t21300, t2792);
        let (t69350, t69353, t69357, t69368) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2438::<F>(t21299, t2844, t2842, t912, t10702, t17527, t4395, t21252, t42100, t42102, t10811, t14271, t14276, t17492, t17544, t17548, t17551, t21115, t2930, t4416, t4438, t4471, t59920, t60407, t69335, t69337, t69340, t69343, t69346, t931);
        let (t69380, t69425) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2439::<F>(t21194, t2888, t41684, t48799, t48800, t48809, t59657, t68442, t68444, t68446, t68448, t68479, t68483, t68486, t68489, t68492, t68494, t68498, t68571, t68577, t68580, t68583);
        let t69445 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2440::<F>(t42245, t47787, t59663, t59665, t59680, t59688, t59694, t59700, t59702, t59704, t59759, t59761, t68586, t68589, t68592, t68596, t68599, t68602, t68605, t68608);
        let t69449 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2441::<F>(t10771, t10811, t14442, t17366, t17554, t17555, t21114, t21195, t21198, t21207, t21239, t21242, t2861, t2886, t2900, t311, t41821, t42128, t42154, t42226, t42228, t4433, t4449, t49285, t49411, t5758, t5762, t5794, t68702, t69380, t69425, t69445, t931, t943, t951);
        let (t69453, t69456) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2442::<F>(t300, t69050, t69180, t69218, t69249, t69286, t69326, t69368, t69449, t14459, t17947, t959);
    (t69335, t69337, t69340, t69343, t69346, t69350, t69353, t69357, t69453, t69456)
}
