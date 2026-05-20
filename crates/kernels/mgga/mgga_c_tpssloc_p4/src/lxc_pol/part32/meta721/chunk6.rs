//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2296/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2296<F: Float>(t24574: F, t29554: F, t1240: F, t6267: F, t2122: F, t29817: F, t1184: F, t6145: F, t1186: F, t1409: F, t1761: F, t19234: F, t19249: F, t24589: F, t24601: F, t24602: F, t27406: F, t27416: F, t27437: F, t27799: F, t27800: F, t29690: F, t29808: F, t5088: F, t7283: F, t7356: F, t85807: F, t86415: F, t94458: F, t94535: F, t95836: F) -> (F, F, F) {
    let t103304 = t24574 * t29554;
    let t103314 = t1240 * t6267;
    let t103315 = t2122 * t103314;
    let t103332 = t24574 * t29817;
    let t103337 = t6145 * t1184;
    let t103341 = -F::cast_from(0.54831135561607547883e-2_f64) * t103304 + t94535 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t27416 - F::new(2.0) * t95836 * t1761 + F::new(2.0) * t19249 * t7356 + F::new(4.0) * t19234 * t7356 + F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t1186 * t103315 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t27800 + F::cast_from(0.54831135561607547883e-2_f64) * t24589 * t94458 * t27437 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t86415 * t29808 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24601 * t24602 * t1409 * t5088 - F::cast_from(0.18277045187202515961e-2_f64) * t103332 + F::cast_from(0.36554090374405031923e-2_f64) * t7283 * t85807 * t29690 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t103337 * t27799;
    (t103314, t103337, t103341)
}
