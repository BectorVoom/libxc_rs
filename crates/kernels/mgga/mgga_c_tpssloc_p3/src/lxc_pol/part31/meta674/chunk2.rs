//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2038/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2038<F: Float>(t101091: F, t101134: F, t102105: F, t102320: F, t102366: F, t102988: F, t103029: F, t103070: F, t100930: F, t1458: F, t16521: F, t16524: F, t19534: F, t20162: F, t20173: F, t20181: F, t2039: F, t24465: F, t27170: F, t27254: F, t27281: F, t28893: F, t28951: F, t29422: F, t29425: F, t3941: F, t4072: F, t5456: F, t5493: F, t55353: F, t577: F, t66958: F, t671: F, t7056: F, t7801: F, t7956: F, t84033: F) -> (F, F) {
    let t103073 = t101091 + t101134 + t102105 + t102320 + t102366 + t102988 + t103029 + t103070;
    let t103088 = F::cast_from(54.0_f64) * t20173 * t29422 + F::cast_from(54.0_f64) * t3941 * t27170 * t1458 + F::cast_from(54.0_f64) * t3941 * t7801 * t4072 + F::cast_from(54.0_f64) * t16524 * t27281 + F::cast_from(27.0_f64) * t20173 * t29425 + F::cast_from(27.0_f64) * t3941 * t28951 * t671 + F::cast_from(27.0_f64) * t100930 * t2039 + F::cast_from(27.0_f64) * t84033 * t5456 + F::cast_from(27.0_f64) * t28893 * t7056 + F::cast_from(27.0_f64) * t16521 * t7801 + F::cast_from(0.135e2_f64) * t20162 * t7056 + F::cast_from(27.0_f64) * t27254 * t4072 + F::cast_from(0.45e1_f64) * t103073 * t577 + F::cast_from(54.0_f64) * t55353 * t7956 + F::cast_from(27.0_f64) * t3941 * t7056 * t5493 + F::cast_from(27.0_f64) * t3941 * t2039 * t19534 + F::cast_from(27.0_f64) * t24465 * t20181 + F::cast_from(0.135e2_f64) * t66958 * t2039;
    (t103073, t103088)
}
