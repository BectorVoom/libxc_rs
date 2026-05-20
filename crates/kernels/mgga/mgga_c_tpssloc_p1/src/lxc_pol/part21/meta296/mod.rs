//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta296 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1616;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta296<F: Float>(t248: F, t3041: F, t3101: F, t3039: F, t3108: F, t3113: F, t10889: F, t3128: F, t3033: F, t3121: F, t1020: F, t698: F, t999: F, t973: F, t2960: F, t3139: F, t1030: F, t363: F, t3068: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10895, t10896, t10898, t10903, t10904) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1616::<F>(t248, t3041, t3101, t3039, t3108, t3113, t10889, t3128, t3033);
        let (t10908, t10909, t10922, t10923, t10927, t10935, t10936) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1617::<F>(t248, t3101, t3121, t1020, t698, t999, t973, t2960, t3139, t1030, t363, t3068);
    (t10895, t10896, t10898, t10903, t10904, t10908, t10909, t10922, t10923, t10927, t10935, t10936)
}
