//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1162/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1162<F: Float>(t225: F, t31585: F, t22724: F, t31569: F, t31589: F, t6897: F, t794: F, t31668: F, t532: F, t1862: F, t8308: F, t31688: F, t31693: F, t31687: F, t8515: F, t9231: F) -> (F, F, F, F, F, F, F) {
    let t115619 = t31585 * t225;
    let t115629 = t22724 * t31569;
    let t115630 = 0.26044789391763585244e-1 * t115629;
    let t115658 = t6897 * t794 * t31589;
    let t115774 = t532 * t31668;
    let t115833 = t8308 * t1862;
    let t115837 = t31688 * t31693;
    let t115846 = t9231 * t31687 * t8515;
    (t115619, t115630, t115658, t115774, t115833, t115837, t115846)
}
