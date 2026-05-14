//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1241/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1241<F: Float>(t20497: F, t22761: F, t20512: F, t80830: F, t1998: F, t20416: F, t236: F, t6926: F, t20470: F, t26309: F, t26257: F, t6431: F, t6427: F, t20433: F, t6952: F, t12289: F, t20490: F, t6936: F) -> (F, F, F, F, F, F, F, F) {
    let t107093 = t22761 * t20497;
    let t107096 = t80830 * t20512;
    let t107100 = t6926 * t1998 * t236 * t20416;
    let t107102 = t26309 * t20470;
    let t107105 = t26257 * t6431;
    let t107107 = t26257 * t6427;
    let t107109 = t6952 * t20433;
    let t107112 = t6936 * t12289 * t20490;
    (t107093, t107096, t107100, t107102, t107105, t107107, t107109, t107112)
}
