//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 386/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk386<F: Float>(t1279: F, t1281: F, t547: F, t548: F, t553: F, t557: F, t561: F, t565: F, t569: F, t573: F, t4: F, t579: F) -> (F, F, F) {
    let t1284 = t1279 * t548 + 3.0 * t1281 * t547;
    let t1286 = -t553 - t557 - t561 - t565 - t569 - t573;
    let t1288 = -t4 - t579;
    (t1284, t1286, t1288)
}
