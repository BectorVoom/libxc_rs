//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 427/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk427<F: Float>(t158: F, t4272: F, t1079: F, t166: F, t4221: F, t4130: F, t4133: F, t4136: F, t4138: F, t4142: F, t4144: F, t4146: F, t4149: F) -> (F, F, F) {
    let t4273 = t158 * t4272;
    let t4275 = F::cast_from(1.0_f64) / t1079 / t166;
    let t4276 = t4221 * t4275;
    let t4287 = -F::cast_from(0.25319e1_f64) * t4130 + F::cast_from(0.16879333333333333333e1_f64) * t4133 - F::cast_from(0.19692555555555555555e1_f64) * t4136 - F::cast_from(0.93011851851851851854e0_f64) * t4138 + F::cast_from(0.13651666666666666667e0_f64) * t4142 - F::cast_from(0.27303333333333333333e0_f64) * t4144 - F::cast_from(0.3185388888888888889e0_f64) * t4146 - F::cast_from(0.36514074074074074075e0_f64) * t4149;
    (t4273, t4276, t4287)
}
