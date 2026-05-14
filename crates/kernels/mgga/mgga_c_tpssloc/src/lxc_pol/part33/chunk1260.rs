//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1260/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1260<F: Float>(t28030: F, t7468: F, t28002: F, t7461: F, t28864: F, t7458: F, t28045: F, t4028: F, t6287: F, t652: F, t7467: F, t22574: F, t28830: F, t33136: F, t106956: F, t1874: F) -> (F, F, F, F, F, F, F, F) {
    let t107519 = 6.0 * t28030 * t7468;
    let t107521 = 12.0 * t28002 * t7461;
    let t107523 = 6.0 * t7458 * t28864;
    let t107525 = 12.0 * t4028 * t28045;
    let t107527 = 12.0 * t7458 * t28045;
    let t107530 = 6.0 * t652 * t6287 * t7467;
    let t107533 = 18.0 * t22574 * t33136 * t28830;
    let t107539 = 6.0 * t106956 * t1874;
    (t107519, t107521, t107523, t107525, t107527, t107530, t107533, t107539)
}
