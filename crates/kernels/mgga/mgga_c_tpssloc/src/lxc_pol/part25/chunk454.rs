//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 454/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk454<F: Float>(t67: F, t753: F, t758: F, t185: F, t2250: F, t707: F, t152: F, t32: F, t2244: F, t181: F, t204: F, t686: F, t756: F, t2373: F, t2377: F, t2408: F, t2417: F, t2423: F, t2426: F, t2429: F, t2432: F, t2450: F, t2486: F, t2518: F, t2520: F, t2530: F, t2533: F, t2537: F, t2539: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2652 = t753 * t67;
    let t2653 = t2652 * t758;
    let t2654 = 0.36622894612013090108e-3 * t2653;
    let t2655 = t185 * t2250;
    let t2657 = 4.0 * t707 * t2655;
    let t2658 = t32 * t152;
    let t2659 = t185 * t2244;
    let t2661 = 12.0 * t2658 * t2659;
    let t2663 = t686 * t204 * t181;
    let t2665 = 0.24415263074675393405e-3 * t756 * t2663;
    let t2666 = -t2654 + t2373 + t2377 - t2486 + t2450 + t2518 + t2408 + t2417 + t2520 + t2539 - t2530 - t2533 - t2537 + t2657 + t2661 - t2426 + t2665 + t2429 + t2432 - t2423;
    (t2652, t2654, t2655, t2657, t2658, t2659, t2661, t2663, t2665, t2666)
}
