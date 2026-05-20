//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta612<F: Float>(t11282: F, t1687: F, t1682: F, t3357: F, t1694: F, t3401: F, t11420: F, t3312: F, t4737: F, t11419: F, t1675: F, t50826: F) -> (F, F, F, F, F, F, F) {
        let (t51376, t51382, t51389, t51392, t51402, t51427, t51550) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2140::<F>(t11282, t1687, t1682, t3357, t1694, t3401, t11420, t3312, t4737, t11419, t1675, t50826);
    (t51376, t51382, t51389, t51392, t51402, t51427, t51550)
}
