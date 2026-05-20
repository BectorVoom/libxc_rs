//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1958;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1959;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta579<F: Float>(t2140: F, t6169: F, t1748: F, t27611: F, t27617: F, t27622: F, t27629: F, t27684: F, t27711: F, t29585: F, t29594: F, t29597: F, t29601: F, t467: F, t488: F, t7326: F, t8040: F, t460: F, t6144: F, t7320: F, t6138: F, t2134: F, t24729: F, t24733: F, t24741: F, t27604: F, t27626: F, t27651: F, t6192: F, t6221: F, t6227: F, t6232: F, t7339: F, t8028: F, t8031: F, t8035: F) -> (F, F, F, F, F, F, F) {
        let (t29606, t29610) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1958::<F>(t2140, t6169, t1748, t27611, t27617, t27622, t27629, t27684, t27711, t29585, t29594, t29597, t29601, t467, t488, t7326, t8040);
        let (t29614, t29615, t29624, t29625, t29636) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1959::<F>(t460, t6144, t7320, t6138, t1748, t2134, t24729, t24733, t24741, t27604, t27626, t27651, t6192, t6221, t6227, t6232, t7339, t8028, t8031, t8035);
    (t29606, t29610, t29614, t29615, t29624, t29625, t29636)
}
