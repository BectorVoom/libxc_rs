//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 783/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk783<F: Float>(t3: F, t7415: F, t112: F, t2169: F, t577: F, t671: F, t7014: F, t7017: F, t7019: F, t2018: F, t3701: F, t590: F, t60: F) -> (F, F, F, F, F) {
    let t7416 = t3 * t7415;
    let t7423 = t2169 * t112;
    let t7426 = F::cast_from(0.45e1_f64) * t7415 * t577 + F::cast_from(0.135e2_f64) * t7423 * t671 + t7014 + t7017 + t7019;
    let t8643 = t3701 * t2018;
    let t8705 = F::cast_from(1.0_f64) / t60 / t590;
    (t7416, t7423, t7426, t8643, t8705)
}
