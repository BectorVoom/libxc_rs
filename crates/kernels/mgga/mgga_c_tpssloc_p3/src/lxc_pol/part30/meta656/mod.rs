//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2073;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta656<F: Float>(t46104: F, t6489: F, t12571: F, t22522: F, t26083: F, t9239: F, t645: F, t7445: F, t1863: F, t22550: F, t7441: F, t9231: F, t2240: F, t26043: F, t33: F, t45844: F, t111: F, t26097: F, t26351: F, t6883: F, t22751: F, t26186: F, t26190: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90182, t90185, t90192, t90248, t90251, t90308) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2073::<F>(t46104, t6489, t12571, t22522, t26083, t9239, t645, t7445, t1863, t22550, t7441, t9231);
        let (t90312, t90330, t90400, t90460, t90469, t90470) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2074::<F>(t2240, t26043, t33, t45844, t6489, t111, t26097, t26351, t6883, t22751, t26186, t26190);
    (t90182, t90185, t90192, t90248, t90251, t90308, t90312, t90330, t90400, t90460, t90469, t90470)
}
