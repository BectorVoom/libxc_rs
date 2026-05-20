//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta16 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk128;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk129;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk130;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk131;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk132;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta16<F: Float>(t134: F, t241: F, t271: F, t281: F, t273: F, t276: F, t279: F, t275: F, t148: F, t154: F, t157: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t282, t283) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk128::<F>(t134, t241, t271);
        let t285 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk129::<F>(t281, t282, t283);
        let (t287, t290, t291) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk130::<F>(t273, t276, t279, t285);
        let (t293, t300) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk131::<F>(t275, t291, t148, t154, t157, zeta_threshold);
        let t302 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk132::<F>(t273);
        let (t307, t310, t311) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk133::<F>(t273, t276, t279, t285);
    (t282, t283, t285, t287, t290, t291, t293, t300, t302, t307, t310, t311)
}
