//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 715/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk715<F: Float>(t234: F, t6604: F, t1484: F, t252: F, t776: F, t25038: F, t7528: F, t794: F, t6562: F, t13380: F, t232: F, t6646: F, t1888: F, t6579: F, t7525: F, t4292: F) -> (F, F, F, F, F, F, F) {
    let t25248 = t6604 * t234;
    let t25249 = t252 * t1484;
    let t25250 = t25249 * t776;
    let t25251 = t25248 * t25250;
    let t25252 = t25038 * t25251;
    let t25258 = t794 * t7528;
    let t25259 = t6562 * t25258;
    let t25272 = t13380 * t232;
    let t25273 = t6646 * t25272;
    let t25274 = t1888 * t25273;
    let t25277 = t6579 * t7525;
    let t25284 = t6646 * t4292;
    (t25248, t25249, t25252, t25259, t25274, t25277, t25284)
}
