//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1360;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1361;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1362;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1363;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta465<F: Float>(t17210: F, t5705: F, t21180: F, t4362: F, t17218: F, t4378: F, t48103: F, t68442: F, t68444: F, t68446: F, t68448: F, t68452: F, t68454: F, t68494: F, t68498: F, t68500: F, t2815: F, t41904: F, t47787: F, t59657: F, t76574: F, t76578: F, t76583: F, t76587: F, t76591: F, t76595: F, t76599: F, t59688: F, t59694: F, t76610: F, t76614: F, t76618: F, t76622: F, t76626: F, t901: F, t5698: F, t41935: F, t60168: F, t60173: F, t60204: F, t68502: F, t68504: F, t68506: F, t76877: F, t76880: F, t76887: F, t76890: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t77028, t77030, t77032, t77034, t77037) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1360::<F>(t17210, t5705, t21180, t4362, t17218, t4378, t48103, t68442, t68444, t68446, t68448, t68452, t68454, t68494, t68498, t68500);
        let (t77041, t77042, t77058) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1361::<F>(t5705, t2815, t41904, t47787, t59657, t68442, t76574, t76578, t76583, t76587, t76591, t76595, t76599);
        let t77071 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1362::<F>(t59688, t59694, t68444, t68446, t68448, t68494, t68498, t76610, t76614, t76618, t76622, t76626);
        let (t77072, t77073, t77075, t77076, t77082) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1363::<F>(t77058, t77071, t901, t5698, t41935, t59657, t60168, t60173, t60204, t68502, t68504, t68506, t76877, t76880, t76887, t76890, t77042);
    (t77028, t77030, t77032, t77034, t77037, t77041, t77042, t77072, t77073, t77075, t77076, t77082)
}
