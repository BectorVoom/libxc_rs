//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1985;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta597<F: Float>(t22723: F, t22891: F, t22920: F, t117: F, t5247: F, t6559: F, t22675: F, t22724: F, t22716: F, t6903: F, t22684: F, t6546: F) -> (F, F, F, F, F, F) {
        let (t80670, t80671, t80681, t80711, t80722, t80727) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1985::<F>(t22723, t22891, t22920, t117, t5247, t6559, t22675, t22724, t22716, t6903, t22684, t6546);
    (t80670, t80671, t80681, t80711, t80722, t80727)
}
