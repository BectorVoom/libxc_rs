//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta92<F: Float>(t218: F, t2710: F, t225: F, t853: F, t257: F, t856: F, t68: F, t865: F, t252: F, t2627: F, t2633: F, t814: F, t852: F) -> (F, F, F, F, F, F, F, F) {
        let (t2711, t2713, t2718, t2719, t2720, t2728, t2729, t2732) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk523::<F>(t218, t2710, t225, t853, t257, t856, t68, t865, t252, t2627, t2633, t814, t852);
    (t2711, t2713, t2718, t2719, t2720, t2728, t2729, t2732)
}
