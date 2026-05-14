//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 915/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk915<F: Float>(t11714: F, t478: F, t10477: F, t483: F, t11713: F, t3508: F, t475: F, t3503: F, t11708: F, t3514: F, t1210: F, t248: F, t3509: F, t3570: F, t3506: F, t135: F, t3561: F) -> (F, F, F, F, F, F, F, F) {
    let t11715 = 1.0 / t11714;
    let t11716 = t11715 * t478;
    let t11717 = t483 * t10477;
    let t11718 = t11716 * t11717;
    let t11719 = t11713 * t11718;
    let t11721 = t3508 * t475;
    let t11727 = t3503 * t11717;
    let t11728 = t11713 * t11727;
    let t11734 = t11708 * t3514;
    let t11737 = t1210 * t11717;
    let t11738 = t11713 * t11737;
    let t11745 = t248 * t3570 * t3509;
    let t11746 = t3506 * t11745;
    let t11754 = t135 * t3561;
    (t11715, t11719, t11721, t11728, t11734, t11738, t11746, t11754)
}
