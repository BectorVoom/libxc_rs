//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta108 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk678;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta108<F: Float>(t40: F, t52: F, t2427: F, t708: F, t607: F, t751: F, t707: F, t195: F, t2244: F, t2250: F, t73: F, t197: F, t76: F, t157: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
        let (t2429, t2430, t2431, t2432, t2433, t2440, t2447, t2448) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk678::<F>(t40, t52, t2427, t708, t607, t751, t707, t195, t2244, t2250, t73, t197, t76, t157, zeta_threshold);
    (t2429, t2430, t2431, t2432, t2433, t2440, t2447, t2448)
}
