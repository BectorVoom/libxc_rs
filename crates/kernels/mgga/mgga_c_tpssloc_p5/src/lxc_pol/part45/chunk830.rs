//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 830/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk830<F: Float>(t23173: F, t7084: F, t814: F, t829: F, t2679: F, t7101: F, t235: F, t24234: F, t2051: F, t226: F, t23156: F, t23160: F, t23166: F, t23169: F, t23178: F, t23182: F, t23187: F, t2613: F, t7104: F, t808: F, t812: F) -> F {
    let t24265 = F::cast_from(0.16449340668482264365e-1_f64) * t23173;
    let t24269 = t814 * t7084;
    let t24270 = t24269 * t829;
    let t24273 = t7101 * t2679;
    let t24278 = t235 * t24234;
    let t24280 = -F::cast_from(0.6579736267392905746e-1_f64) * t23156 - F::cast_from(0.3289868133696452873e-1_f64) * t23160 + F::cast_from(0.3289868133696452873e-1_f64) * t23166 + F::cast_from(0.15352717957250113407e0_f64) * t23169 - t24265 - F::cast_from(0.3289868133696452873e-1_f64) * t23178 - F::cast_from(0.16449340668482264365e-1_f64) * t23182 + F::cast_from(0.16449340668482264365e-1_f64) * t23187 - F::new(2.0) * t812 * t24270 - t812 * t24273 + F::new(2.0) * t808 * t7104 + t2613 * t2051 + t226 * t24278;
    t24280
}
