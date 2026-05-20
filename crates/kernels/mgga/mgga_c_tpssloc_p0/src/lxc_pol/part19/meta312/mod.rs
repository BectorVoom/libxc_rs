//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta312<F: Float>(t1294: F, t39344: F, t9810: F, t9844: F, t39321: F, t12458: F, t12461: F, t677: F, t9713: F, t3684: F, t181: F, t2558: F, t686: F) -> (F, F, F, F, F, F, F) {
        let (t39346, t39347, t39349, t39350, t39354, t39356, t39358) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1112::<F>(t1294, t39344, t9810, t9844, t39321, t12458, t12461, t677, t9713, t3684, t181, t2558, t686);
    (t39346, t39347, t39349, t39350, t39354, t39356, t39358)
}
