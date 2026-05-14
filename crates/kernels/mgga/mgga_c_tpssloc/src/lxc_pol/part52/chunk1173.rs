//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1173/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1173<F: Float>(t33133: F, t6997: F, t24987: F, t8490: F, t1437: F, t31: F, t607: F, t1410: F, t645: F, t6504: F, t8308: F, t641: F, t7440: F, t1433: F, t33106: F, t8513: F) -> (F, F, F, F, F, F, F, F, F) {
    let t119875 = t33133 * t6997;
    let t119877 = t24987 * t8490;
    let t119878 = t1437 * t31;
    let t119879 = t119878 * t607;
    let t119883 = t1410 * t645;
    let t119888 = t8308 * t1410 * t6504;
    let t119891 = t1410 * t641;
    let t119897 = t8308 * t7440 * t31 * t607;
    let t119901 = t1433 * t31 * t607;
    let t119913 = t8513 * t33106 * t6504;
    (t119875, t119877, t119879, t119883, t119888, t119891, t119897, t119901, t119913)
}
