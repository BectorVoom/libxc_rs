//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta689 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta689<F: Float>(t15402: F, t18225: F, t3447: F, t11589: F, t18427: F, t18221: F, t15376: F, t15399: F, t15403: F, t18409: F, t15339: F, t15419: F, t18232: F) -> (F, F, F, F, F, F, F, F) {
        let (t64696, t64699, t64702, t64711, t64713, t64718, t64730, t64733) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2267::<F>(t15402, t18225, t3447, t11589, t18427, t18221, t15376, t15399, t15403, t18409, t15339, t15419, t18232);
    (t64696, t64699, t64702, t64711, t64713, t64718, t64730, t64733)
}
