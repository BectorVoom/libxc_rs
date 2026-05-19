//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 996/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk996<F: Float>(t118: F, t25854: F, t27048: F, t27101: F, t326: F, t46375: F, t46379: F, t46382: F, t46386: F, t46389: F, t46392: F, t46395: F, t46398: F, t46400: F, t46403: F, t46406: F, t46409: F) -> F {
    let t46411 = -F::cast_from(0.11974241701863808564e0_f64) * t326 * t46375 - F::cast_from(0.39914139006212695214e-1_f64) * t118 * t46379 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t46382 + F::cast_from(0.17961362552795712846e0_f64) * t46386 + F::cast_from(0.11974241701863808564e0_f64) * t46389 + F::cast_from(0.71845450211182851384e0_f64) * t46392 + F::cast_from(0.17961362552795712846e0_f64) * t46395 - F::cast_from(0.17961362552795712846e0_f64) * t46398 - F::cast_from(0.47896966807455234256e0_f64) * t27101 * t46400 + F::cast_from(0.71845450211182851384e0_f64) * t25854 * t46403 + F::cast_from(0.71845450211182851384e0_f64) * t27048 * t46406 + F::cast_from(0.40911992481368012592e-1_f64) * t46409;
    t46411
}
