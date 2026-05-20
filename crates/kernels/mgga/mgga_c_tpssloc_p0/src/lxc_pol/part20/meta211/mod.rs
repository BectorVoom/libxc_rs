//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta211 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1251;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1252;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1253;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta211<F: Float>(t25: F, t28: F, t17: F, t5168: F, t1408: F, t3704: F, t1298: F, t2: F, t584: F, t606: F, t1649: F, t3711: F, t1302: F, t1081: F, zeta_threshold: F, t1804: F, t3726: F, t131: F, t3732: F, t205: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t5169, t5170, t5173, t5177, t5178, t5181, t5185) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1251::<F>(t25, t28, t17, t5168, t1408, t3704, t1298, t2, t584, t606, t1649, t3711, t1302, t1081, zeta_threshold);
        let t5187 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1252::<F>(t5177, t5185);
        let (t5192, t5194, t5195) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1253::<F>(t1804, t3726, t131, t3732, t205);
    (t5169, t5170, t5173, t5178, t5181, t5187, t5192, t5194, t5195)
}
