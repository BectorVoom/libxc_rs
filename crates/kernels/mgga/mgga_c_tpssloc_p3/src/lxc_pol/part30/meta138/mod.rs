//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta138 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta138<F: Float>(t1932: F, t3508: F, t1209: F, t3032: F, t3499: F, t475: F, t500: F, t526: F, t528: F, t118: F, t521: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3612, t3623, t3624, t3625, t3639, t3640, t3664, t3672, t3684) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk766::<F>(t1932, t3508, t1209, t3032, t3499, t475, t500, t526, t528, t118, t521);
    (t3612, t3623, t3624, t3625, t3639, t3640, t3664, t3672, t3684)
}
