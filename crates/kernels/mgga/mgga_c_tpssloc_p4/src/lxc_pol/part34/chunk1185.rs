//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1185/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1185<F: Float>(t1437: F, t5441: F, t72: F, t3953: F, t5392: F, t1433: F, t5389: F, t5399: F, t20201: F, t79: F, t5445: F, t20288: F) -> (F, F, F, F, F, F, F) {
    let t106813 = t72 * t5441 * t1437;
    let t106816 = t3953 * t5392;
    let t106826 = t72 * t1433 * t5389;
    let t106829 = t3953 * t5399;
    let t106836 = t72 * t79 * t20201;
    let t106842 = t72 * t1433 * t5445;
    let t106849 = t72 * t79 * t20288;
    (t106813, t106816, t106826, t106829, t106836, t106842, t106849)
}
