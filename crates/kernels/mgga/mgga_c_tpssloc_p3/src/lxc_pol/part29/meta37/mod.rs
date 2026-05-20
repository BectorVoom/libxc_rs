//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta37 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk266;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta37<F: Float>(t40: F, t52: F, t185: F, t607: F, t707: F, t73: F, t76: F, t145: F, t164: F, t159: F, t688: F, t690: F, t694: F, t699: F, zeta_threshold: F, t167: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t708, t710, t717, t718, t719, t723, t724, t725, t730) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk266::<F>(t40, t52, t185, t607, t707, t73, t76, t145, t164, t159, t688, t690, t694, t699, zeta_threshold);
        let t731 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk267::<F>(t167);
    (t708, t710, t717, t718, t719, t723, t724, t725, t730, t731)
}
