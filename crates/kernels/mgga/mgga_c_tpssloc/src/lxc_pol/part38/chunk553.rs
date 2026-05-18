//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 553/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk553<F: Float>(t2471: F, t731: F, t723: F, t159: F, t167: F, t2461: F, t676: F, t682: F, t268: F, t703: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2472 = t2471 * t731;
    let t2475 = t723 * t723;
    let t2476 = F::new(1.0) / t2475;
    let t2477 = t159 * t2476;
    let t2478 = t167 * t167;
    let t2479 = F::new(1.0) / t2478;
    let t2480 = t2461 * t2479;
    let t2483 = t676 * t682;
    let t2486 = F::new(0.35616666666666666666e-1) * t268 * t2483 * t703;
    (t2472, t2475, t2476, t2477, t2478, t2479, t2480, t2483, t2486)
}
