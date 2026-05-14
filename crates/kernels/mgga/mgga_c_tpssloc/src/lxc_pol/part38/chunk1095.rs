//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1095/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1095<F: Float>(t15288: F, t4919: F, t11531: F, t11534: F, t11537: F, t11541: F, t11591: F, t1174: F, t15265: F, t15269: F, t15274: F, t15278: F, t15284: F, t15287: F, t3447: F, t11583: F, t3961: F) -> (F, F) {
    let t15289 = t4919 * t15288;
    let t15292 = 0.12345679012345679012e-3 * t11531 - 0.9259259259259259259e-4 * t11534 - 0.18518518518518518518e-3 * t11537 + 0.12345679012345679012e-3 * t11541 + 0.18518518518518518518e-3 * t11591 + 0.49382716049382716049e-3 * t15265 - 0.16666666666666666666e-2 * t1174 * t15269 - 0.83333333333333333332e-3 * t1174 * t15274 - 0.27777777777777777777e-3 * t1174 * t15278 - t15284 - t15287 + 0.55555555555555555554e-3 * t3447 * t15289;
    let t15293 = t11583 * t3961;
    (t15292, t15293)
}
