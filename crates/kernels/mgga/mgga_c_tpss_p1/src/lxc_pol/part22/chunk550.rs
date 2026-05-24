//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 550/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk550<F: Float>(t219: F, t2352: F, t73: F, t799: F, t2116: F, t2133: F, t778: F, t222: F, t224: F, t776: F, t779: F) -> (F, F, F, F) {
    let t2353 = t2352 * t219;
    let t2357 = t73 * t799;
    let t2358 = t2357 * t2116;
    let t2361 = t778 * t2133;
    let t2364 = -F::new(12.0) * t222 * t2358 + F::new(3.0) * t222 * t2361 - t224 * t2353 + F::new(6.0) * t776 * t779;
    (t2353, t2358, t2361, t2364)
}
