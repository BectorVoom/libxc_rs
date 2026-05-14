//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 852/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk852<F: Float>(t2458: F, t45: F, t672: F, t930: F, t925: F, t361: F, t650: F, t242: F, t949: F, t946: F, t2464: F, t265: F, t606: F, t2719: F, t72: F, t2737: F) -> (F, F, F, F, F, F, F, F) {
    let t8443 = t2458 * t45;
    let t8444 = 1.0 / t8443;
    let t8455 = t672 * t930;
    let t8456 = t925 * t8455;
    let t8469 = t650 * t361;
    let t8471 = t242 * t8469 * t949;
    let t8472 = t946 * t8471;
    let t8491 = 1.0 / t265 / t2464;
    let t8493 = 1.0 / t2458 / t606;
    let t8507 = t2719 * t72;
    let t8508 = t2737 * t8507;
    (t8444, t8456, t8469, t8472, t8491, t8493, t8507, t8508)
}
