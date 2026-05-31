//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2340/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2340<F: Float>(t46104: F, t7245: F, t12571: F, t24525: F, t27331: F, t9239: F, t2110: F, t22527: F, t22531: F, t22537: F, t22546: F, t24514: F, t26055: F, t27341: F, t6492: F, t7256: F, t7259: F, t7432: F, t7978: F, t85510: F, t90196: F, t90202: F, t90205: F) -> F {
    let t96025 = t46104 * t7245;
    let t96028 = t12571 * t24525;
    let t96045 = t9239 * t27331;
    let t96050 = t22537 * t7978 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t96025 * t6492 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t96028 * t6492 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t27341 * t22527 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t27341 * t22531 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t90202 * t2110 + t90205 * t2110 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26055 * t7256 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26055 * t7259 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t85510 * t7432 - F::cast_from(5.0_f64) * t96045 * t22546 - F::cast_from(5.0_f64) * t24514 * t90196;
    t96050
}
