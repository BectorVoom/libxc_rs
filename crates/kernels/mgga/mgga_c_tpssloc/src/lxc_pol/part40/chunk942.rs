//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 942/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk942<F: Float>(t3314: F, t422: F, t1146: F, t3399: F, t3402: F, t448: F, t445: F, t1143: F, t3375: F, t1124: F, t3331: F, t440: F) -> (F, F, F, F, F, F, F) {
    let t11277 = F::new(1.0) / t3314 / t422;
    let t11282 = F::new(1.0) / t3399 / t1146;
    let t11285 = F::new(1.0) / t3402 / t448;
    let t11292 = F::new(1.0) / t3399 / t445;
    let t11297 = t1143 * t3375;
    let t11303 = t1124 * t3331;
    let t11310 = t440 * t11282;
    (t11277, t11282, t11285, t11292, t11297, t11303, t11310)
}
