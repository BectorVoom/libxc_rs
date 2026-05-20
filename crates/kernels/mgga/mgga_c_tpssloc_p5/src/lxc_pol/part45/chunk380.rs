//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 380/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk380<F: Float>(t123: F, t126: F, t131: F, t119: F, t132: F, t63: F, t204: F, t686: F, t685: F, t120: F, t118: F, t693: F) -> (F, F, F, F, F, F) {
    let t2385 = F::new(1.0) / t126 / t123 * t131;
    let t2386 = t132 * t119;
    let t2387 = t2386 * t63;
    let t2388 = t2385 * t2387;
    let t2390 = t686 * t204;
    let t2391 = t685 * t2390;
    let t2393 = t120 * t204;
    let t2394 = t118 * t2393;
    let t2396 = F::new(1.0)/F::sqrt(t123);
    let t2397 = t2396 * t131;
    let t2398 = t2397 * t2387;
    let t2400 = t693 * t2390;
    (t2388, t2391, t2393, t2394, t2398, t2400)
}
