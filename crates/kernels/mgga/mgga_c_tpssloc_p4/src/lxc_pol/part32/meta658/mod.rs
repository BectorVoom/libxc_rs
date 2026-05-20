//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta658 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta658<F: Float>(t90659: F, t90663: F, t90837: F, t90868: F, t90900: F, t90980: F, t90993: F, t91000: F, t91149: F, t91167: F, t91305: F, t91312: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93445, t93446, t93517, t93538, t93563, t93595, t93605, t93615, t93650, t93656, t93721, t93723) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2087::<F>(t90659, t90663, t90837, t90868, t90900, t90980, t90993, t91000, t91149, t91167, t91305, t91312);
    (t93445, t93446, t93517, t93538, t93563, t93595, t93605, t93615, t93650, t93656, t93721, t93723)
}
