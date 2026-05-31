//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2280/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2280<F: Float>(t11925: F, t14972: F, t24563: F, t24589: F, t24604: F, t24884: F, t27382: F, t27406: F, t27433: F, t27751: F, t3471: F, t7283: F, t7356: F, t8002: F, t8061: F, t85701: F, t85728: F, t86415: F, t94436: F, t94439: F, t94446: F, t94451: F, t94456: F, t94458: F) -> F {
    let t94464 = F::cast_from(0.36554090374405031922e-2_f64) * t85701 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27751 * t24563 + F::cast_from(4.0_f64) * t14972 * t7356 - F::cast_from(0.18277045187202515961e-2_f64) * t94436 - t94439 + F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t3471 * t27382 + F::cast_from(0.73108180748810063843e-2_f64) * t27406 * t24884 - t94446 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t86415 * t27433 + t94451 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t85728 * t8002 - t94456 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t94458 * t24604 + F::cast_from(2.0_f64) * t11925 * t8061;
    t94464
}
