//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta453<F: Float>(t6652: F, t794: F, t6562: F, t6547: F, t6653: F, t22723: F, t6561: F) -> (F, F, F, F, F) {
        let (t23025, t23026, t23028, t23029, t23030) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1771::<F>(t6652, t794, t6562, t6547, t6653, t22723, t6561);
    (t23025, t23026, t23028, t23029, t23030)
}
