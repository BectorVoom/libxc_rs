//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 542/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk542<F: Float>(t232: F, t776: F, t67: F, t753: F, t758: F, t152: F, t32: F, t181: F, t204: F, t686: F) -> (F, F, F, F, F) {
    let t2647 = t232 * t776;
    let t2652 = t753 * t67;
    let t2653 = t2652 * t758;
    let t2658 = t32 * t152;
    let t2663 = t686 * t204 * t181;
    (t2647, t2652, t2653, t2658, t2663)
}
