//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2583/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2583<F: Float>(t11292: F, t1687: F, t11365: F, t1694: F, t3331: F, t4794: F, t14933: F, t300: F, t3401: F, t11310: F, t15823: F, t225: F) -> (F, F, F, F, F, F, F, F) {
    let t51680 = t1687 * t11292;
    let t51727 = t11365 * t1694;
    let t51730 = t4794 * t3331;
    let t51807 = t300 * t14933;
    let t51810 = t300 * t3401;
    let t51819 = t300 * t11310;
    let t51848 = t300 * t11365;
    let t51925 = t15823 * t225;
    (t51680, t51727, t51730, t51807, t51810, t51819, t51848, t51925)
}
