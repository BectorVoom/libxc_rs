//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1052/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1052<F: Float>(t15248: F, t15251: F, t15292: F, t15294: F, t15296: F, t15299: F, t15301: F, t15304: F, t15307: F, t15309: F, t15312: F, t11938: F, t12060: F, t15264: F, t15268: F, t15273: F, t15277: F, t15283: F, t15288: F, t15334: F, t15339: F, t15342: F) -> (F, F) {
    let t15669 = -0.69463333333333333333e-1 * t15248 + 0.516475e0 * t15251 + 0.3529725e1 * t15292 + 0.6311625e0 * t15294 + 0.23154444444444444445e-1 * t15296 - 0.157790625e0 * t15299 + 0.6311625e0 * t15301 + 0.31558125e0 * t15304 + 0.264729375e1 * t15307 - 0.3529725e1 * t15309 - 0.17648625e1 * t15312;
    let t15690 = -0.20839e0 * t15334 + 0.45908888888888888888e0 * t11938 - t12060 - 0.34431666666666666667e0 * t15283 + 0.46308888888888888889e-1 * t15339 - 0.34731666666666666667e-1 * t15342 - 0.68863333333333333334e0 * t15268 - 0.20659e1 * t15264 + 0.20659e1 * t15277 + 0.309885e1 * t15273 + 0.103295e1 * t15288;
    (t15669, t15690)
}
