//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1371;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1372;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta319<F: Float>(t10947: F, t3185: F, t3199: F, t1014: F, t10471: F, t10470: F, t1057: F, t10960: F, t3120: F, t3188: F, t10474: F, t10482: F, t6739: F, t3127: F, t3131: F, t3215: F, t390: F, t300: F, t3368: F, t268: F, t405: F, t6546: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11034, t11037, t11046, t11051, t11054, t11059, t11060) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1371::<F>(t10947, t3185, t3199, t1014, t10471, t10470, t1057, t10960, t3120, t3188, t10474, t10482, t6739);
        let (t11065, t11066, t11094, t11126, t11135) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1372::<F>(t10471, t3127, t10470, t3131, t6739, t3215, t390, t300, t3368, t268, t405, t6546);
    (t11034, t11037, t11046, t11051, t11054, t11059, t11060, t11065, t11066, t11094, t11126, t11135)
}
