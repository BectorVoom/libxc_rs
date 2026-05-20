//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2122;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta600<F: Float>(t1580: F, t2930: F, t2885: F, t4408: F, t47705: F, t47707: F, t47730: F, t10632: F, t4471: F, t48096: F, t2904: F, t4446: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48783, t48789, t48799, t48800, t48809, t48890, t48919, t48924, t48946, t48947, t48956, t49096) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2122::<F>(t1580, t2930, t2885, t4408, t47705, t47707, t47730, t10632, t4471, t48096, t2904, t4446);
    (t48783, t48789, t48799, t48800, t48809, t48890, t48919, t48924, t48946, t48947, t48956, t49096)
}
