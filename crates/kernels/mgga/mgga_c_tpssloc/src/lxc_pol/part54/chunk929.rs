//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 929/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk929<F: Float>(t645: F, t72: F, t7431: F, t1437: F, t1864: F, t1410: F, t2240: F, t4017: F, t71: F, t12568: F, t33: F, t3953: F, t608: F, t641: F, t4021: F, t79: F) -> (F, F, F, F, F, F, F, F) {
    let t26009 = t72 * t7431 * t645;
    let t26012 = t1864 * t1437;
    let t26016 = t2240 * t1410;
    let t26024 = t71 * t4017;
    let t26028 = t12568 * t33;
    let t26055 = t3953 * t608;
    let t26062 = t641 * t1437;
    let t26063 = t72 * t26062;
    let t26066 = t79 * t4021;
    (t26009, t26012, t26016, t26024, t26028, t26055, t26063, t26066)
}
