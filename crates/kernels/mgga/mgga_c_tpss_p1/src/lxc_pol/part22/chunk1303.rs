//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1303/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1303<F: Float>(t60684: F, t60722: F, t18948: F, t219: F, t1219: F, t5918: F, t198: F, t206: F, t5848: F, t5831: F, t768: F, t61024: F) -> (F, F, F, F, F, F, F) {
    let t62375 = F::cast_from(595.0_f64) / F::cast_from(2592.0_f64) * t60684;
    let t62390 = F::cast_from(455.0_f64) / F::cast_from(648.0_f64) * t60722;
    let t62453 = t18948 * t219;
    let t62508 = t1219 * t5918;
    let t62610 = t198 * t206 * t5848;
    let t62671 = t768 * t5831;
    let t62690 = F::cast_from(595.0_f64) / F::cast_from(2592.0_f64) * t61024;
    (t62375, t62390, t62453, t62508, t62610, t62671, t62690)
}
