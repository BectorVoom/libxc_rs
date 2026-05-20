//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2044;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta547<F: Float>(t2221: F, t3826: F, t12132: F, t592: F, t1336: F, t1339: F, t2691: F, t12344: F, t3777: F, t10021: F, t154: F, t59: F, t3749: F, t598: F, t535: F, t795: F, t215: F, t39933: F, t12227: F, t9577: F, t116: F, t557: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40225, t40230, t40281, t40292, t40341) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2044::<F>(t2221, t3826, t12132, t592, t1336, t1339, t2691, t12344, t3777, t10021, t154, t59);
        let (t40343, t40344, t40347, t40350, t40351, t40353) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2045::<F>(t3749, t40341, t59, t598, t535, t795, t215, t39933, t12227, t9577, t116, t557);
    (t40225, t40230, t40281, t40292, t40341, t40343, t40344, t40347, t40350, t40351, t40353)
}
