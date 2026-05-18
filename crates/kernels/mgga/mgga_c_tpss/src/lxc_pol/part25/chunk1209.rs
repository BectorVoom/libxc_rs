//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1209/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1209<F: Float>(t19466: F, t19479: F, t19491: F, t18438: F, t18452: F, t18466: F, t18934: F, t18943: F, t19471: F, t19473: F, t19477: F, t19481: F, t19483: F, t19485: F, t19489: F, t19493: F, t19495: F) -> F {
    let t20142 = F::new(7.0) / F::new(72.0) * t19466;
    let t20146 = F::new(7.0) / F::new(1152.0) * t19479;
    let t20151 = F::new(7.0) / F::new(288.0) * t19491;
    let t20154 = t18934 + t18438 + t20142 + t19471 / F::new(8.0) - t19473 / F::new(24.0) + t19477 / F::new(384.0) + t20146 + t19481 / F::new(192.0) - t19483 / F::new(768.0) - t19485 / F::new(768.0) + t18452 + t18943 + t18466 + t19489 / F::new(192.0) + t20151 + F::new(5.0) / F::new(192.0) * t19493 - t19495 / F::new(192.0);
    t20154
}
