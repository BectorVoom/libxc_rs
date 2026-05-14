//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 489/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk489<F: Float>(t14267: F, t3056: F, t641: F, t2046: F, t2049: F, t2604: F, t3072: F, t13989: F, t7788: F, t13993: F, t7782: F, t14004: F, t7835: F, t265: F, t664: F) -> (F, F, F, F, F, F, F) {
    let t14269 = t3056 * t14267 * t641;
    let t14272 = t2046 * t2049 * t641;
    let t14274 = t2604 * t3072;
    let t14275 = 0.2993560425465952141e-1 * t14274;
    let t14276 = t7788 * t13989;
    let t14278 = t7782 * t13993;
    let t14280 = t7835 * t14004;
    let t14286 = t265 * t664;
    (t14269, t14272, t14275, t14276, t14278, t14280, t14286)
}
