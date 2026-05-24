//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 973/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk973<F: Float>(t36119: F, t46109: F, t36103: F, t46106: F, t36110: F, t36: F, t5840: F, t262: F, t2115: F, t41146: F, t41160: F, t41171: F, t46084: F, t46087: F, t46090: F, t46093: F, t46096: F, t46099: F, t46102: F, t46107: F) -> (F, F, F) {
    let t46110 = t36119 * t46109;
    let t46112 = t36103 * t46106;
    let t46114 = t36110 * t46109;
    let t46116 = t36 * t5840;
    let t46117 = t262 * t46116;
    let t46118 = t2115 * t46117;
    let t46120 = -F::cast_from(0.2727466165424534173e-1_f64) * t46084 + F::cast_from(0.13637330827122670865e-1_f64) * t46087 - F::cast_from(0.2727466165424534173e-1_f64) * t46090 - F::cast_from(0.2727466165424534173e-1_f64) * t46093 + F::cast_from(0.68186654135613354324e-1_f64) * t46096 - F::cast_from(0.13637330827122670865e0_f64) * t46099 + F::cast_from(0.45457769423742236216e-1_f64) * t46102 - F::cast_from(0.15965655602485078086e0_f64) * t41146 + F::cast_from(0.7080615522698976714e-2_f64) * t41160 - t41171 - F::cast_from(0.5454932330849068346e-1_f64) * t46107 + F::cast_from(0.13637330827122670865e0_f64) * t46110 - F::cast_from(0.63504270469206447405e-2_f64) * t46112 + F::cast_from(0.10160683275073031585e-1_f64) * t46114 + F::cast_from(0.9072038638458063915e-4_f64) * t46118;
    (t46116, t46117, t46120)
}
