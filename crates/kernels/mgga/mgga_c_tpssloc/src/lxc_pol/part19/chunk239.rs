//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 239/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk239<F: Float>(t111: F, t89: F, t107: F, t626: F, t106: F, t38: F, t606: F, tau0: F) -> (F, F, F, F, F, F) {
    let t652 = t89 * t111;
    let t654 = t626 * t107 / 3.0;
    let t655 = t106 * t106;
    let t656 = 1.0 / t655;
    let t657 = tau0 * t38;
    let t659 = t606 / 2.0;
    (t652, t654, t655, t656, t657, t659)
}
