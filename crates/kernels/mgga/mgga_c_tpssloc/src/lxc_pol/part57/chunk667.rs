//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 667/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk667<F: Float>(t1921: F, t25749: F, t7561: F, t968: F, t1920: F, t1625: F, t6688: F, t23384: F, t7557: F, t3216: F, t7627: F, t10143: F, t28: F, t1437: F, t1864: F, t1863: F) -> (F, F, F, F, F, F, F, F) {
    let t25784 = t1921 * t25749;
    let t25806 = t968 * t7561;
    let t25807 = t1920 * t25806;
    let t25810 = t6688 * t1625;
    let t25824 = t23384 * t7557;
    let t25840 = t7627 * t3216;
    let t25927 = t10143 * t28;
    let t26012 = t1864 * t1437;
    let t26013 = t1863 * t26012;
    (t25784, t25807, t25810, t25824, t25840, t25927, t26012, t26013)
}
