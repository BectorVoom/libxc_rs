//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 120/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk120<F: Float>(t309: F, t318: F, t287: F, t294: F, t296: F, t305: F, t199: F, t235: F, zeta_threshold: F) -> (F, F, F, F) {
    let t288 = 2.0 <= zeta_threshold;
    let t291 = 0.0 <= zeta_threshold;
    let t319 = t309 * t318;
    let t322 = t294 * (-0.310907e-1 * t296 * t305 + t287 - 0.19751673498613801407e-1 * t319);
    let t324 = 0.19751673498613801407e-1 * t294 * t319;
    let t325 = piecewise3(t288, t199, t235);
    let t326 = piecewise3(t291, t199, 0.0);
    let t328 = t325 / 2.0 + t326 / 2.0;
    let t329 = t328 * t328;
    (t322, t324, t328, t329)
}
