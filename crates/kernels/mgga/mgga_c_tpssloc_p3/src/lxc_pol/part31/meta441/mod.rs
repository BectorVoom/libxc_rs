//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1586;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1587;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta441<F: Float>(t22468: F, t111: F, t7039: F, t2094: F, t531: F, t7025: F, t9239: F, t33: F, t625: F, t2240: F) -> (F, F, F, F, F, F) {
        let (t23912, t23938) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1586::<F>(t22468, t111, t7039);
        let (t23957, t23963) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1587::<F>(t2094, t531, t7025, t9239);
        let (t23966, t23967) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1588::<F>(t33, t625, t2240);
    (t23912, t23938, t23957, t23963, t23966, t23967)
}
