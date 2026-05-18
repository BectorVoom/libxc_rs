//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 581/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk581<F: Float>(t2460: F, t2515: F, t141: F, t2465: F, t861: F, t2469: F, t2455: F, t2462: F, t2467: F, t2471: F, t2489: F, t2497: F, t2499: F, t2505: F, t2507: F, t2512: F, t2513: F) -> (F, F, F, F, F, F, F) {
    let t2516 = t2515 * t2460;
    let t2517 = t141 * t2516;
    let t2519 = t861 * t2465;
    let t2520 = t141 * t2519;
    let t2522 = t861 * t2469;
    let t2523 = t141 * t2522;
    let t2525 = -F::new(0.9494625e0) * t2489 + F::new(0.1898925e1) * t2497 + t2499 + F::new(0.19931111111111111111e0) * t2455 - F::new(0.19931111111111111111e0) * t2462 + F::new(0.59793333333333333334e0) * t2467 - F::new(0.29896666666666666667e0) * t2471 + F::new(0.15358125e0) * t2505 + F::new(0.3071625e0) * t2507 + t2512 + F::new(0.10954222222222222222e0) * t2513 - F::new(0.27385555555555555556e-1) * t2517 + F::new(0.16431333333333333333e0) * t2520 - F::new(0.82156666666666666667e-1) * t2523;
    (t2516, t2517, t2519, t2520, t2522, t2523, t2525)
}
