//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1702;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta445<F: Float>(t22724: F, t6973: F, t6982: F, t794: F, t6897: F, t6883: F, t6983: F, t6914: F, t6979: F, t6546: F, t6887: F) -> (F, F, F, F, F, F) {
        let (t22726, t22727, t22728, t22730, t22745, t22751) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1702::<F>(t22724, t6973, t6982, t794, t6897, t6883, t6983, t6914, t6979, t6546, t6887);
    (t22726, t22727, t22728, t22730, t22745, t22751)
}
