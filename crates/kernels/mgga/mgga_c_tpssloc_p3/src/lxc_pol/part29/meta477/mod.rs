//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta477<F: Float>(t24574: F, t7303: F, t7291: F, t1251: F, t7391: F, t3598: F, t2123: F, t3427: F, t2121: F, t221: F, t3448: F, t2127: F) -> (F, F, F, F, F, F, F) {
        let (t24575, t24577, t24582, t24585, t24587, t24588, t24589) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1815::<F>(t24574, t7303, t7291, t1251, t7391, t3598, t2123, t3427, t2121, t221, t3448, t2127);
    (t24575, t24577, t24582, t24585, t24587, t24588, t24589)
}
