//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1138;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1139;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta321<F: Float>(t2225: F, t3824: F, t1287: F, t9214: F, t12129: F, t588: F, t39033: F, t522: F, t39035: F, t39031: F, t1285: F, t9216: F, t9218: F, t16: F, t185: F, t520: F, t1284: F, t17: F, t9861: F, t3719: F, t12012: F, t12303: F, t193: F, t3918: F, t3919: F, t3924: F, t39590: F, t39593: F, t5126: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39595, t39597, t39602, t39604, t39606, t39608, t39609) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1138::<F>(t2225, t3824, t1287, t9214, t12129, t588, t39033, t522, t39035, t39031, t1285, t9216);
        let (t39610, t39612, t39615, t39621, t39622) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1139::<F>(t39609, t1285, t9218, t16, t185, t520, t1284, t17, t9861, t3719);
        let t39626 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1140::<F>(t12012, t12303, t193, t3918, t3919, t3924, t39590, t39593, t39595, t39597, t39602, t39604, t39606, t39608, t39610, t39612, t39615, t39621, t39622, t5126);
    (t39595, t39597, t39602, t39604, t39606, t39608, t39610, t39612, t39615, t39621, t39622, t39626)
}
