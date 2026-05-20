//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta250 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1364;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1365;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta250<F: Float>(t1030: F, t3036: F, t1015: F, t3033: F, t3128: F, t698: F, t999: F, t973: F, t10277: F, t2978: F, t363: F, t3068: F, t1058: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10890, t10891, t10903, t10904, t10923, t10930, t10935, t10936) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1364::<F>(t1030, t3036, t1015, t3033, t3128, t698, t999, t973, t10277, t2978, t363, t3068);
        let t10937 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1365::<F>(t1058, t10936);
    (t10890, t10891, t10903, t10904, t10923, t10930, t10935, t10936, t10937)
}
