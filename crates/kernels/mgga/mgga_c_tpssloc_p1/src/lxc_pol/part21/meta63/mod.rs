//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta63 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk454;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk455;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk456;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk457;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk458;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk459;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk460;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk461;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk462;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk463;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta63<F: Float>(t1235: F, t466: F, t225: F, t492: F, t496: F, t68: F, t1011: F, t1209: F, t1206: F, t1215: F, t491: F, t357: F, t475: F, t493: F, t1201: F, t470: F, t494: F, t1191: F, t498: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1236, t1238) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk454::<F>(t1235, t466, t225, t492);
        let (t1239, t1241) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk455::<F>(t496, t68);
        let t1243 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk456::<F>(t1011, t1209);
        let t1244 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk457::<F>(t1206, t1243);
        let (t1245, t1246) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk458::<F>(t1215, t491, t357, t475);
        let t1247 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk459::<F>(t1245, t1246);
        let t1249 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk460::<F>(t1235, t493);
        let t1251 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk461::<F>(t1201, t1244, t1247, t1249, t470, t494);
        let t1252 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk462::<F>(t1241, t1251);
        let t1254 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk463::<F>(t1191, t1236, t1238, t1252, t498);
    (t1236, t1238, t1239, t1241, t1243, t1244, t1246, t1247, t1249, t1251, t1252, t1254)
}
