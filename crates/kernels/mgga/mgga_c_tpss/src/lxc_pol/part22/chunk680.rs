//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 680/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk680<F: Float>(t116: F, t2061: F, t117: F, t2105: F, t1279: F, t1281: F, t3403: F, t547: F, t548: F, t1953: F, t1957: F, t1960: F, t1964: F, t1967: F, t1973: F, t1286: F, t577: F) -> (F, F, F, F, F) {
    let t3407 = t116 * t2061;
    let t3410 = t117 * t2105;
    let t3413 = 6.0 * t1279 * t1281 + t3403 * t548 + 6.0 * t3407 * t547 + 3.0 * t3410 * t547;
    let t3416 = -t1953 + t1957 - t1960 + t1964 - t1967 + t1973;
    let t3418 = t1286 * t577;
    (t3407, t3410, t3413, t3416, t3418)
}
