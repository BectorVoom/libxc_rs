//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1099/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1099<F: Float>(t119878: F, t607: F, t8308: F, t1410: F, t645: F, t6504: F, t641: F, t113875: F, t31: F, t7440: F, t1433: F, t12571: F, t31009: F, t33106: F, t8513: F, t7431: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t119879 = t119878 * t607;
    let t119880 = t8308 * t119879;
    let t119883 = t1410 * t645;
    let t119884 = t8308 * t119883;
    let t119888 = t8308 * t1410 * t6504;
    let t119891 = t1410 * t641;
    let t119892 = t113875 * t119891;
    let t119897 = t8308 * t7440 * t31 * t607;
    let t119901 = t1433 * t31 * t607;
    let t119902 = t113875 * t119901;
    let t119905 = t12571 * t31009;
    let t119909 = t8513 * t33106 * t645;
    let t119913 = t8513 * t33106 * t6504;
    let t119917 = t8513 * t7431 * t641;
    (t119880, t119884, t119888, t119892, t119897, t119902, t119905, t119909, t119913, t119917)
}
