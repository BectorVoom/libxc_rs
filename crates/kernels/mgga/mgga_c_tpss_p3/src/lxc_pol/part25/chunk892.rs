//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 892/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk892<F: Float>(t126: F, t2761: F, t2464: F, t277: F, t934: F, param_beta: F) -> (F, F, F) {
    let t8528 = t126 * t2761;
    let t8539 = F::cast_from(1.0_f64) / t277 / t2464;
    let t8546 = t934 * t934;
    let t8547 = F::cast_from(1.0_f64) / t8546;
    let t8548 = param_beta * t8547;
    (t8528, t8539, t8548)
}
