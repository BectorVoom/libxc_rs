//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1180/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1180<F: Float>(t136: F, t14795: F, t1113: F, t14744: F, t11265: F, t1661: F, t3271: F, t11243: F, t3270: F, t4756: F, t1102: F, t3279: F, t4748: F) -> (F, F, F, F, F, F) {
    let t14796 = t136 * t14795;
    let t14798 = t1113 * t14744;
    let t14799 = t136 * t14798;
    let t14801 = t11265 * t1661;
    let t14802 = t14801 * t3271;
    let t14804 = t11243 * t1661;
    let t14805 = t14804 * t3271;
    let t14808 = t3270 * t4756;
    let t14809 = t14808 * t1102;
    let t14811 = t4748 * t3279;
    (t14796, t14799, t14802, t14805, t14809, t14811)
}
