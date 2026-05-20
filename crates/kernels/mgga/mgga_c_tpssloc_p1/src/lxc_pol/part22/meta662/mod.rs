//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta662<F: Float>(t16985: F, t2697: F, t1516: F, t47275: F, t47278: F, t5628: F, t9601: F, t5619: F, t9671: F, t16853: F, t16673: F, t2638: F) -> (F, F, F, F, F, F, F) {
        let (t59257, t59259, t59261, t59263, t59276, t59279, t59281) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2207::<F>(t16985, t2697, t1516, t47275, t47278, t5628, t9601, t5619, t9671, t16853, t16673, t2638);
    (t59257, t59259, t59261, t59263, t59276, t59279, t59281)
}
