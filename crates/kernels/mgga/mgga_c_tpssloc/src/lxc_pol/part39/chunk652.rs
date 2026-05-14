//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 652/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk652<F: Float>(t3243: F, t3297: F, t136: F, t1113: F, t3248: F, t3252: F, t3238: F, t3245: F, t3250: F, t3254: F, t3272: F, t3280: F, t3282: F, t3288: F, t3290: F, t3294: F, t3295: F) -> (F, F, F, F, F, F, F) {
    let t3298 = t3297 * t3243;
    let t3299 = t136 * t3298;
    let t3301 = t1113 * t3248;
    let t3302 = t136 * t3301;
    let t3304 = t1113 * t3252;
    let t3305 = t136 * t3304;
    let t3307 = -0.9494625e0 * t3272 + 0.1898925e1 * t3280 + t3282 - 0.19931111111111111111e0 * t3238 - 0.19931111111111111111e0 * t3245 + 0.59793333333333333334e0 * t3250 + 0.29896666666666666667e0 * t3254 + 0.15358125e0 * t3288 + 0.3071625e0 * t3290 + t3294 - 0.10954222222222222222e0 * t3295 - 0.27385555555555555556e-1 * t3299 + 0.16431333333333333333e0 * t3302 + 0.82156666666666666667e-1 * t3305;
    (t3298, t3299, t3301, t3302, t3304, t3305, t3307)
}
