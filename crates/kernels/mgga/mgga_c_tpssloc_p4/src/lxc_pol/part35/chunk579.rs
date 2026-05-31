//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 579/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk579<F: Float>(t33: F, t5427: F, t2291: F, t5392: F, t5398: F, t634: F, t2298: F, t638: F, t72: F, t1411: F, t1427: F, t1434: F, t5393: F, t5400: F, t5403: F, t66: F, t80: F) -> (F, F, F, F) {
    let t5428 = t33 * t5427;
    let t5433 = t2291 * t5392;
    let t5435 = t634 * t5398;
    let t5437 = t2298 * t5392;
    let t5439 = t638 * t5398;
    let t5441 = F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t5433 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t5435 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t5437 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t5439;
    let t5442 = t72 * t5441;
    let t5445 = -t5393 * t80 / F::cast_from(12.0_f64) - t5400 * t80 / F::cast_from(12.0_f64) - t5403 * t80 / F::cast_from(6.0_f64) - t1411 * t1434 / F::cast_from(6.0_f64) + t5428 * t80 / F::cast_from(24.0_f64) + t1427 * t1434 / F::cast_from(12.0_f64) + t66 * t5442 / F::cast_from(24.0_f64);
    (t5428, t5441, t5442, t5445)
}
