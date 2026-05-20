//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta329<F: Float>(t11588: F, t1184: F, t3451: F, t3447: F, t3448: F, t3475: F, t1239: F, t68: F, t225: F, t3484: F, t1222: F, t3567: F) -> (F, F, F, F, F, F, F, F) {
        let (t11589, t11591, t11593, t11604, t11605, t11606, t11613, t11642) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1387::<F>(t11588, t1184, t3451, t3447, t3448, t3475, t1239, t68, t225, t3484, t1222, t3567);
    (t11589, t11591, t11593, t11604, t11605, t11606, t11613, t11642)
}
