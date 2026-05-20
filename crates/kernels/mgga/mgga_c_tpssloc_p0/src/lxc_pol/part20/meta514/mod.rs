//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2039;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta514<F: Float>(t2512: F, t39378: F, t9489: F, t1294: F, t12088: F, t2371: F, t2509: F, t39389: F, t763: F, t9697: F, t3684: F, t2393: F) -> (F, F, F, F, F, F, F, F) {
        let (t39488, t39490, t39491, t39494, t39496, t39497, t39499, t39500) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2039::<F>(t2512, t39378, t9489, t1294, t12088, t2371, t2509, t39389, t763, t9697, t3684, t2393);
    (t39488, t39490, t39491, t39494, t39496, t39497, t39499, t39500)
}
