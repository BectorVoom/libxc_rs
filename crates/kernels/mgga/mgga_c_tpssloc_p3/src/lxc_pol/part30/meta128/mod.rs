//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta128 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta128<F: Float>(t1097: F, t409: F) -> (F, F, F) {
        let (t3311, t3312, t3313) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk744::<F>(t1097, t409);
    (t3311, t3312, t3313)
}
