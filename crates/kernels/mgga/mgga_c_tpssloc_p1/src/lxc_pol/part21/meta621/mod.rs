//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2398;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2399;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta621<F: Float>(t3809: F, t40281: F, t12267: F, t3865: F, t12344: F, t3777: F, t1369: F, t12250: F, t3850: F, t10021: F, t154: F, t59: F, t3749: F, t598: F, t535: F, t795: F, t215: F, t39933: F, t12227: F, t9577: F, t116: F, t557: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40282, t40284, t40292, t40293, t40335, t40341) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2398::<F>(t3809, t40281, t12267, t3865, t12344, t3777, t1369, t12250, t3850, t10021, t154, t59);
        let (t40343, t40344, t40347, t40350, t40351, t40353) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2399::<F>(t3749, t40341, t59, t598, t535, t795, t215, t39933, t12227, t9577, t116, t557);
    (t40282, t40284, t40292, t40293, t40335, t40341, t40343, t40344, t40347, t40350, t40351, t40353)
}
