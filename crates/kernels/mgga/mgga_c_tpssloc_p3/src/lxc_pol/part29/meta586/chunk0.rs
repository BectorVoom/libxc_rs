//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2008/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2008<F: Float>(t1354: F, t80991: F, t1336: F, t22759: F, t835: F, t3795: F, t22765: F, t3853: F, t22704: F, t22898: F, t80798: F, t12248: F, t6604: F) -> (F, F, F, F, F) {
    let t80992 = t80991 * t1354;
    let t80997 = t1336 * t22759 * t835;
    let t80998 = t80997 * t3795;
    let t81007 = t22765 * t3853;
    let t81022 = t22704 * t80798 * t22898;
    let t81027 = t6604 * t12248;
    (t80992, t80998, t81007, t81022, t81027)
}
