//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 911/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk911<F: Float>(t2798: F, t8508: F, t2782: F, t2762: F, t774: F, t126: F, t2761: F, t2464: F, t277: F, t934: F) -> (F, F, F, F, F, F, F, F) {
    let t8509 = t2798 * t8508;
    let t8514 = t2782 * t8508;
    let t8523 = t774 * t2762;
    let t8528 = t126 * t2761;
    let t8539 = 1.0 / t277 / t2464;
    let t8546 = t934 * t934;
    let t8547 = 1.0 / t8546;
    let t8548 = param_beta * t8547;
    (t8509, t8514, t8523, t8528, t8539, t8546, t8547, t8548)
}
