//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 890/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk890<F: Float>(t810: F, t73: F, t2157: F, t806: F, t2458: F, t45: F, t672: F, t930: F, t925: F, t361: F, t650: F, t242: F, t949: F) -> (F, F, F, F, F, F, F, F) {
    let t8346 = t810 * t810;
    let t8347 = F::new(1.0) / t8346;
    let t8348 = t73 * t8347;
    let t8361 = t2157 * t806;
    let t8443 = t2458 * t45;
    let t8444 = F::new(1.0) / t8443;
    let t8455 = t672 * t930;
    let t8456 = t925 * t8455;
    let t8469 = t650 * t361;
    let t8471 = t242 * t8469 * t949;
    (t8346, t8347, t8348, t8361, t8444, t8456, t8469, t8471)
}
