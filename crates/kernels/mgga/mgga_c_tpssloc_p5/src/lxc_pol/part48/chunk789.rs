//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 789/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk789<F: Float>(t3630: F, t7301: F, t7300: F, t1235: F, t7299: F, t7302: F, t2123: F, t3477: F, t2127: F, t23383: F) -> (F, F, F, F) {
    let t24563 = t7301 * t3630;
    let t24564 = t7300 * t24563;
    let t24567 = t7299 * t1235;
    let t24568 = t24567 * t7302;
    let t24571 = t3477 * t2123;
    let t24574 = t2127 * t23383;
    (t24564, t24568, t24571, t24574)
}
