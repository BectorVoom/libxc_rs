//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1092/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1092<F: Float>(t1025: F, t11954: F, t11942: F, t1032: F, t11878: F, t9185: F, t141: F, t11932: F, t11938: F, t11952: F, t9221: F, t9223: F, t9226: F, t9228: F) -> (F, F, F, F) {
    let t11955 = t1025 * t11954;
    let t11958 = F::new(0.19931111111111111111e0) * t11942;
    let t11960 = t1032 * t11954;
    let t11962 = t9185 * t11878;
    let t11963 = t141 * t11962;
    let t11965 = F::new(0.26574814814814814816e0) * t9221 + F::new(0.66437037037037037038e-1) * t9223 - F::new(0.19931111111111111111e0) * t9226 - F::new(0.99655555555555555557e-1) * t9228 + F::new(0.36514074074074074074e-1) * t11932 + F::new(0.1898925e1) * t11955 + F::new(0.13287407407407407408e0) * t11938 - t11958 + F::new(0.29896666666666666667e0) * t11952 + F::new(0.3071625e0) * t11960 + F::new(0.36514074074074074075e-1) * t11963;
    (t11955, t11960, t11963, t11965)
}
