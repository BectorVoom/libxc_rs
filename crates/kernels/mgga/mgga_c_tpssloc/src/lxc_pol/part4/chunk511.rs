//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 511/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk511<F: Float>(t706: F, t717: F, t607: F, t751: F, t707: F, t195: F, t197: F, t676: F, t724: F, t164: F, t723: F, t159: F, t730: F, t731: F, t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2427 = t706 * t717;
    let t2430 = t751 * t607;
    let t2431 = t707 * t2430;
    let t2433 = 1.0 / t195;
    let t2440 = 1.0 / t197;
    let t2454 = t676 * t724;
    let t2458 = t723 * t164;
    let t2459 = 1.0 / t2458;
    let t2460 = t159 * t2459;
    let t2461 = t730 * t730;
    let t2462 = t2461 * t731;
    let t2471 = -0.78438333333333333333e0 * t2388 + 0.15687666666666666667e1 * t2391 + 0.68863333333333333333e0 * t2394 + 0.14025833333333333333e0 * t2398 + 0.28051666666666666667e0 * t2400 + 0.17365833333333333333e0 * t2403;
    (t2427, t2430, t2431, t2433, t2440, t2454, t2459, t2460, t2461, t2462, t2471)
}
