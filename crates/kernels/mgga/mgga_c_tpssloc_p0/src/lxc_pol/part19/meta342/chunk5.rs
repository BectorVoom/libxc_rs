//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1224/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1224<F: Float>(t41186: F, t41229: F, t225: F, t2639: F, t9960: F, t39249: F, t39256: F, t39309: F, t39312: F, t39316: F, t39320: F, t40627: F, t40663: F, t40668: F, t40671: F, t40674: F) -> (F, F, F, F) {
    let t41230 = t41186 + t41229;
    let t41231 = t41230 * t225;
    let t41237 = t2639 * t9960;
    let t41241 = -t39249 + t40627 + t40663 - t39256 - t39309 + t39312 + t39316 + t39320 - t40668 - t40671 + t40674;
    (t41230, t41231, t41237, t41241)
}
