//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta103 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta103<F: Float>(t706: F, t717: F, t607: F, t751: F, t707: F, t195: F, t197: F, t676: F, t724: F, t164: F, t723: F, t159: F) -> (F, F, F, F, F, F, F, F) {
        let (t2427, t2430, t2431, t2433, t2440, t2454, t2459, t2460) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk619::<F>(t706, t717, t607, t751, t707, t195, t197, t676, t724, t164, t723, t159);
    (t2427, t2430, t2431, t2433, t2440, t2454, t2459, t2460)
}
