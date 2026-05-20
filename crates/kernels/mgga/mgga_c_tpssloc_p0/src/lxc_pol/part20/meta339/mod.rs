//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1637;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1638;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta339<F: Float>(t25: F, t514: F, t3665: F, t606: F, t3704: F, t1298: F, t2249: F, t9257: F, t28: F, t517: F, t1081: F, t3673: F, zeta_threshold: F, t3711: F, t11122: F, t1302: F, t3231: F) -> (F, F, F, F, F, F, F) {
        let (t11985, t11987, t11988, t11997, t11998, t12000, t12001) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1637::<F>(t25, t514, t3665, t606, t3704, t1298, t2249, t9257, t28, t517, t1081, t3673, zeta_threshold);
        let t12012 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1638::<F>(t28, t1081, t3711, t11122, t12000, t12001, t1302, t3231, t11997, zeta_threshold);
    (t11985, t11987, t11988, t11998, t12000, t12001, t12012)
}
