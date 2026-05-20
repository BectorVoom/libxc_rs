//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta144 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk796;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta144<F: Float>(t3106: F, t3165: F, t349: F, t1050: F, t225: F, t1053: F, t386: F, t68: F, t1065: F, t1057: F, t3112: F, t3032: F, t3127: F, t3031: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3166, t3167, t3169, t3174, t3175, t3176, t3180) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk796::<F>(t3106, t3165, t349, t1050, t225, t1053, t386, t68, t1065, t1057, t3112);
        let (t3185, t3186) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk797::<F>(t3032, t3127, t3031);
    (t3166, t3167, t3169, t3174, t3175, t3176, t3180, t3185, t3186)
}
