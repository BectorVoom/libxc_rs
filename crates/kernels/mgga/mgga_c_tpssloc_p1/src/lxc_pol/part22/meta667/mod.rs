//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2222;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta667<F: Float>(t2403: F, t5720: F, t5723: F, t17246: F, t699: F, t17249: F, t17252: F, t5717: F, t17255: F, t17279: F, t17240: F, t17243: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t60168, t60173, t60192, t60194, t60202, t60204, t60274, t60308, t60310, t60312) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2222::<F>(t2403, t5720, t5723, t17246, t699, t17249, t17252, t5717, t17255, t17279, t17240, t17243);
    (t60168, t60173, t60192, t60194, t60202, t60204, t60274, t60308, t60310, t60312)
}
