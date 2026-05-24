//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1082/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1082<F: Float>(t41265: F, t36127: F, t36141: F, t36152: F, t36154: F, t41263: F, t41271: F, t41274: F, t41277: F, t41279: F, t41281: F, t41283: F, t41285: F, t41287: F, t41289: F, t41291: F) -> F {
    let t43571 = F::cast_from(0.64905642291407286545e-2_f64) * t41265;
    let t43586 = F::cast_from(0.31862769852145395214e-1_f64) * t41263 - t43571 - F::cast_from(0.3193131120497015617e0_f64) * t36127 - F::cast_from(0.21241846568096930144e-1_f64) * t36141 + F::cast_from(0.31931311204970156171e0_f64) * t36152 + F::cast_from(0.53218852008283593618e-1_f64) * t36154 - F::cast_from(0.20697688152926545821e-2_f64) * t41271 + F::cast_from(0.13637330827122670865e-1_f64) * t41274 + F::cast_from(0.45457769423742236216e-1_f64) * t41277 + F::cast_from(0.1814407727691612783e-2_f64) * t41279 + F::cast_from(0.9072038638458063915e-3_f64) * t41281 - F::cast_from(0.25401708187682578962e-2_f64) * t41283 - F::cast_from(0.12700854093841289481e-2_f64) * t41285 - F::cast_from(0.25401708187682578962e-2_f64) * t41287 - F::cast_from(0.12700854093841289481e-2_f64) * t41289 + F::cast_from(0.33868944250243438616e-2_f64) * t41291;
    t43586
}
