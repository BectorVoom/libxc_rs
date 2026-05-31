//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2654/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2654<F: Float>(t12633: F, t12636: F, t12648: F, t1426: F, t1434: F, t19331: F, t19334: F, t19335: F, t19338: F, t2252: F, t2255: F, t2283: F, t2304: F, t31: F, t3976: F, t4018: F, t5399: F, t5400: F, t5428: F, t5442: F, t55677: F, t628: F, t642: F, t65: F, t80: F) -> F {
    let t55709 = -t19331 * t642 / F::cast_from(6.0_f64) - t31 * t55677 * t65 * t80 / F::cast_from(12.0_f64) - t19334 * t628 * t80 / F::cast_from(6.0_f64) - t19335 * t642 / F::cast_from(6.0_f64) - t5399 * t2283 * t80 / F::cast_from(12.0_f64) - t19338 * t642 / F::cast_from(6.0_f64) - t5400 * t2304 / F::cast_from(12.0_f64) - t12648 * t1426 * t80 / F::cast_from(6.0_f64) - t12633 * t1434 / F::cast_from(6.0_f64) - t12636 * t1434 / F::cast_from(3.0_f64) - t3976 * t4018 / F::cast_from(3.0_f64) - t2252 * t5442 / F::cast_from(12.0_f64) - t2255 * t5442 / F::cast_from(6.0_f64) + t5428 * t2304 / F::cast_from(24.0_f64);
    t55709
}
