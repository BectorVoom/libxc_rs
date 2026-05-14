//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1157/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1157<F: Float>(t2085: F, t794: F, t22892: F, t6891: F, t22642: F, t22690: F, t31618: F, t22751: F, t31620: F, t552: F, t7191: F, t22893: F, t31619: F, t31628: F, t6914: F, t22704: F, t22705: F, t31627: F) -> (F, F, F, F, F, F, F, F) {
    let t115352 = t794 * t2085;
    let t115354 = t22892 * t115352 * t6891;
    let t115390 = t22642 * t22690 * t31618;
    let t115391 = 0.82246703342411321824e-2 * t115390;
    let t115397 = t22751 * t31620;
    let t115399 = t552 * t7191;
    let t115409 = t22892 * t22893 * t31619;
    let t115415 = t6914 * t31628;
    let t115423 = t22704 * t22705 * t31627;
    (t115352, t115354, t115391, t115397, t115399, t115409, t115415, t115423)
}
