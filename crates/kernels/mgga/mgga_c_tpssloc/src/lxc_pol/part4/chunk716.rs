//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 716/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk716<F: Float>(t33: F, t5427: F, t2291: F, t5392: F, t5398: F, t634: F, t2298: F, t638: F, t72: F, t1411: F, t1427: F, t1434: F, t5393: F, t5400: F, t5403: F, t66: F, t80: F) -> (F, F, F) {
    let t5428 = t33 * t5427;
    let t5433 = t2291 * t5392;
    let t5435 = t634 * t5398;
    let t5437 = t2298 * t5392;
    let t5439 = t638 * t5398;
    let t5441 = 28.0 / 9.0 * t5433 - 4.0 / 3.0 * t5435 + 28.0 / 9.0 * t5437 + 4.0 / 3.0 * t5439;
    let t5442 = t72 * t5441;
    let t5445 = -t5393 * t80 / 12.0 - t5400 * t80 / 12.0 - t5403 * t80 / 6.0 - t1411 * t1434 / 6.0 + t5428 * t80 / 24.0 + t1427 * t1434 / 12.0 + t66 * t5442 / 24.0;
    (t5428, t5442, t5445)
}
