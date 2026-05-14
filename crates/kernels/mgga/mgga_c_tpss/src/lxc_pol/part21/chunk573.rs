//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 573/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk573<F: Float>(t2383: F, t803: F, t206: F, t237: F, t235: F, t72: F, t2116: F, t774: F, t2133: F, t801: F, t2142: F, t2144: F, t2147: F, t2149: F, t2153: F, t2160: F, t2165: F, t2170: F, t2173: F, t2179: F, t2367: F, t2372: F, t2381: F, t761: F, t771: F, t797: F) -> (F, F, F, F, F, F) {
    let t2384 = t2383 * t803;
    let t2387 = 1.0 / t237 / t206;
    let t2388 = t235 * t2387;
    let t2389 = t2388 * t72;
    let t2391 = t2389 * t774 * t2116;
    let t2395 = t801 * t774 * t2133;
    let t2398 = t2142 + 7.0 / 72.0 * t2144 + t2147 * t2149 / 16.0 - t761 * t2153 / 48.0 + t2160 * t2165 / 1536.0 + 7.0 / 2304.0 * t2170 + t2173 * t2179 / 384.0 - t771 * t2367 / 3072.0 - t771 * t2372 / 3072.0 + t2381 + 7.0 / 576.0 * t2384 + 5.0 / 768.0 * t797 * t2391 - t797 * t2395 / 768.0;
    (t2384, t2387, t2389, t2391, t2395, t2398)
}
