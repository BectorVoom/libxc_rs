//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1123/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1123<F: Float>(t25854: F, t27048: F, t326: F, t40806: F, t44029: F, t44035: F, t46056: F, t46059: F, t46062: F, t46064: F, t46066: F, t46069: F, t48271: F, t48278: F, t48976: F) -> F {
    let t49294 = -F::cast_from(0.59871208509319042821e-1_f64) * t326 * t48976 - t44029 - F::cast_from(0.47896966807455234255e0_f64) * t40806 - F::cast_from(0.95793933614910468512e0_f64) * t46056 - F::cast_from(0.15965655602485078085e0_f64) * t46059 + t44035 + F::cast_from(0.71845450211182851384e0_f64) * t25854 * t48271 + F::cast_from(0.71845450211182851384e0_f64) * t27048 * t48278 - F::cast_from(0.40911992481368012596e-1_f64) * t46062 - F::cast_from(0.5987120850931904282e-1_f64) * t46064 + F::cast_from(0.11974241701863808564e0_f64) * t46066 - F::cast_from(0.17961362552795712846e0_f64) * t46069;
    t49294
}
