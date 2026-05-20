//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2013;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta454<F: Float>(t14687: F, t15856: F, t3701: F, t5356: F, t3719: F, t5127: F, t5168: F, t588: F, t592: F, t5166: F, t5187: F, t571: F) -> (F, F, F, F, F, F, F, F) {
        let (t15857, t15868, t15872, t15876, t15877, t15878, t15880, t15883) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2013::<F>(t14687, t15856, t3701, t5356, t3719, t5127, t5168, t588, t592, t5166, t5187, t571);
    (t15857, t15868, t15872, t15876, t15877, t15878, t15880, t15883)
}
