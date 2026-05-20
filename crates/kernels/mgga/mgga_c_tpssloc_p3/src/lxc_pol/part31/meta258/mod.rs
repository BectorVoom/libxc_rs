//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta258 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1079;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1080;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta258<F: Float>(t2075: F, t671: F, t6548: F, t6564: F, t2047: F, t798: F, t6579: F, t6586: F, t6602: F, t6617: F, t6582: F, t6594: F, t6607: F, t6610: F, t6615: F, t6622: F, t218: F, t2048: F, t225: F, t2053: F, t2718: F, t865: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7061, t7067, t7069, t7072, t7074, t7076, t7078, t7082, t7084) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1079::<F>(t2075, t671, t6548, t6564, t2047, t798, t6579, t6586, t6602, t6617, t6582, t6594, t6607, t6610, t6615, t6622);
        let (t7085, t7087) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1080::<F>(t218, t7084, t2048, t225);
        let t7092 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1081::<F>(t2053, t2718, t865);
    (t7061, t7067, t7069, t7072, t7074, t7076, t7078, t7082, t7084, t7085, t7087, t7092)
}
