//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 967/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk967<F: Float>(t2289: F, t38638: F, t1356: F, t1923: F, t2131: F, t35566: F, t40085: F, t40087: F, t40089: F, t45994: F, t45999: F, t46001: F, t46003: F, t46005: F, t46018: F, t46020: F, t46022: F, t46024: F, t4985: F, t5879: F, t6355: F, t7399: F, t7703: F, t8371: F, t8399: F) -> F {
    let t46026 = t38638 * t2289;
    let t46028 = -F::new(0.23948483403727617128e0) * t6355 * t8371 - t35566 + F::new(0.85129199786595678796e-5) * t45994 - F::new(0.23942587439980034662e-4) * t45999 + t40085 + t40087 + t40089 + F::new(0.44903406381989282115e-1) * t46001 - F::new(0.17961362552795712846e0) * t46003 - F::new(0.11974241701863808564e0) * t1356 * t7703 * t46005 - F::new(0.23948483403727617128e0) * t4985 * t8399 - F::new(0.2363e1) * t5879 * t2131 - F::new(0.2363e1) * t1923 * t7399 - F::new(0.53205749866622299248e-5) * t46018 - F::new(0.25538759935978703638e-4) * t46020 - F::new(0.25538759935978703638e-4) * t46022 + F::new(0.1064114997332445985e-4) * t46024 + F::new(0.59590439850616975155e-4) * t46026;
    t46028
}
