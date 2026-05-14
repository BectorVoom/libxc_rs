//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1016/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1016<F: Float>(t11066: F, t3040: F, t360: F, t6739: F, t135: F, t457: F, t1090: F, t7319: F, t11545: F, t60: F, t461: F, t11588: F, t134: F, t3439: F, t3507: F, t475: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14590 = t11066 * t3040;
    let t14630 = t6739 * t3040 * t360;
    let t15281 = t135 * t457;
    let t15288 = t7319 * t1090;
    let t15394 = t60 * t11545;
    let t15395 = t15394 * t461;
    let t15402 = t11588 * t461;
    let t15418 = t134 * t3439;
    let t15419 = t15418 * t461;
    let t15429 = t6739 * t3507 * t475;
    (t14590, t14630, t15281, t15288, t15394, t15395, t15402, t15418, t15419, t15429)
}
