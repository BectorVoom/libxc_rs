//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1072/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1072<F: Float>(t118: F, t159: F, t168: F, t2458: F, t2459: F, t2461: F, t2471: F, t2472: F, t2475: F, t2476: F, t2479: F, t2495: F, t2504: F, t2510: F, t2512: F, t39273: F, t39275: F, t39278: F, t39281: F, t39283: F, t39284: F, t39289: F, t39291: F, t39293: F, t39295: F, t39298: F, t39378: F, t39389: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t39664: F, t690: F, t725: F, t730: F, t731: F, t9730: F, t9733: F, t9739: F, t9758: F, t9892: F, t9905: F) -> (F,) {
    let t39803 = 0.12414243100625616072e5 * t9730 * t2471 * t9733 * t2461 + 0.1301229756036208781e0 * t690 * t9905 - 0.24828486201251232145e5 * t159 / t2475 / t2458 * t39664 * t9733 + 1.0 * t725 * (-0.39219166666666666667e1 * t39273 + 0.376504e2 * t39275 - 0.13944592592592592593e2 * t39278 + 0.12201518518518518519e2 * t39281 + 0.5356037037037037037e1 * t39284 + 0.14025833333333333333e0 * t39289 - 0.22441333333333333332e1 * t39291 + 0.24934814814814814815e1 * t39293 + 0.21817962962962962963e1 * t39295 + 0.16979925925925925926e1 * t39298) * t731 + 0.21053605041484726346e2 * t2510 * t2495 * t2504 - t39463 + t39468 + 0.51947577317044391277e2 * t2510 * t39389 * t2512 + t39472 + t39476 - 24.0 * t9739 * t39664 * t731 - t39483 - 0.55209406483950617283e-2 * t118 * t39283 * t168 + 0.6233709278045326953e3 * t9758 * t39378 * t2512 + 0.41096e0 * t690 * t2459 * t730 * t2472 - 0.6609050294782684211e1 * t690 * t2476 * t2471 * t2479 * t730 - 0.19263893255070628431e1 * t690 * t9892;
    (t39803,)
}
