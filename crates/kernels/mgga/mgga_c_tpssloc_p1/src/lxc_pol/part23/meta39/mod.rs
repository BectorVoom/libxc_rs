//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta39 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk275;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk276;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk277;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk278;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk279;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta39<F: Float>(t257: F, t68: F, t252: F, t814: F, t261: F, t154: F, t676: F, t268: F, t271: F, t376: F, t632: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t856, t858, t860) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk275::<F>(t257, t68, t252, t814);
        let t870 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk276::<F>(t261);
        let (t878, t880) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk277::<F>(t154, t676, t268, t271);
        let (t881, t882) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk278::<F>(t880, t154, t376);
        let t883 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk279::<F>(t632);
    (t856, t858, t860, t870, t878, t880, t881, t882, t883)
}
