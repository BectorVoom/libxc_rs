//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1457;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1458;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1459;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1460;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1461;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta388<F: Float>(t3502: F, t42341: F, t44696: F, t23508: F, t3508: F, t225: F, t44657: F, t1209: F, t475: F, t43670: F, t43672: F, t43674: F, t43678: F, t43683: F, t43685: F, t43687: F, t43695: F, t43702: F, t43915: F, t43924: F, t43953: F, t43956: F, t43958: F, t43961: F, t43963: F, t43966: F, t43973: F, t43975: F, t43979: F, t43982: F, t43987: F, t43989: F, t43994: F, t43997: F, t44000: F, t44002: F, t44006: F, t44072: F, t44080: F, t44082: F, t44085: F, t44089: F, t44092: F, t44369: F, t44161: F, t44164: F, t44167: F, t44358: F, t44372: F, t44375: F, t44377: F, t44384: F, t44388: F, t44392: F, t44396: F, t44400: F, t1174: F, t11765: F, t135: F, t43763: F, t44620: F, t3551: F, t698: F, t11545: F, t43791: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44753, t44754, t44774, t44785, t44786, t44792) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1457::<F>(t3502, t42341, t44696, t23508, t3508, t225, t44657, t1209, t475, t43670, t43672, t43674, t43678, t43683, t43685, t43687, t43695, t43702, t43915, t43924);
        let t44793 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1458::<F>(t43953, t43956, t43958, t43961, t43963, t43966, t43973, t43975, t43979, t43982, t43987, t43989);
        let t44795 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1459::<F>(t43994, t43997, t44000, t44002, t44006, t44072, t44080, t44082, t44085, t44089, t44092, t44369);
        let t44796 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1460::<F>(t44161, t44164, t44167, t44358, t44372, t44375, t44377, t44384, t44388, t44392, t44396, t44400);
        let (t44798, t44803, t44805, t44811, t44817) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1461::<F>(t44792, t44793, t44795, t44796, t1174, t11765, t135, t43763, t44620, t3551, t698, t11545, t43791);
    (t44753, t44754, t44774, t44785, t44786, t44798, t44803, t44805, t44811, t44817)
}
