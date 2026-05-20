//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1614/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1614<F: Float>(t25249: F, t776: F, t25248: F, t25038: F, t7528: F, t794: F, t6562: F, t13380: F, t232: F, t6646: F, t1888: F, t6579: F, t7525: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25250 = t25249 * t776;
    let t25251 = t25248 * t25250;
    let t25252 = t25038 * t25251;
    let t25258 = t794 * t7528;
    let t25259 = t6562 * t25258;
    let t25272 = t13380 * t232;
    let t25273 = t6646 * t25272;
    let t25274 = t1888 * t25273;
    let t25277 = t6579 * t7525;
    (t25250, t25251, t25252, t25258, t25259, t25272, t25273, t25274, t25277)
}
