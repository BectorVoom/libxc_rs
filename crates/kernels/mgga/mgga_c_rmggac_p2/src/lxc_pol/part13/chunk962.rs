//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 962/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk962<F: Float>(t30526: F, t8645: F, t3851: F, t39059: F, t38745: F, t39879: F, t3839: F, t39875: F, t3814: F, t39684: F, t40897: F, t25525: F, t40901: F) -> (F, F, F, F, F, F, F, F) {
    let t41134 = t30526 * t8645;
    let t41136 = t3851 * t39059;
    let t41138 = t3851 * t38745;
    let t41140 = t3851 * t39879;
    let t41142 = t3839 * t39875;
    let t41144 = t3814 * t39684;
    let t41146 = t3851 * t40897;
    let t41148 = t25525 * t40901;
    (t41134, t41136, t41138, t41140, t41142, t41144, t41146, t41148)
}
