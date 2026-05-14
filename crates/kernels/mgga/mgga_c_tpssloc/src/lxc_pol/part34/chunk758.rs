//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 758/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk758<F: Float>(t5: F, t12571: F, t1437: F, t19299: F, t20193: F, t20201: F, t20204: F, t20288: F, t2240: F, t3953: F, t5389: F, t5445: F, t605: F, t86: F, t9239: F, t112: F, t1441: F, t5456: F) -> (F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t20292 = piecewise3(t8, 0.0, 60.0 * t12571 * t5389 - 12.0 * t1437 * t19299 + t20193 * t86 - 120.0 * t20201 * t9239 + 60.0 * t20204 * t2240 - 4.0 * t20288 * t605 - 12.0 * t3953 * t5445);
    let t20293 = t20292 * t112;
    let t20296 = t1441 * t5456;
    (t20292, t20293, t20296)
}
