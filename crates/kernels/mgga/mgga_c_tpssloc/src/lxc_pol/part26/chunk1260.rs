//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1260/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1260<F: Float>(t510: F, t652: F, t81455: F, t1983: F, t22584: F, t22591: F, t25014: F, t9616: F, t25373: F, t46320: F, t193: F, t201: F, t6665: F) -> (F, F, F, F, F) {
    let t81458 = F::new(2.0) * t652 * t510 * t81455;
    let t81469 = F::new(9.0) * t1983 * t22591 * t22584;
    let t81470 = t25014 * t9616;
    let t81476 = t25373 * t46320;
    let t81483 = t193 * t201 * t6665;
    (t81458, t81469, t81470, t81476, t81483)
}
