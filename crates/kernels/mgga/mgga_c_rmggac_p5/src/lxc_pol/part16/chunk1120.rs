//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1120/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1120<F: Float>(t10252: F, t1923: F, t38029: F, t43861: F, t43862: F, t43864: F, t47510: F, t47512: F, t47516: F, t47520: F, t47524: F, t47528: F, t47530: F, t47532: F, t47534: F, t47536: F, t47538: F, t8048: F, t9128: F) -> F {
    let t49237 = -F::cast_from(0.1276937996798935182e-3_f64) * t47510 + F::cast_from(0.19863479950205658386e-4_f64) * t47512 - F::new(0.2363e1) * t1923 * t8048 - F::cast_from(0.11974241701863808564e0_f64) * t9128 * t10252 - t43861 + t43862 - F::cast_from(0.85129199786595678799e-5_f64) * t47516 - F::cast_from(0.7661627980793611092e-4_f64) * t47520 + F::cast_from(0.10215503974391481456e-3_f64) * t47524 + F::cast_from(0.2553875993597870364e-4_f64) * t47528 - F::cast_from(0.2553875993597870364e-4_f64) * t47530 + F::cast_from(0.212822999466489197e-4_f64) * t47532 + F::cast_from(0.5107751987195740728e-4_f64) * t47534 - F::cast_from(0.5107751987195740728e-4_f64) * t47536 + F::cast_from(0.8980681276397856423e-1_f64) * t47538 - t43864 + t38029;
    t49237
}
