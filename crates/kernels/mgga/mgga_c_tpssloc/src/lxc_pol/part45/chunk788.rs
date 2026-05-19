//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 788/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk788<F: Float>(t23445: F, t23486: F, t23532: F, t23569: F, t349: F, t23346: F, t23385: F, t23387: F, t23389: F, t23392: F, t23396: F, t23399: F, t23403: F, t23408: F, t23410: F, t388: F, t6687: F, t6692: F) -> (F, F) {
    let t23571 = t23445 + t23486 + t23532 + t23569;
    let t23572 = t349 * t23571;
    let t23574 = -F::cast_from(0.54831135561607547884e-2_f64) * t23385 - F::cast_from(0.54831135561607547884e-2_f64) * t23387 - F::cast_from(0.14621636149762012769e-1_f64) * t23389 + F::cast_from(0.54831135561607547884e-2_f64) * t23392 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t23396 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t23399 - F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t23403 - F::cast_from(0.14621636149762012769e-1_f64) * t23346 * t6692 + t23408 * t388 + F::new(2.0) * t23410 * t388 + t23572 * t388;
    (t23571, t23574)
}
