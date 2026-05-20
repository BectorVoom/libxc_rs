//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta134 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk710;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta134<F: Float>(t3584: F, t61: F, t1236: F, t225: F, t1239: F, t496: F, t68: F, t1243: F, t3534: F, t3032: F, t3502: F, t3499: F) -> (F, F, F, F, F, F) {
        let (t3585, t3593, t3598, t3604, t3609, t3610) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk710::<F>(t3584, t61, t1236, t225, t1239, t496, t68, t1243, t3534, t3032, t3502, t3499);
    (t3585, t3593, t3598, t3604, t3609, t3610)
}
