//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 463/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk463<F: Float>(t40: F, t52: F, t2427: F, t708: F, t607: F, t751: F, t707: F, t195: F, t2244: F, t2250: F, t73: F, t197: F, t76: F, t157: F, t182: F, t676: F, t724: F, t164: F, t723: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t2429 = 8.0 * t2427 * t708;
    let t2430 = t751 * t607;
    let t2431 = t707 * t2430;
    let t2432 = 8.0 * t2431;
    let t2433 = 1.0 / t195;
    let t2439 = piecewise3(t146, 0.0, 4.0 / 9.0 * t2433 * t2244 + 4.0 / 3.0 * t73 * t2250);
    let t2440 = 1.0 / t197;
    let t2446 = piecewise3(t150, 0.0, 4.0 / 9.0 * t2440 * t2244 - 4.0 / 3.0 * t76 * t2250);
    let t2447 = t2439 + t2446;
    let t2448 = t2447 * t157;
    let t2450 = 0.19751673498613801407e-1 * t2448 * t182;
    let t2454 = t676 * t724;
    let t2458 = t723 * t164;
    (t2429, t2430, t2432, t2433, t2440, t2447, t2448, t2450, t2454, t2458)
}
