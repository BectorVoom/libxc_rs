//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1114/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1114<F: Float>(t36168: F, t41355: F, t41358: F, t46232: F, t46235: F, t46238: F, t46242: F, t46244: F, t46246: F, t46248: F, t46250: F, t46252: F, t46254: F, t46256: F, t46259: F, t46262: F) -> F {
    let t49126 = F::cast_from(0.11974241701863808564e0_f64) * t46232 - F::cast_from(0.79656924630363488034e-2_f64) * t46235 + F::cast_from(0.15931384926072697607e-1_f64) * t46238 - F::cast_from(0.17779038707952519053e0_f64) * t41355 - t41358 + F::cast_from(0.2927036860455597649e0_f64) * t36168 + F::cast_from(0.79656924630363488034e-2_f64) * t46242 - F::cast_from(0.27879923620627220812e-1_f64) * t46244 + F::cast_from(0.44607877793003553299e-1_f64) * t46246 + F::cast_from(0.5987120850931904282e0_f64) * t46248 - F::cast_from(0.23948483403727617128e0_f64) * t46250 + F::cast_from(0.11974241701863808564e0_f64) * t46252 + F::cast_from(0.11974241701863808564e0_f64) * t46254 - F::cast_from(0.23948483403727617128e0_f64) * t46256 - F::cast_from(0.11974241701863808564e0_f64) * t46259 + F::cast_from(0.59871208509319042821e-1_f64) * t46262;
    t49126
}
