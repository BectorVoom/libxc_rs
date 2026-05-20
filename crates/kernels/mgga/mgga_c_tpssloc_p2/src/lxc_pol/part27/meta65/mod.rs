//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta65 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk440;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk441;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk442;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk443;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk444;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk445;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta65<F: Float>(t1235: F, t466: F, t225: F, t492: F, t496: F, t68: F, t1011: F, t1209: F, t1206: F, t1215: F, t491: F, t357: F, t475: F, t493: F, t1201: F, t470: F, t494: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1236, t1238) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk440::<F>(t1235, t466, t225, t492);
        let (t1239, t1241) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk441::<F>(t496, t68);
        let t1243 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk442::<F>(t1011, t1209);
        let t1244 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk443::<F>(t1206, t1243);
        let (t1245, t1246) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk444::<F>(t1215, t491, t357, t475);
        let (t1247, t1249, t1251, t1252) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk445::<F>(t1245, t1246, t1235, t493, t1201, t1244, t470, t494, t1241);
    (t1236, t1238, t1239, t1241, t1243, t1244, t1246, t1247, t1249, t1251, t1252)
}
