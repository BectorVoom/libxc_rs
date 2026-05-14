//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 899/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk899<F: Float>(t10993: F, t973: F, t248: F, t3101: F, t3132: F, t3130: F, t225: F, t3167: F, t10947: F, t3185: F, t3199: F, t1014: F, t10471: F, t10470: F, t1057: F, t10960: F) -> (F, F, F, F, F, F, F) {
    let t10994 = t973 * t10993;
    let t11002 = t248 * t3101 * t3132;
    let t11003 = t3130 * t11002;
    let t11010 = t3167 * t225;
    let t11034 = t10947 * t3185;
    let t11037 = t10947 * t3199;
    let t11045 = t10471 * t1014;
    let t11046 = t10470 * t11045;
    let t11051 = t10960 * t1057;
    (t10994, t11003, t11010, t11034, t11037, t11046, t11051)
}
