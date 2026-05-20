//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta718 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2560;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta718<F: Float>(t10480: F, t13969: F, t13986: F, t3039: F, t4599: F, t49850: F, t10870: F, t4644: F, t10875: F, t48569: F, t10937: F, t13765: F, t10903: F, t14507: F, t14651: F, t3069: F, t10956: F, t1611: F, t10517: F, t4630: F, t10459: F, t4608: F, t698: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50255, t50258, t50262, t50265, t50272) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2560::<F>(t10480, t13969, t13986, t3039, t4599, t49850, t10870, t4644, t10875, t48569, t10937, t13765);
        let (t50302, t50324, t50334, t50337, t50343, t50361) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2561::<F>(t10903, t14507, t14651, t3069, t10956, t1611, t10517, t4630, t10459, t4644, t4608, t698, t973);
    (t50255, t50258, t50262, t50265, t50272, t50302, t50324, t50334, t50337, t50343, t50361)
}
