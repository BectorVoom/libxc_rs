//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1117/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1117<F: Float>(t18438: F, t18452: F, t18466: F, t18934: F, t18943: F, t19471: F, t19473: F, t19477: F, t19481: F, t19483: F, t19485: F, t19489: F, t19493: F, t19495: F, t20142: F, t20146: F, t20151: F) -> (F,) {
    let t20154 = t18934 + t18438 + t20142 + t19471 / 8.0 - t19473 / 24.0 + t19477 / 384.0 + t20146 + t19481 / 192.0 - t19483 / 768.0 - t19485 / 768.0 + t18452 + t18943 + t18466 + t19489 / 192.0 + t20151 + 5.0 / 192.0 * t19493 - t19495 / 192.0;
    (t20154,)
}
