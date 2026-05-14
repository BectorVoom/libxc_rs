//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 380/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk380<F: Float>(t2374: F, t2375: F, t123: F, t126: F, t131: F, t119: F, t132: F, t63: F, t204: F, t686: F, t685: F, t120: F, t118: F, t693: F, t133: F, t702: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2377 = 0.10843581300301739842e-1 * t2374 * t2375;
    let t2385 = 1.0 / t126 / t123 * t131;
    let t2386 = t132 * t119;
    let t2387 = t2386 * t63;
    let t2388 = t2385 * t2387;
    let t2390 = t686 * t204;
    let t2391 = t685 * t2390;
    let t2393 = t120 * t204;
    let t2394 = t118 * t2393;
    let t2396 = 1.0/f64::sqrt(t123);
    let t2397 = t2396 * t131;
    let t2398 = t2397 * t2387;
    let t2400 = t693 * t2390;
    let t2402 = t119 * t63;
    let t2403 = t133 * t2402;
    let t2405 = -0.42198333333333333333e0 * t2388 + 0.84396666666666666666e0 * t2391 + 0.39862222222222222223e0 * t2394 + 0.68258333333333333333e-1 * t2398 + 0.13651666666666666667e0 * t2400 + 0.13692777777777777778e0 * t2403;
    let t2406 = t2405 * t702;
    (t2377, t2388, t2391, t2393, t2394, t2398, t2400, t2403, t2406)
}
