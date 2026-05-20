//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1425;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta276<F: Float>(t1406: F, t2239: F, t1409: F, t9321: F, t2291: F, t3966: F, t584: F, t9212: F, t9330: F, t2298: F, t2267: F, t2274: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12571, t12595, t12598, t12603, t12604, t12609, t12612, t12680, t12698) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1425::<F>(t1406, t2239, t1409, t9321, t2291, t3966, t584, t9212, t9330, t2298, t2267, t2274);
    (t12571, t12595, t12598, t12603, t12604, t12609, t12612, t12680, t12698)
}
