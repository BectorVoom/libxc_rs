//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 578/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk578<F: Float>(t2645: F, t2646: F, t2647: F, t67: F, t753: F, t758: F, t185: F, t2250: F, t707: F, t152: F, t32: F, t2244: F) -> (F, F, F, F, F, F, F, F) {
    let t2649 = t2645 * t2646 * t2647;
    let t2652 = t753 * t67;
    let t2653 = t2652 * t758;
    let t2654 = F::cast_from(0.36622894612013090108e-3_f64) * t2653;
    let t2655 = t185 * t2250;
    let t2657 = F::new(4.0) * t707 * t2655;
    let t2658 = t32 * t152;
    let t2659 = t185 * t2244;
    (t2649, t2652, t2653, t2654, t2655, t2657, t2658, t2659)
}
