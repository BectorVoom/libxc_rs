//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2090;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2091;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta488<F: Float>(t16758: F, t829: F, t4234: F, t4282: F, t5550: F, t9573: F, t213: F, t5527: F, t221: F, t776: F, t4119: F, t4128: F, t12986: F, t13002: F, t13005: F, t13010: F, t4127: F, t9526: F, t9540: F, t9542: F, t9547: F, t9572: F) -> (F, F, F, F, F, F) {
        let (t16759, t16762, t16769, t16771, t16773, t16777) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2090::<F>(t16758, t829, t4234, t4282, t5550, t9573, t213, t5527, t221, t776, t4119, t4128);
        let t16781 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2091::<F>(t12986, t13002, t13005, t13010, t16769, t16773, t16777, t4127, t9526, t9540, t9542, t9547, t9572);
    (t16759, t16762, t16771, t16773, t16777, t16781)
}
