//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta106 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk626;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk627;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk628;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk629;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta106<F: Float>(t157: F, t2516: F, t153: F, t193: F, t201: F, t868: F, t870: F, t2369: F, t2509: F, t2512: F, t761: F, t172: F, t753: F, t763: F, t2504: F, t739: F, t746: F) -> (F, F, F, F, F, F, F, F, F) {
        let t2517 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk626::<F>(t157, t2516);
        let (t2518, t2522) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk627::<F>(t153, t2517, t193, t201);
        let (t2523, t2528) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk628::<F>(t868, t870, t2369, t2509, t2512);
        let (t2530, t2531, t2532, t2535) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk629::<F>(t2528, t761, t172, t753, t763, t2504, t739, t746);
    (t2517, t2518, t2522, t2523, t2528, t2530, t2531, t2532, t2535)
}
