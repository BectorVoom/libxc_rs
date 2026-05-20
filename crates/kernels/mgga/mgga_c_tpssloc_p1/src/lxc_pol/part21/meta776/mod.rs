//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta776 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2685;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta776<F: Float>(t2221: F, t6328: F, t2223: F, t2225: F, t39571: F, t17: F, t2516: F, t6320: F, t19572: F, t750: F, t184: F, t56349: F, t1388: F, t5356: F, t15899: F, t39570: F, t39585: F, t39590: F, t39593: F, t5160: F) -> (F, F, F, F, F, F, F, F) {
        let (t56391, t56393, t56395, t56396, t56398, t56401, t56403) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2685::<F>(t2221, t6328, t2223, t2225, t39571, t17, t2516, t6320, t19572, t750, t184, t56349);
        let t56408 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2686::<F>(t1388, t5356, t15899, t39570, t39585, t39590, t39593, t5160, t56391, t56393, t56395, t56396, t56398, t56401, t56403);
    (t56391, t56393, t56395, t56396, t56398, t56401, t56403, t56408)
}
