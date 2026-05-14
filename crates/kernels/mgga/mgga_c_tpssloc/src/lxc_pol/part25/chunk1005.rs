//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1005/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1005<F: Float>(t22635: F, t26331: F, t3719: F, t81330: F, t6888: F, t6891: F, t80707: F, t1377: F, t1385: F, t22633: F, t22638: F, t81159: F, t22892: F, t80645: F, t6892: F, t81186: F) -> (F, F, F, F, F, F) {
    let t81333 = t26331 * t22635 * t81330 * t3719;
    let t81339 = t6888 * t80707 * t6891;
    let t81346 = t22633 * t22635 * t1377 * t3719 * t1385;
    let t81350 = t81159 * t22638;
    let t81365 = t22892 * t80645 * t6891;
    let t81375 = t81186 * t6892;
    (t81333, t81339, t81346, t81350, t81365, t81375)
}
