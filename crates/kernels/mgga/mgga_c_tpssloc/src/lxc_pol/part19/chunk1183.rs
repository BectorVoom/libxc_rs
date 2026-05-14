//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1183/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1183<F: Float>(t2403: F, t2830: F, t10317: F, t699: F, t136: F, t2826: F, t41697: F, t41701: F, t41709: F, t908: F, t41640: F, t41642: F, t41656: F, t41658: F, t41660: F, t41662: F, t41669: F, t41673: F, t41675: F) -> (F, F, F, F, F, F, F) {
    let t41831 = t2403 * t2830;
    let t41833 = t699 * t10317;
    let t41836 = t136 * t2826 * t41697;
    let t41839 = t136 * t2826 * t41701;
    let t41842 = t136 * t908 * t41709;
    let t41845 = t136 * t908 * t41640;
    let t41855 = 0.11038e1 * t41831 + 0.132456e1 * t41833 - 0.99342e0 * t41836 - 0.82785e-1 * t41839 + 0.198684e1 * t41842 + 0.49671e0 * t41845 + 0.181155e1 * t41642 - 0.80513333333333333336e0 * t41656 - 0.53675555555555555556e0 * t41658 + 0.44729629629629629629e0 * t41660 + 0.40256666666666666668e0 * t41662 - 0.89459259259259259259e0 * t41669 - 0.301925e0 * t41673 + 0.16102666666666666667e1 * t41675;
    (t41831, t41833, t41836, t41839, t41842, t41845, t41855)
}
