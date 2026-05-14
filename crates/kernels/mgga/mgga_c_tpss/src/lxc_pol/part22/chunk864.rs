//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 864/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk864<F: Float>(t242: F, t2460: F, t8528: F, t967: F, t2464: F, t277: F, t934: F) -> (F, F, F) {
    let t8530 = t242 * t8528 * t2460;
    let t8531 = t967 * t8530;
    let t8539 = 1.0 / t277 / t2464;
    let t8546 = t934 * t934;
    let t8547 = 1.0 / t8546;
    let t8548 = param_beta * t8547;
    (t8531, t8539, t8548)
}
