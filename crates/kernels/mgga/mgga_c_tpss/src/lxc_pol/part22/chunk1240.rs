//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1240/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1240<F: Float>(t60725: F, t60731: F, t60733: F, t60739: F, t60744: F, t60750: F, t60752: F, t65636: F, t65641: F, t65645: F, t67183: F, t67185: F, t67187: F, t67149: F, t67163: F, t67177: F) -> (F,) {
    let t67191 = -7.0 / 24.0 * t60725 - 35.0 / 54.0 * t60731 + 7.0 / 72.0 * t60733 + t65636 / 192.0 - 7.0 / 144.0 * t60739 - t67183 + t65641 / 192.0 + t67185 - t65645 / 192.0 - t67187 - 35.0 / 288.0 * t60744 - 119.0 / 432.0 * t60750 + 7.0 / 288.0 * t60752;
    let t67193 = t67149 + t67163 + t67177 + t67191;
    (t67193,)
}
