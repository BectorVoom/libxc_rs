//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2290/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2290<F: Float>(t29809: F, t85639: F, t1251: F, t5392: F, t1751: F, t8034: F, t29822: F, t17635: F, t17691: F, t2128: F, t24589: F, t24601: F, t27382: F, t27388: F, t27433: F, t27434: F, t27444: F, t27549: F, t27774: F, t27820: F, t4936: F, t7287: F, t8002: F, t85652: F, t94297: F, t94354: F, t94363: F, t94365: F, t94395: F, t94458: F) -> (F, F) {
    let t103130 = t85639 * t29809;
    let t103132 = t5392 * t1251;
    let t103143 = t8034 * t1751;
    let t103149 = t85639 * t29822;
    let t103164 = -F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t24601 * t27444 * t17691 + F::cast_from(0.18277045187202515961e-2_f64) * t103130 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24601 * t85652 * t103132 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t94354 * t8002 + F::cast_from(0.16449340668482264365e-1_f64) * t2128 * t4936 * t27382 + F::cast_from(0.54831135561607547883e-2_f64) * t24589 * t103143 * t7287 - F::cast_from(0.14621636149762012769e-1_f64) * t94395 * t27434 + F::cast_from(0.18277045187202515961e-2_f64) * t103149 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t94458 * t27433 + F::cast_from(0.36554090374405031923e-2_f64) * t27549 * t24601 * t27774 * t17635 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t27820 * t27388 + t94363 + t94365 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t94297 * t8002;
    (t103132, t103164)
}
