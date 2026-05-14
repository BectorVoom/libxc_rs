//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1182/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1182<F: Float>(t1845: F, t6995: F, t1799: F, t1437: F, t31: F, t607: F, t1410: F, t645: F, t641: F, t1433: F, t32: F, t26502: F, t3701: F, t26114: F, t8327: F, t19456: F, t8326: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t119832 = t1845 * t6995;
    let t119853 = t1799 * t6995;
    let t119878 = t1437 * t31;
    let t119879 = t119878 * t607;
    let t119883 = t1410 * t645;
    let t119891 = t1410 * t641;
    let t119901 = t1433 * t31 * t607;
    let t119931 = t32 * t607;
    let t120016 = t3701 * t26502;
    let t120067 = 2.0 * t26114 * t8327;
    let t120120 = t19456 * t8326;
    (t119832, t119853, t119879, t119883, t119891, t119901, t119931, t120016, t120067, t120120)
}
