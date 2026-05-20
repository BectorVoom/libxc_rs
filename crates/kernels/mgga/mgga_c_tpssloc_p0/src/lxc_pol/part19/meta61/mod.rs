//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta61 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk388;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk389;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk390;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk391;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk392;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta61<F: Float>(t496: F, t68: F, t1011: F, t1209: F, t1206: F, t1215: F, t491: F, t357: F, t475: F, t1235: F, t493: F, t1201: F, t470: F, t494: F, t1191: F, t1236: F, t1238: F, t498: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1239, t1241, t1243, t1244) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk388::<F>(t496, t68, t1011, t1209, t1206);
        let (t1245, t1246) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk389::<F>(t1215, t491, t357, t475);
        let (t1247, t1249, t1251) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk390::<F>(t1245, t1246, t1235, t493, t1201, t1244, t470, t494);
        let t1252 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk391::<F>(t1241, t1251);
        let t1254 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk392::<F>(t1191, t1236, t1238, t1252, t498);
    (t1239, t1241, t1243, t1244, t1245, t1246, t1247, t1249, t1251, t1252, t1254)
}
