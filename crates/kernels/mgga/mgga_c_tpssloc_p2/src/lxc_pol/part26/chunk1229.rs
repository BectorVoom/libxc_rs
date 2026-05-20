//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1229/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1229<F: Float>(t22765: F, t3858: F, t22764: F, t3777: F, t1354: F, t22756: F, t1336: F, t22759: F, t835: F, t3795: F, t22760: F, t3853: F) -> (F, F, F, F, F, F) {
    let t80989 = t22765 * t3858;
    let t80991 = t3777 * t22764;
    let t80992 = t80991 * t1354;
    let t80994 = t22756 * t3858;
    let t80997 = t1336 * t22759 * t835;
    let t80998 = t80997 * t3795;
    let t81000 = t3777 * t22760;
    let t81001 = t81000 * t3795;
    let t81003 = t22756 * t3853;
    (t80989, t80992, t80994, t80998, t81001, t81003)
}
