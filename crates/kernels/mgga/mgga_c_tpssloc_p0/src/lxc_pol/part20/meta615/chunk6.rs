//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2222/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2222<F: Float>(t112: F, t46116: F, t1268: F, t12725: F, t12734: F, t12739: F, t12813: F, t1458: F, t19456: F, t2314: F, t2363: F, t39235: F, t4028: F, t4072: F, t45590: F, t45602: F, t45632: F, t45637: F, t45782: F, t45814: F, t5113: F, t671: F, t9348: F, t9416: F) -> (F, F) {
    let t46117 = t46116 * t112;
    let t46118 = F::cast_from(2.0_f64) * t1268 * t45782 + F::cast_from(6.0_f64) * t12725 * t2363 + F::cast_from(12.0_f64) * t12734 * t4072 + F::cast_from(6.0_f64) * t12739 * t4072 + F::cast_from(6.0_f64) * t12813 * t2314 + F::cast_from(6.0_f64) * t12813 * t5113 + F::cast_from(2.0_f64) * t1458 * t39235 + F::cast_from(6.0_f64) * t1458 * t45602 + F::cast_from(6.0_f64) * t1458 * t45637 + F::cast_from(2.0_f64) * t1458 * t45814 + F::cast_from(6.0_f64) * t19456 * t2363 + F::cast_from(2.0_f64) * t4028 * t9416 + F::cast_from(6.0_f64) * t4072 * t9348 + F::cast_from(6.0_f64) * t45632 * t671 + F::cast_from(6.0_f64) * t45590 + t46117;
    (t46117, t46118)
}
