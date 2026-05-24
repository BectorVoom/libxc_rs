//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 538/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk538<F: Float>(t2267: F, t705: F, t697: F, t164: F, t172: F, t2257: F, t123: F, t147: F, t2192: F) -> (F, F, F, F, F, F, F, F) {
    let t2268 = t2267 * t705;
    let t2271 = t697 * t697;
    let t2272 = F::new(1.0) / t2271;
    let t2273 = t164 * t2272;
    let t2274 = t172 * t172;
    let t2275 = F::new(1.0) / t2274;
    let t2276 = t2257 * t2275;
    let t2281 = F::cast_from(0.14764627977777777777e-2_f64) * t123 * t2192 * t147;
    (t2268, t2271, t2272, t2273, t2274, t2275, t2276, t2281)
}
