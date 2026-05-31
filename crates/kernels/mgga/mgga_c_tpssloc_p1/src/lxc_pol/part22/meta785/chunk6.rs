//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2709/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2709<F: Float>(t5: F, t12568: F, t12571: F, t1437: F, t19299: F, t19310: F, t19313: F, t19318: F, t39043: F, t3958: F, t4021: F, t45844: F, t46085: F, t46086: F, t46087: F, t46088: F, t46089: F, t46090: F, t46104: F, t5389: F, t5445: F, t55880: F, t55921: F, t645: F, t75284: F, t75552: F, t86: F) -> F {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t75554 = piecewise3::<F>(t8, F::cast_from(0.0_f64), (-t46085 - t46086 - t46087 - t46088 + t46089 + t46090 + t39043) * t86 - F::cast_from(4.0_f64) * t75284 * t645 - F::cast_from(12.0_f64) * t55880 * t1437 + F::cast_from(60.0_f64) * t55921 * t3958 - F::cast_from(12.0_f64) * t19299 * t4021 + F::cast_from(60.0_f64) * t46104 * t5389 - F::cast_from(360.0_f64) * t45844 * t19310 + F::cast_from(120.0_f64) * t12571 * t19313 - F::cast_from(12.0_f64) * t12568 * t5445 + F::cast_from(60.0_f64) * t12571 * t19318 + t75552);
    t75554
}
