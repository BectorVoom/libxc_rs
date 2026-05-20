//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta60 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk383;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk384;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk385;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk386;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta60<F: Float>(t1214: F, t1216: F, t248: F, t122: F, t374: F, t486: F, t485: F, t372: F, t483: F, t479: F, t471: F, t404: F, t415: F, t61: F, t1090: F, t1174: F, t1195: F, t1198: F, t1203: F, t1213: F, t488: F, t466: F, t225: F, t492: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1218, t1222, t1224, t1226, t1227) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk383::<F>(t1214, t1216, t248, t122, t374, t486, t485, t372, t483, t479, t471);
        let t1229 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk384::<F>(t404, t415);
        let t1230 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk385::<F>(t1229, t61);
        let (t1232, t1235) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk386::<F>(t1090, t1230, t248, t1174, t1195, t1198, t1203, t1213, t1218, t1224, t1227, t488);
        let (t1236, t1238) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk387::<F>(t1235, t466, t225, t492);
    (t1218, t1222, t1226, t1227, t1229, t1230, t1232, t1235, t1236, t1238)
}
