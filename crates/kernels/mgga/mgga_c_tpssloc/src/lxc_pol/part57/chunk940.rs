//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 940/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk940<F: Float>(t234: F, t7823: F, t23110: F, t23185: F, t33379: F, t23168: F, t33376: F, t33380: F, t6579: F, t33384: F, t6547: F, t33429: F) -> (F, F, F, F, F, F) {
    let t121506 = t234 * t7823;
    let t121524 = t23185 * t23110 * t33379;
    let t121533 = t23168 * t33376;
    let t121536 = t6579 * t33380;
    let t121574 = t6547 * t33384;
    let t121629 = t6547 * t33429;
    (t121506, t121524, t121533, t121536, t121574, t121629)
}
