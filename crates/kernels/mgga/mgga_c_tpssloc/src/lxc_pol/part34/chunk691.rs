//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 691/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk691<F: Float>(t10276: F, t344: F, t241: F, t625: F, t281: F, t283: F, t2978: F, t340: F, t63: F, t221: F, t339: F, t2393: F, t374: F, t376: F, t370: F, t3036: F, t67: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10277 = 1.0 / t10276;
    let t10278 = t344 * t10277;
    let t10292 = t625 * t241;
    let t10294 = t281 * t10292 * t283;
    let t10295 = 20.0 / 27.0 * t10294;
    let t10304 = t241 * t2978;
    let t10335 = t63 * t340;
    let t10336 = t10335 * t344;
    let t10337 = t221 * t10336;
    let t10339 = 0.3086419753086419753e-3 * t339 * t10337;
    let t10375 = t374 * t2393 * t376;
    let t10377 = t370 * t10375 / 10368.0;
    let t10383 = t221 * t10335;
    let t10385 = 5.0 / 1296.0 * t339 * t10383;
    let t10401 = t3036 * t67;
    (t10277, t10278, t10292, t10294, t10295, t10304, t10339, t10377, t10385, t10401)
}
