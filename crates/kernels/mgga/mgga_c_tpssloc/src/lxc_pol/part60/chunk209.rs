//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 209/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk209<F: Float>(t419: F, t409: F, t410: F, t1086: F, t407: F, t281: F, t415: F, t904: F, t241: F, t457: F, t422: F, t432: F, t427: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1097 = t419 * t419;
    let t1098 = 1.0 / t1097;
    let t1099 = t409 * t1098;
    let t1100 = 1.0 / t410;
    let t1105 = 0.29896666666666666667e0 * t1086;
    let t1107 = f64::sqrt(t407);
    let t1111 = t281 * t904 * t415;
    let t1112 = 0.82156666666666666667e-1 * t1111;
    let t1113 = t241 * t457;
    let t1118 = 1.0 / t422;
    let t1122 = 0.17123333333333333333e-1 * t1086;
    let t1127 = t432 * t432;
    let t1128 = 1.0 / t1127;
    let t1129 = t427 * t1128;
    let t1131 = 0.516475e0 * t1086;
    let t1134 = 0.104195e0 * t1111;
    (t1097, t1098, t1099, t1100, t1105, t1107, t1111, t1112, t1113, t1118, t1122, t1127, t1128, t1129, t1131, t1134)
}
