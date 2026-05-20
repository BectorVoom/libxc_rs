//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1111;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1112;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1113;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta338<F: Float>(t1294: F, t39344: F, t9810: F, t9844: F, t39321: F, t677: F, t9713: F, t3684: F, t181: F, t2558: F, t686: F, t1291: F, t2369: F, t9720: F, t9843: F, t2411: F, t2414: F, t39246: F, t2508: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39346, t39347, t39349, t39354, t39356, t39358, t39360) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1111::<F>(t1294, t39344, t9810, t9844, t39321, t677, t9713, t3684, t181, t2558, t686, t1291);
        let (t39362, t39364, t39373) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1112::<F>(t2369, t9720, t9843, t1294, t2411, t2414, t39246);
        let (t39377, t39378) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1113::<F>(t2508, t2369);
    (t39346, t39347, t39349, t39354, t39356, t39358, t39360, t39362, t39364, t39373, t39377, t39378)
}
