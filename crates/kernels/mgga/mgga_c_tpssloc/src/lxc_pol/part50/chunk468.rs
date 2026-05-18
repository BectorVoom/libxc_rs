//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 468/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk468<F: Float>(t730: F, t731: F, t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F, t723: F, t159: F, t167: F, t676: F, t682: F) -> (F, F, F, F, F) {
    let t2461 = t730 * t730;
    let t2462 = t2461 * t731;
    let t2471 = -F::new(0.78438333333333333333e0) * t2388 + F::new(0.15687666666666666667e1) * t2391 + F::new(0.68863333333333333333e0) * t2394 + F::new(0.14025833333333333333e0) * t2398 + F::new(0.28051666666666666667e0) * t2400 + F::new(0.17365833333333333333e0) * t2403;
    let t2472 = t2471 * t731;
    let t2475 = t723 * t723;
    let t2476 = F::new(1.0) / t2475;
    let t2477 = t159 * t2476;
    let t2478 = t167 * t167;
    let t2479 = F::new(1.0) / t2478;
    let t2480 = t2461 * t2479;
    let t2483 = t676 * t682;
    (t2462, t2472, t2477, t2480, t2483)
}
