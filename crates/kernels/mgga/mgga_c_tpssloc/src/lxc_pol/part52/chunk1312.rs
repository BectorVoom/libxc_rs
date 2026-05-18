//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1312/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1312<F: Float>(t31: F, t607: F, t7440: F, t8308: F, t1433: F, t33106: F, t6504: F, t8513: F, t32: F, t33114: F, t645: F, t79: F) -> (F, F, F, F, F, F) {
    let t119897 = t8308 * t7440 * t31 * t607;
    let t119901 = t1433 * t31 * t607;
    let t119913 = t8513 * t33106 * t6504;
    let t119931 = t32 * t607;
    let t119938 = t8513 * t33114 * t645;
    let t119942 = t79 * t7440;
    (t119897, t119901, t119913, t119931, t119938, t119942)
}
