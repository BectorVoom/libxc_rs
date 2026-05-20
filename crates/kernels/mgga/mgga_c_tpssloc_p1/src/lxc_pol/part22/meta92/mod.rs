//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta92 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk637;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk638;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk639;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk640;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk641;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta92<F: Float>(t701: F, t141: F, t2411: F, t138: F, t681: F, t125: F, t702: F, t118: F, t142: F, t2393: F, t706: F, t717: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2412 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk637::<F>(t701);
        let (t2413, t2414) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk638::<F>(t141);
        let (t2415, t2417) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk639::<F>(t2412, t2414, t2411);
        let (t2418, t2419, t2420, t2421, t2423) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk640::<F>(t138, t681, t125, t2412, t702);
        let t2426 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk641::<F>(t118, t142, t2393);
        let t2427 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk642::<F>(t706, t717);
    (t2412, t2413, t2414, t2415, t2417, t2418, t2419, t2420, t2421, t2423, t2426, t2427)
}
