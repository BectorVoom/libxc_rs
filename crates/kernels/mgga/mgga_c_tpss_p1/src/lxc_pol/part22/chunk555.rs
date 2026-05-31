//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 555/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk555<F: Float>(t2383: F, t803: F, t206: F, t237: F, t235: F, t72: F, t2116: F, t774: F, t2133: F, t801: F, t2142: F, t2144: F, t2147: F, t2149: F, t2153: F, t2160: F, t2165: F, t2170: F, t2173: F, t2179: F, t2367: F, t2372: F, t2381: F, t761: F, t771: F, t797: F) -> (F, F, F, F, F, F) {
    let t2384 = t2383 * t803;
    let t2387 = F::cast_from(1.0_f64) / t237 / t206;
    let t2388 = t235 * t2387;
    let t2389 = t2388 * t72;
    let t2391 = t2389 * t774 * t2116;
    let t2395 = t801 * t774 * t2133;
    let t2398 = t2142 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t2144 + t2147 * t2149 / F::cast_from(16.0_f64) - t761 * t2153 / F::cast_from(48.0_f64) + t2160 * t2165 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t2170 + t2173 * t2179 / F::cast_from(384.0_f64) - t771 * t2367 / F::cast_from(3072.0_f64) - t771 * t2372 / F::cast_from(3072.0_f64) + t2381 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t2384 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t797 * t2391 - t797 * t2395 / F::cast_from(768.0_f64);
    (t2384, t2387, t2389, t2391, t2395, t2398)
}
