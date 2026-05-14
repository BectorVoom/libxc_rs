//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 835/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk835<F: Float>(t23562: F, t23563: F, t23384: F, t6692: F, t1049: F, t6688: F, t1054: F, t1065: F, t1921: F, t3034: F, t38: F, t131: F, t350: F, t3030: F, t344: F, t225: F, t6733: F) -> (F, F, F, F, F, F, F, F) {
    let t23564 = t23562 * t23563;
    let t23579 = t23384 * t6692;
    let t23581 = t6688 * t1049;
    let t23587 = t1054 * t1065;
    let t23588 = t1921 * t23587;
    let t23598 = 1.0 / t3034;
    let t23599 = t38 * t23598;
    let t23600 = t23599 * t131;
    let t23601 = t23600 * t350;
    let t23602 = t344 * t3030;
    let t23613 = t6733 * t225;
    (t23564, t23579, t23581, t23587, t23588, t23601, t23602, t23613)
}
