//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1090/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1090<F: Float>(t1043: F, t15408: F, t1024: F, t5117: F, t9504: F, t2998: F, t5177: F, t4206: F, t1089: F, t5161: F, t9347: F, t9172: F, t1080: F, t9176: F, t11938: F, t11988: F, t11989: F, t11990: F, t15239: F, t15241: F, t15243: F, t15251: F, t15259: F, t15264: F, t15268: F, t15273: F, t15277: F, t15283: F, t15288: F, t9221: F, t9331: F) -> (F, F, F, F, F, F) {
    let t15409 = t15408 * t1043;
    let t15411 = 1.0 * t1024 * t15409;
    let t15413 = 0.16081979498692535067e2 * t9504 * t5117;
    let t15414 = t2998 * t5177;
    let t15415 = t15414 * t4206;
    let t15417 = 0.17315859105681463759e2 * t1089 * t15415;
    let t15418 = t9347 * t5161;
    let t15419 = t15418 * t4206;
    let t15421 = 0.10389515463408878255e3 * t1089 * t15419;
    let t15422 = t9172 * t5161;
    let t15423 = t9176 * t1080;
    let t15424 = t15422 * t15423;
    let t15426 = 0.10254018858216406658e4 * t1089 * t15424;
    let t15440 = -t9331 + 0.41203703703703703703e-2 * t9221 + 0.82407407407407407408e-2 * t11938 + t11988 - t11989 - t11990 + 0.20601851851851851852e-2 * t15239 + 0.10300925925925925926e-1 * t15259 - 0.37083333333333333333e-1 * t15264 - 0.12361111111111111111e-1 * t15268 - 0.61805555555555555557e-2 * t15241 + 0.55625000000000000001e-1 * t15273 + 0.37083333333333333334e-1 * t15277 - 0.30902777777777777778e-2 * t15243 - 0.61805555555555555555e-2 * t15283 + 0.18541666666666666667e-1 * t15288 + 0.92708333333333333333e-2 * t15251;
    (t15411, t15413, t15417, t15421, t15426, t15440)
}
