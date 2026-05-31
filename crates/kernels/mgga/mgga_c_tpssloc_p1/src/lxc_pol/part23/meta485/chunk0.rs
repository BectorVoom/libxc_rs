//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1484/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1484<F: Float>(t1410: F, t1434: F, t1864: F, t19322: F, t20207: F, t20217: F, t20222: F, t20227: F, t20264: F, t20265: F, t33: F, t5398: F, t5399: F, t5400: F, t5427: F, t5442: F, t65: F, t7445: F, t75361: F, t75847: F, t79692: F, t80: F) -> F {
    let t79707 = -t5399 * t5427 * t80 / F::cast_from(2.0_f64) - t20222 * t1434 - t5400 * t5442 / F::cast_from(2.0_f64) - t1410 * t20264 * t80 / F::cast_from(3.0_f64) - t20227 * t1434 + t33 * t79692 * t80 / F::cast_from(24.0_f64) + t20265 * t1434 / F::cast_from(6.0_f64) - t75847 * t65 * t80 / F::cast_from(4.0_f64) - t75361 * t20207 - t19322 * t7445 * t5398 - t19322 * t1864 * t20217 / F::cast_from(3.0_f64);
    t79707
}
