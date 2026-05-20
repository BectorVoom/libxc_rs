//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1776;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1777;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta436<F: Float>(t25: F, t6320: F, t67: F, t758: F, t12061: F, t6305: F, t3664: F, t5397: F, t16557: F, t2219: F, t5134: F, t514: F, t606: F, zeta_threshold: F, t28: F, t12072: F, t6312: F, t3672: F, t5966: F, t1081: F, t18196: F, t5142: F, t517: F, t157: F) -> (F, F, F, F, F, F) {
        let (t19541, t19542, t19543, t19547, t19558) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1776::<F>(t25, t6320, t67, t758, t12061, t6305, t3664, t5397, t16557, t2219, t5134, t514, t606, zeta_threshold);
        let (t19559, t19572) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1777::<F>(t28, t12072, t6312, t3672, t5966, t1081, t18196, t2219, t5142, t517, t157, t19558, zeta_threshold);
    (t19541, t19542, t19543, t19547, t19559, t19572)
}
