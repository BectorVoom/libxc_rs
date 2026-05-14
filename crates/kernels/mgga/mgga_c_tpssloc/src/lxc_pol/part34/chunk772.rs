//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 772/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk772<F: Float>(t12155: F, t20356: F, t5279: F, t6347: F, t1347: F, t20416: F, t1819: F, t1821: F, t20536: F, t5278: F, t546: F, t548: F, t6404: F, t6408: F, t6411: F, t550: F) -> (F, F) {
    let t20544 = t12155 * t20356;
    let t20547 = t5279 * t6347;
    let t20550 = t1347 * t20416;
    let t20553 = -36.0 * t1819 * t6408 + 9.0 * t1819 * t6411 + 9.0 * t1821 * t6404 - t20536 * t548 + 60.0 * t20544 * t546 - 36.0 * t20547 * t5278 + 3.0 * t20550 * t546;
    let t20554 = t20553 * t550;
    (t20553, t20554)
}
