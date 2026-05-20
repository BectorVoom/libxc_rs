//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2889/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2889<F: Float>(t17279: F, t699: F, t17240: F, t17243: F, t136: F, t2826: F, t59715: F, t10304: F, t59751: F, t59719: F, t59706: F, t41880: F, t59711: F) -> (F, F, F, F, F, F, F, F) {
    let t60308 = t699 * t17279;
    let t60310 = t699 * t17240;
    let t60312 = t699 * t17243;
    let t60315 = t136 * t2826 * t59715;
    let t60318 = t136 * t10304 * t59751;
    let t60321 = t136 * t2826 * t59719;
    let t60324 = t136 * t10304 * t59706;
    let t60327 = t136 * t41880 * t59711;
    (t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327)
}
