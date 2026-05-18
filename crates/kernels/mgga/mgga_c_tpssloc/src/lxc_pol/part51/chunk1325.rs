//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1325/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1325<F: Float>(t119878: F, t607: F, t1410: F, t645: F, t6504: F, t8308: F, t641: F, t31: F, t7440: F, t1433: F, t33106: F, t8513: F) -> (F, F, F, F, F, F, F) {
    let t119879 = t119878 * t607;
    let t119883 = t1410 * t645;
    let t119888 = t8308 * t1410 * t6504;
    let t119891 = t1410 * t641;
    let t119897 = t8308 * t7440 * t31 * t607;
    let t119901 = t1433 * t31 * t607;
    let t119913 = t8513 * t33106 * t6504;
    (t119879, t119883, t119888, t119891, t119897, t119901, t119913)
}
