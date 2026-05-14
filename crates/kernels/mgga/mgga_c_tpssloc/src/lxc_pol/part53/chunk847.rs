//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 847/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk847<F: Float>(t214: F, t7191: F, t22751: F, t31645: F, t31612: F, t6883: F, t2085: F, t794: F, t22892: F, t6891: F, t22642: F, t22690: F, t31618: F, t31620: F, t552: F, t22893: F, t31619: F) -> (F, F, F, F, F, F, F, F, F) {
    let t115332 = t214 * t7191;
    let t115339 = t22751 * t31645;
    let t115341 = t6883 * t31612;
    let t115352 = t794 * t2085;
    let t115354 = t22892 * t115352 * t6891;
    let t115390 = t22642 * t22690 * t31618;
    let t115397 = t22751 * t31620;
    let t115399 = t552 * t7191;
    let t115409 = t22892 * t22893 * t31619;
    (t115332, t115339, t115341, t115352, t115354, t115390, t115397, t115399, t115409)
}
