//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 117/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk117<F: Float>(t309: F, t318: F, t287: F, t294: F, t296: F, t305: F, t199: F, t235: F, zeta_threshold: F) -> (F, F, F, F) {
    let t288 = F::cast_from(2.0_f64) <= zeta_threshold;
    let t291 = F::cast_from(0.0_f64) <= zeta_threshold;
    let t319 = t309 * t318;
    let t322 = t294 * (-F::cast_from(0.310907e-1_f64) * t296 * t305 + t287 - F::cast_from(0.19751673498613801407e-1_f64) * t319);
    let t324 = F::cast_from(0.19751673498613801407e-1_f64) * t294 * t319;
    let t325 = piecewise3::<F>(t288, t199, t235);
    let t326 = piecewise3::<F>(t291, t199, F::cast_from(0.0_f64));
    let t328 = t325 / F::cast_from(2.0_f64) + t326 / F::cast_from(2.0_f64);
    let t329 = t328 * t328;
    (t322, t324, t328, t329)
}
