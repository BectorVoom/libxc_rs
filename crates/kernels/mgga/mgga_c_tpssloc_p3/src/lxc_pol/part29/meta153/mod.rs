//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta153 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta153<F: Float>(t1137: F, t3333: F, t3236: F, t3293: F, t3238: F, t3245: F, t3250: F, t3254: F, t3272: F, t3280: F, t3288: F, t3290: F, t3295: F, t3299: F, t3302: F, t3305: F) -> (F, F, F, F) {
        let (t3334, t3339, t3346, t3351) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk824::<F>(t1137, t3333, t3236, t3293, t3238, t3245, t3250, t3254, t3272, t3280, t3288, t3290, t3295, t3299, t3302, t3305);
    (t3334, t3339, t3346, t3351)
}
