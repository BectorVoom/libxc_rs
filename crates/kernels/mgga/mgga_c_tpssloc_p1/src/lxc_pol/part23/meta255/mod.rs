//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta255<F: Float>(t5392: F, t9321: F, t9330: F, t111: F, t5449: F, t5465: F, t626: F, t5464: F, t9365: F, t5489: F, t5468: F, t9384: F) -> (F, F, F, F, F, F, F) {
        let (t19420, t19430, t19451, t19471, t19473, t19480, t19488) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk916::<F>(t5392, t9321, t9330, t111, t5449, t5465, t626, t5464, t9365, t5489, t5468, t9384);
    (t19420, t19430, t19451, t19471, t19473, t19480, t19488)
}
