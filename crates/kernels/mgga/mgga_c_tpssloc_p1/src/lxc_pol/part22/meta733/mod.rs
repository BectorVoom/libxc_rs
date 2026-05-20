//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta733 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2404;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2405;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2406;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2407;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2408;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2409;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta733<F: Float>(t13515: F, t5727: F, t17423: F, t4354: F, t49269: F, t5730: F, t21268: F, t42143: F, t21300: F, t2787: F, t47705: F, t47707: F, t48103: F, t48919: F, t48924: F, t68442: F, t68444: F, t68446: F, t68448: F, t68452: F, t68454: F, t41684: F, t41863: F, t68460: F, t68464: F, t68468: F, t68472: F, t68479: F, t68483: F, t68486: F, t68489: F, t68492: F, t68494: F, t68498: F, t68500: F, t68502: F, t68504: F, t68506: F, t68509: F, t68511: F, t68515: F, t68518: F, t68523: F, t68527: F, t68530: F, t48155: F, t59657: F, t60163: F, t60168: F, t60173: F, t68536: F, t68541: F, t68545: F, t68549: F, t68552: F, t68556: F, t68563: F, t48157: F, t60192: F, t60194: F, t60202: F, t68571: F, t68577: F, t68580: F, t68583: F, t68586: F, t68589: F, t68592: F, t42086: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t60204: F, t68596: F, t68599: F, t68602: F, t68605: F, t68608: F, t42087: F, t47787: F, t59700: F, t59702: F, t59704: F, t60274: F, t68619: F, t68626: F, t68628: F, t68630: F, t68633: F, t68635: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t68767, t68769, t68771, t68773, t68775, t68785) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2404::<F>(t13515, t5727, t17423, t4354, t49269, t5730, t21268, t42143, t21300, t2787, t47705, t47707, t48103, t48919, t48924, t68442, t68444, t68446, t68448, t68452, t68454);
        let t68798 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2405::<F>(t41684, t41863, t68460, t68464, t68468, t68472, t68479, t68483, t68486, t68489, t68492, t68494);
        let t68812 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2406::<F>(t68498, t68500, t68502, t68504, t68506, t68509, t68511, t68515, t68518, t68523, t68527, t68530);
        let t68825 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2407::<F>(t48155, t59657, t60163, t60168, t60173, t68536, t68541, t68545, t68549, t68552, t68556, t68563);
        let (t68839, t68851) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2408::<F>(t48157, t60192, t60194, t60202, t68571, t68577, t68580, t68583, t68586, t68589, t68592, t42086, t59663, t59665, t59680, t59688, t59694, t60204, t68596, t68599, t68602, t68605, t68608);
        let t68864 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2409::<F>(t42087, t47787, t59700, t59702, t59704, t60274, t68619, t68626, t68628, t68630, t68633, t68635);
    (t68767, t68769, t68771, t68773, t68775, t68785, t68798, t68812, t68825, t68839, t68851, t68864)
}
