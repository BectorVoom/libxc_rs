//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 878/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk878<F: Float>(t2960: F, t2974: F, t3014: F, t984: F, t340: F, t343: F, t974: F, t135: F, t3016: F, t973: F, t10263: F, t10267: F, t10274: F, t10280: F, t10283: F, t10287: F, t10290: F, t10328: F, t10331: F, t10333: F, t10339: F, t2996: F, t3000: F, t3011: F, t3017: F, t346: F, t987: F) -> F {
    let t10342 = t2960 * t2974;
    let t10346 = t3014 * t984;
    let t10348 = t340 * t10346 * t343;
    let t10349 = t974 * t10348;
    let t10352 = t135 * t3016;
    let t10353 = t973 * t10352;
    let t10357 = -F::cast_from(0.14814814814814814814e-2_f64) * t10267 - F::cast_from(0.22222222222222222221e-2_f64) * t2960 * t3000 + F::cast_from(0.44444444444444444442e-2_f64) * t2960 * t2996 - F::cast_from(0.55555555555555555554e-3_f64) * t10274 - F::cast_from(0.22222222222222222221e-2_f64) * t973 * t10280 - F::cast_from(0.38024691358024691358e-1_f64) * t10283 * t346 + F::cast_from(0.55555555555555555554e-3_f64) * t10287 - F::cast_from(0.83333333333333333331e-3_f64) * t10290 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t10328 + F::cast_from(0.81481481481481481478e-2_f64) * t10331 + F::cast_from(0.14814814814814814814e-2_f64) * t10333 + t10339 - F::cast_from(0.24444444444444444444e-1_f64) * t10263 * t987 + F::cast_from(0.44444444444444444443e-2_f64) * t10342 + F::cast_from(0.66666666666666666666e-2_f64) * t2960 * t3011 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t10349 - F::cast_from(0.83333333333333333331e-3_f64) * t10353 + F::cast_from(0.66666666666666666666e-2_f64) * t2960 * t3017;
    t10357
}
