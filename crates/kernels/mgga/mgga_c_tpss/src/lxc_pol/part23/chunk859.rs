//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 859/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk859<F: Float>(t114: F, t6106: F, t626: F, t1333: F, t5527: F, t5526: F) -> (F, F) {
    let t115 = 1.0 < t114;
    let t6108 = 2.0 * t626 * t6106;
    let t6109 = t5527 * t1333;
    let t6112 = piecewise3(t115, 0.0, -t5526 - t6109 / 8.0);
    (t6108, t6112)
}
