//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta40 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk290;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk291;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk292;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk293;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk294;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta40<F: Float>(t120: F, t212: F, t118: F, t207: F, t792: F, t785: F, t787: F, t789: F, t252: F, t154: F, t782: F, t222: F, t119: F, t776: F, t210: F, t225: F, t237: F, t226: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t794 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk290::<F>(t120, t212);
        let t795 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk291::<F>(t118, t794);
        let (t797, t798) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk292::<F>(t207, t792, t795, t785, t787, t789);
        let (t799, t801, t803, t804, t805, t808) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk293::<F>(t252, t798, t154, t782, t222, t119, t776, t210, t225);
        let (t809, t812) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk294::<F>(t237, t808, t226, t68);
    (t794, t795, t797, t798, t799, t801, t803, t804, t805, t808, t809, t812)
}
