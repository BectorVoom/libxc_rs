//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 656/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk656<F: Float>(t1114: F, t4231: F, t3931: F, t1569: F, t943: F, t1108: F, t938: F) -> (F, F, F) {
    let t4252 = t4231 * t1114;
    let t4253 = t3931 * t4252;
    let t4256 = t1569 * t943;
    let t4258 = t938 * t1108 * t4256;
    (t4252, t4253, t4258)
}
