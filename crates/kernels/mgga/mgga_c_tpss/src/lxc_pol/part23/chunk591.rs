//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 591/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk591<F: Float>(t284: F, t2482: F, t2531: F, t2453: F, t2455: F, t2462: F, t2467: F, t2471: F, t872: F, t876: F, t301: F, t875: F, t296: F, t884: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2532 = t284 * t284;
    let t2533 = 1.0 / t2532;
    let t2534 = t2482 * t2533;
    let t2536 = 0.16081979498692535067e2 * t2531 * t2534;
    let t2537 = 0.22831111111111111111e-1 * t2453;
    let t2542 = t2537 + 0.11415555555555555555e-1 * t2455 - 0.11415555555555555555e-1 * t2462 + 0.34246666666666666666e-1 * t2467 - 0.17123333333333333333e-1 * t2471;
    let t2545 = t872 * t876;
    let t2548 = t875 * t301;
    let t2549 = 1.0 / t2548;
    let t2550 = t296 * t2549;
    let t2551 = t884 * t884;
    (t2532, t2533, t2534, t2536, t2537, t2542, t2545, t2549, t2550, t2551)
}
