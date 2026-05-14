//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 753/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk753<F: Float>(t520: F, t5407: F, t1224: F, t774: F, t5380: F, t3348: F, t5371: F, t1248: F, t5366: F, t1213: F, t1222: F, t1244: F, t3239: F, t3244: F, t3258: F, t3271: F, t3340: F, t4402: F, t4422: F, t4476: F, t5373: F, t5377: F, t5383: F, t5389: F) -> (F, F, F, F, F, F, F) {
    let t5408 = t5407 * t520;
    let t5410 = t1224 * t774 * t5408;
    let t5413 = t5380 * t520;
    let t5415 = t1224 * t774 * t5413;
    let t5420 = t3348 * t774 * t5371;
    let t5424 = t1248 * t774 * t5366;
    let t5427 = t3239 + 7.0 / 72.0 * t4402 + t3244 * t5373 / 16.0 - t1213 * t5377 / 48.0 + t3258 * t5383 / 1536.0 + 7.0 / 2304.0 * t4422 + t3271 * t5389 / 384.0 - t1222 * t5410 / 3072.0 - t1222 * t5415 / 3072.0 + t3340 + 7.0 / 576.0 * t4476 + 5.0 / 768.0 * t1244 * t5420 - t1244 * t5424 / 768.0;
    (t5408, t5410, t5413, t5415, t5420, t5424, t5427)
}
