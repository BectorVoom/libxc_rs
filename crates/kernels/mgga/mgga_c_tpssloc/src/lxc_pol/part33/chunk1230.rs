//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1230/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1230<F: Float>(t1433: F, t5389: F, t72: F, t3953: F, t5399: F, t20201: F, t79: F, t5445: F, t1865: F, t22544: F, t26051: F, t26084: F, t27961: F, t27976: F, t27979: F, t6490: F, t7432: F, t7442: F, t7446: F, t83830: F, t90192: F, t90330: F, t96547: F) -> (F,) {
    let t106826 = t72 * t1433 * t5389;
    let t106829 = t3953 * t5399;
    let t106836 = t72 * t79 * t20201;
    let t106842 = t72 * t1433 * t5445;
    let t106847 = -15.0 * t90330 * t27961 - 15.0 * t90192 * t27961 - 15.0 * t22544 * t106826 + t106829 * t1865 + 5.0 / 2.0 * t26084 * t27976 - 5.0 * t96547 * t7432 + 35.0 * t83830 * t106836 + t27979 * t7442 + t27979 * t7446 + 5.0 / 2.0 * t6490 * t106842 + 5.0 / 2.0 * t26051 * t27976;
    (t106847,)
}
