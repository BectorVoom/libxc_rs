//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1218;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta405<F: Float>(t3375: F, t6063: F, t3400: F, t3312: F, t5983: F, t2403: F, t6011: F, t6014: F, t6017: F, t3356: F, t6031: F, t3263: F) -> (F, F, F, F, F, F, F, F) {
        let (t63454, t63602, t63755, t63888, t63893, t63911, t64103, t64257) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1218::<F>(t3375, t6063, t3400, t3312, t5983, t2403, t6011, t6014, t6017, t3356, t6031, t3263);
    (t63454, t63602, t63755, t63888, t63893, t63911, t64103, t64257)
}
