//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 872/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk872<F: Float>(t2303: F, t655: F, t130: F, t2289: F, t675: F, t146: F, t2306: F) -> (F, F) {
    let t7938 = F::new(1.0) / t2303 / t655;
    let t7939 = t130 * t7938;
    let t7940 = t2289 * t675;
    let t7942 = F::new(1.0) / t2306 / t146;
    let t7943 = t7940 * t7942;
    let t7945 = F::cast_from(0.51726012919273400301e3_f64) * t7939 * t7943;
    (t7940, t7945)
}
