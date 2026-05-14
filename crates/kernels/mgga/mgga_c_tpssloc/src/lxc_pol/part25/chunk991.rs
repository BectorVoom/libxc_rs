//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 991/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk991<F: Float>(t2229: F, t583: F, t60: F, t1995: F, t22816: F, t22818: F, t22827: F, t3788: F, t3792: F, t54770: F, t1339: F, t54591: F, t550: F, t40197: F, t54858: F, t6936: F) -> (F, F, F, F, F, F) {
    let t80967 = 1.0 / t60 / t2229 / t583;
    let t80970 = t80967 * t1995 * t22816 * t22818;
    let t80974 = t22827 * t3788 * t54770 * t3792;
    let t80978 = t22827 * t1339 * t54591 * t550;
    let t80982 = t22827 * t1339 * t40197 * t550;
    let t80985 = t6936 * t3788 * t54858;
    (t80967, t80970, t80974, t80978, t80982, t80985)
}
