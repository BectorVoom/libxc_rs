//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1040/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1040<F: Float>(t6557: F, t7778: F, t903: F, t26857: F, t8410: F, t6355: F, t8542: F, t2283: F, t38355: F, t8571: F, t8582: F, t10194: F, t1550: F, t1668: F, t2868: F, t302: F, t4044: F, t40563: F, t40565: F, t40567: F, t40579: F, t46365: F, t46779: F, t5048: F, t6394: F, t6397: F, t665: F, t72: F, t8384: F, t8817: F, t884: F, t9128: F, t9840: F) -> F {
    let t47100 = t903 * t7778 * t6557;
    let t47108 = t26857 * t8410;
    let t47110 = t6355 * t8542;
    let t47112 = t38355 * t2283;
    let t47114 = t8571 * t8582;
    let t47116 = -t40563 - t40565 + t40567 + F::cast_from(0.23948483403727617128e0_f64) * t2868 * t8384 - t40579 - F::cast_from(0.71845450211182851384e0_f64) * t4044 * t665 * t6394 + F::cast_from(0.11974241701863808564e1_f64) * t5048 * t665 * t6397 - F::cast_from(0.4726e1_f64) * t1668 * t8817 - F::cast_from(0.11974241701863808564e0_f64) * t1550 * t46779 + F::cast_from(0.23948483403727617128e0_f64) * t47100 + F::cast_from(0.11974241701863808564e0_f64) * t884 * t46365 - F::cast_from(0.11974241701863808564e0_f64) * t9128 * t9840 + t72 * t302 * t10194 + F::cast_from(0.17961362552795712846e0_f64) * t47108 + F::cast_from(0.5987120850931904282e-1_f64) * t47110 - F::cast_from(0.85129199786595678796e-5_f64) * t47112 - F::cast_from(0.85129199786595678796e-5_f64) * t47114;
    t47116
}
