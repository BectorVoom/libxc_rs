//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1426;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta277<F: Float>(t111: F, t4025: F, t1454: F, t2281: F, t4044: F, t626: F, t4068: F, t2331: F, t4067: F, t2341: F, t92: F, t100: F, t2349: F) -> (F, F, F, F, F, F, F) {
        let (t12725, t12747, t12750, t12752, t12757, t12774, t12795) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1426::<F>(t111, t4025, t1454, t2281, t4044, t626, t4068, t2331, t4067, t2341, t92, t100, t2349);
    (t12725, t12747, t12750, t12752, t12757, t12774, t12795)
}
