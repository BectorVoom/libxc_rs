//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1117/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1117<F: Float>(t15518: F, t15542: F, t15566: F, t15802: F, t15826: F, t15860: F, t15880: F, t15927: F, t219: F, t5271: F, t1148: F, t5275: F, t9739: F, t1586: F, t3118: F, t4322: F) -> (F, F, F, F, F) {
    let t15930 = t15518 + t15542 + t15566 + t15802 + t15826 + t15860 + t15880 + t15927;
    let t15931 = param_beta * t15930;
    let t15933 = t5271 * t219;
    let t15944 = t9739 * t5275 * t1148;
    let t15948 = t3118 * t1586 * t4322;
    (t15930, t15931, t15933, t15944, t15948)
}
