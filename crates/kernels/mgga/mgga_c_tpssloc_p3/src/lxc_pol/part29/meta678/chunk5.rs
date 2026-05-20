//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2275/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2275<F: Float>(t1751: F, t7319: F, t1240: F, t5088: F, t1089: F, t3597: F, t1090: F, t12648: F, t1409: F, t14165: F, t24589: F, t24601: F, t24883: F, t24887: F, t27381: F, t27444: F, t27445: F, t27549: F, t27774: F, t27775: F, t27820: F, t3248: F, t3252: F, t3599: F, t7287: F, t8002: F, t85640: F, t85648: F, t86415: F) -> (F, F) {
    let t94297 = t7319 * t1751;
    let t94319 = t1240 * t5088;
    let t94332 = t3597 * t1089;
    let t94341 = F::cast_from(0.73108180748810063846e-2_f64) * t27549 * t86415 * t27775 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t94297 * t7287 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t27820 * t24883 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t27820 * t24887 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t24601 * t27381 * t3252 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24601 * t27381 * t3248 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t85648 * t8002 + F::cast_from(0.18277045187202515961e-2_f64) * t85640 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24601 * t94319 * t1090 - F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24601 * t27444 * t12648 - F::cast_from(0.16449340668482264365e-1_f64) * t24589 * t24601 * t27774 * t14165 - F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24601 * t94332 * t1409 * t3599 - F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t86415 * t27445;
    (t94319, t94341)
}
