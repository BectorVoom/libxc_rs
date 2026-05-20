//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1730;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta456<F: Float>(t23041: F, t831: F, t2627: F, t59: F, t240: F, t812: F, t2617: F, t6613: F, t1878: F, t244: F, t2230: F, t6589: F, t213: F, t6593: F, t229: F, t6546: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t23042, t23046, t23047, t23048, t23053, t23056, t23061) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1730::<F>(t23041, t831, t2627, t59, t240, t812, t2617, t6613, t1878, t244, t2230, t6589);
        let (t23062, t23063, t23069) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1731::<F>(t213, t23061, t6593, t229, t6546);
    (t23042, t23046, t23047, t23048, t23053, t23056, t23061, t23062, t23063, t23069)
}
