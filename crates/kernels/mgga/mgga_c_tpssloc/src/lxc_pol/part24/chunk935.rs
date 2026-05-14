//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 935/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk935<F: Float>(t11228: F, t11268: F, t1118: F, t1099: F, t1097: F, t3311: F, t409: F, t3314: F, t422: F, t11191: F, t1146: F, t3399: F, t3402: F, t448: F, t11129: F, t1164: F) -> (F, F, F, F, F) {
    let t11269 = t11228 + t11268;
    let t11270 = t11269 * t1118;
    let t11272 = 1.0 * t1099 * t11270;
    let t11274 = 1.0 / t3311 / t1097;
    let t11275 = t409 * t11274;
    let t11277 = 1.0 / t3314 / t422;
    let t11278 = t11191 * t11277;
    let t11280 = 0.51726012919273400301e3 * t11275 * t11278;
    let t11282 = 1.0 / t3399 / t1146;
    let t11285 = 1.0 / t3402 / t448;
    let t11286 = t11282 * t11129 * t11285;
    let t11288 = 0.10254018858216406658e4 * t1164 * t11286;
    (t11272, t11280, t11282, t11285, t11288)
}
