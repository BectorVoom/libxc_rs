//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2063;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta559<F: Float>(t2289: F, t2769: F, t41654: F, t10629: F, t938: F, t2903: F, t2928: F, t315: F, t909: F, t9709: F, t10213: F, t241: F) -> (F, F, F, F, F, F, F) {
        let (t41687, t41741, t41821, t41825, t41826, t41863, t41880) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2063::<F>(t2289, t2769, t41654, t10629, t938, t2903, t2928, t315, t909, t9709, t10213, t241);
    (t41687, t41741, t41821, t41825, t41826, t41863, t41880)
}
