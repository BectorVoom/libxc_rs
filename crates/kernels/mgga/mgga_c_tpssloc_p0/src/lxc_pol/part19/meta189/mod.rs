//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta189 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta189<F: Float>(t10046: F, t218: F, t225: F, t2592: F, t2627: F, t852: F, t2633: F, t235: F, t860: F, t9958: F, t2679: F, t2732: F) -> (F, F, F, F, F, F, F) {
        let (t10047, t10049, t10054, t10055, t10058, t10069, t10073) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk847::<F>(t10046, t218, t225, t2592, t2627, t852, t2633, t235, t860, t9958, t2679, t2732);
    (t10047, t10049, t10054, t10055, t10058, t10069, t10073)
}
