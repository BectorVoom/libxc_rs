//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta50 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk319;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk320;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk321;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk322;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk323;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk324;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta50<F: Float>(t122: F, t374: F, t486: F, t485: F, t372: F, t483: F, t479: F, t471: F, t404: F, t415: F, t61: F, t225: F, t492: F, t496: F, t68: F, t1011: F, t1209: F, t1206: F, t357: F, t475: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1222 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk319::<F>(t122, t374, t486);
        let (t1224, t1226, t1227) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk320::<F>(t1222, t485, t372, t483, t479, t471);
        let t1229 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk321::<F>(t404, t415);
        let t1230 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk322::<F>(t1229, t61);
        let t1238 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk323::<F>(t225, t492);
        let (t1239, t1241, t1243, t1244) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk324::<F>(t496, t68, t1011, t1209, t1206);
        let t1246 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk325::<F>(t357, t475);
    (t1222, t1224, t1226, t1227, t1229, t1230, t1238, t1239, t1241, t1243, t1244, t1246)
}
