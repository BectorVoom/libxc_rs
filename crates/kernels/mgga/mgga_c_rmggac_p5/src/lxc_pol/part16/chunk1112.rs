//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1112/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1112<F: Float>(t41257: F, t41265: F, t41271: F, t41299: F, t41302: F, t43588: F, t43592: F, t43594: F, t43596: F, t46189: F, t46191: F, t46193: F, t46195: F, t46197: F, t46199: F, t46201: F) -> F {
    let t49095 = -F::cast_from(0.4838420607177634088e-3_f64) * t46189 + F::cast_from(0.56448240417072397693e-3_f64) * t46191 - F::cast_from(0.36366215538993788973e-1_f64) * t46193 - F::cast_from(0.12122071846331262991e0_f64) * t46195 + F::cast_from(0.58540737209111952978e0_f64) * t41257 - F::cast_from(0.12981128458281457309e-1_f64) * t41265 - F::cast_from(0.66380770525302906695e-3_f64) * t46197 + F::cast_from(0.2993560425465952141e-1_f64) * t46199 + F::cast_from(0.53218852008283593619e-1_f64) * t46201 - F::cast_from(0.41395376305853091643e-2_f64) * t41271 - t43588 - F::cast_from(0.90317184667315836312e-2_f64) * t41299 - F::cast_from(0.72732431077987577944e-1_f64) * t41302 + t43592 + t43594 - t43596;
    t49095
}
