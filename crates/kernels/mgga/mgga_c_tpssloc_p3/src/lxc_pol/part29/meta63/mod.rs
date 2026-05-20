//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta63 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk422;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk423;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk424;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk425;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk426;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk427;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk428;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk429;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk430;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk431;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta63<F: Float>(t1207: F, t1212: F, t486: F, t61: F, t1096: F, t1121: F, t1161: F, t1163: F, t1168: F, t475: F, t248: F, t122: F, t374: F, t485: F, t372: F, t483: F, t479: F, t471: F, t404: F, t415: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1213 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk422::<F>(t1207, t1212);
        let t1214 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk423::<F>(t486, t61);
        let t1215 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk424::<F>(t1096, t1121, t1161, t1163, t1168);
        let t1216 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk425::<F>(t1215, t475);
        let t1218 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk426::<F>(t1214, t1216, t248);
        let t1222 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk427::<F>(t122, t374, t486);
        let (t1224, t1225, t1226) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk428::<F>(t1222, t485, t372, t483, t479);
        let t1227 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk429::<F>(t1226, t471);
        let t1229 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk430::<F>(t404, t415);
        let t1230 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk431::<F>(t1229, t61);
    (t1213, t1214, t1215, t1216, t1218, t1222, t1224, t1225, t1226, t1227, t1229, t1230)
}
