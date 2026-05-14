//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1101/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1101<F: Float>(t15440: F, t434: F, t294: F, t1023: F, t5076: F, t1044: F, t11971: F, t1519: F, t11873: F, t11938: F, t12231: F, t12232: F, t15239: F, t15241: F, t15243: F, t15251: F, t15259: F, t15264: F, t15268: F, t15273: F, t15277: F, t15283: F, t15288: F, t9221: F, t9399: F) -> (F, F, F, F, F) {
    let t15441 = t15440 * t434;
    let t15443 = 0.19751673498613801407e-1 * t294 * t15441;
    let t15444 = t5076 * t1023;
    let t15446 = 1.0 * t15444 * t1044;
    let t15448 = 2.0 * t11971 * t1519;
    let t15463 = -t9399 + 0.79148148148148148147e-2 * t9221 + 0.15829629629629629629e-1 * t11938 + 0.79148148148148148147e-2 * t11873 - t12231 - t12232 + 0.39574074074074074073e-2 * t15239 + 0.19787037037037037037e-1 * t15259 - 0.71233333333333333332e-1 * t15264 - 0.23744444444444444444e-1 * t15268 - 0.11872222222222222222e-1 * t15241 + 0.10685e0 * t15273 + 0.71233333333333333332e-1 * t15277 - 0.5936111111111111111e-2 * t15243 - 0.11872222222222222222e-1 * t15283 + 0.35616666666666666666e-1 * t15288 + 0.17808333333333333333e-1 * t15251;
    (t15441, t15443, t15446, t15448, t15463)
}
