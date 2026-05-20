//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta603<F: Float>(t1568: F, t2886: F, t2860: F, t4408: F, t10770: F, t1561: F, t2791: F, t4351: F, t10660: F, t1543: F, t10756: F, t300: F) -> (F, F, F, F, F, F) {
        let (t49422, t49427, t49430, t49486, t49489, t49513) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2125::<F>(t1568, t2886, t2860, t4408, t10770, t1561, t2791, t4351, t10660, t1543, t10756, t300);
    (t49422, t49427, t49430, t49486, t49489, t49513)
}
