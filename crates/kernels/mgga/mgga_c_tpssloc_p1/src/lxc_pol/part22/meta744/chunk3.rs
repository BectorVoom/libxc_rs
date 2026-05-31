//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2471/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2471<F: Float>(t1023: F, t10390: F, t1041: F, t17637: F, t17643: F, t21134: F, t21403: F, t21532: F, t21574: F, t3070: F, t3071: F, t42397: F, t42483: F, t42508: F, t4582: F, t4583: F, t4644: F, t4650: F, t48607: F, t49854: F, t5685: F, t69643: F, t70316: F, t70321: F, t70330: F, t884: F) -> F {
    let t70335 = t42508 * t21532 / F::cast_from(288.0_f64) + t42483 * t3071 * t21403 * t884 / F::cast_from(4608.0_f64) + t3070 * t3071 * t5685 * t4650 / F::cast_from(1536.0_f64) + t3070 * t3071 * t21134 * t1023 / F::cast_from(4608.0_f64) + t10390 * t21574 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t48607 * t42397 * t69643 - t1041 * t4582 * t4583 * t70316 / F::cast_from(768.0_f64) - t1041 * t4582 * t4583 * t70321 / F::cast_from(768.0_f64) - t4644 * t17637 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t4644 * t17643 - F::cast_from(5.0_f64) / F::cast_from(432.0_f64) * t1041 * t4582 * t49854 * t70330;
    t70335
}
