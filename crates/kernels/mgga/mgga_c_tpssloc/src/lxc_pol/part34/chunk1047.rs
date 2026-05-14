//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1047/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1047<F: Float>(t1799: F, t6463: F, t20347: F, t88: F, t20304: F, t81446: F, t22473: F, t75603: F, t20342: F, t6530: F, t1458: F, t5449: F, t1845: F, t6330: F, t22633: F, t22635: F, t26337: F, t6460: F) -> (F, F, F, F, F, F, F, F) {
    let t106902 = t1799 * t6463;
    let t106935 = t88 * t20347;
    let t106944 = t81446 * t20304;
    let t106946 = t22473 * t75603;
    let t106948 = t6530 * t20342;
    let t106956 = t5449 * t1458;
    let t106971 = t6330 * t1845;
    let t106982 = t22633 * t22635 * t26337 * t6460;
    (t106902, t106935, t106944, t106946, t106948, t106956, t106971, t106982)
}
