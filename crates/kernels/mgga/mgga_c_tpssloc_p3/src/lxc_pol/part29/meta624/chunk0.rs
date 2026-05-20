//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2066/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2066<F: Float>(t3469: F, t52: F, t24682: F, t460: F, t3475: F, t11702: F, t7339: F, t24684: F, t27634: F, t1210: F, t24654: F, t24721: F) -> (F, F, F, F, F, F, F) {
    let t86197 = t52 * t3469;
    let t86199 = t24682 * t86197 * t460;
    let t86202 = t52 * t3475;
    let t86204 = t24682 * t86202 * t460;
    let t86228 = t7339 * t11702;
    let t86234 = t27634 * t24684;
    let t86248 = t24721 * t1210 * t24654;
    (t86197, t86199, t86202, t86204, t86228, t86234, t86248)
}
