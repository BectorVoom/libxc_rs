//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 640/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk640<F: Float>(t1398: F, t2436: F, t1364: F, t2440: F, t1692: F, t198: F, t207: F, t2208: F, t2217: F, t2245: F, t2292: F, t2302: F, t2310: F, t2333: F, t2347: F, t2439: F, t3594: F, t3610: F, t3644: F, t3646: F, t3647: F, t3724: F, t740: F, t821: F, t823: F) -> (F, F) {
    let t3728 = t1398 * t2436;
    let t3731 = t2440 * t1364;
    let t3734 = t198 * t207 * t3724 * t823 - t1692 * t3728 * t821 + 3.0 * t198 * t3610 * t740 + 3.0 * t2439 * t3731 - t2208 - t2217 + t2245 - t2292 + t2302 + t2310 + t2333 + t2347 - t3594 + t3644 + t3646 + t3647;
    (t3728, t3734)
}
