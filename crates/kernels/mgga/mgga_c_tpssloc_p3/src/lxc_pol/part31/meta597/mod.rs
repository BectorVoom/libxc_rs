//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta597<F: Float>(t22549: F, t92047: F, t2031: F, t90094: F, t26009: F, t84219: F, t90247: F, t111: F, t26966: F, t86588: F, t86590: F, t2094: F, t40611: F) -> (F, F, F, F, F, F, F, F) {
        let (t92049, t92052, t92056, t92057, t92090, t92122, t92123, t92169) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1842::<F>(t22549, t92047, t2031, t90094, t26009, t84219, t90247, t111, t26966, t86588, t86590, t2094, t40611);
    (t92049, t92052, t92056, t92057, t92090, t92122, t92123, t92169)
}
