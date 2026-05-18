//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1188/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1188<F: Float>(t1985: F, t7700: F, t97511: F, t1842: F, t22635: F, t26331: F, t96922: F, t1992: F, t26354: F, t6460: F, t22633: F, t97637: F) -> (F, F, F, F) {
    let t106986 = t1985 * t97511 * t7700;
    let t106991 = t26331 * t22635 * t96922 * t1842;
    let t107007 = t1992 * t22635 * t26354 * t6460;
    let t107015 = t22633 * t22635 * t97637 * t1842;
    (t106986, t106991, t107007, t107015)
}
