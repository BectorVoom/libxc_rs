//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 698/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk698<F: Float>(t1250: F, t3342: F, t508: F, t526: F, t235: F, t72: F, t3245: F, t774: F, t1248: F, t3234: F, t1213: F, t1222: F, t1244: F, t3239: F, t3241: F, t3244: F, t3247: F, t3251: F, t3258: F, t3263: F, t3268: F, t3271: F, t3277: F, t3329: F, t3334: F, t3340: F) -> (F, F, F, F, F, F) {
    let t3343 = t3342 * t1250;
    let t3346 = 1.0 / t526 / t508;
    let t3347 = t235 * t3346;
    let t3348 = t3347 * t72;
    let t3350 = t3348 * t774 * t3245;
    let t3354 = t1248 * t774 * t3234;
    let t3357 = t3239 + 7.0 / 72.0 * t3241 + t3244 * t3247 / 16.0 - t1213 * t3251 / 48.0 + t3258 * t3263 / 1536.0 + 7.0 / 2304.0 * t3268 + t3271 * t3277 / 384.0 - t1222 * t3329 / 3072.0 - t1222 * t3334 / 3072.0 + t3340 + 7.0 / 576.0 * t3343 + 5.0 / 768.0 * t1244 * t3350 - t1244 * t3354 / 768.0;
    (t3343, t3346, t3348, t3350, t3354, t3357)
}
