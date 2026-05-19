//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1355/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1355<F: Float>(t100378: F, t100390: F, t100399: F, t106355: F, t1920: F, t1948: F, t1953: F, t21481: F, t21614: F, t21617: F, t23327: F, t25470: F, t28609: F, t345: F, t353: F, t383: F, t6797: F, t6799: F, t6800: F, t82799: F, t89431: F, t89449: F) -> F {
    let t106375 = t353 * t383 * t106355 + t82799 - F::cast_from(0.16449340668482264365e-1_f64) * t23327 * t25470 * t28609 + F::cast_from(0.24674011002723396548e-1_f64) * t6797 * t6799 * t21617 * t6800 - F::cast_from(0.54831135561607547883e-2_f64) * t100378 + F::cast_from(0.82246703342411321825e-2_f64) * t1920 * t345 * t1948 * t21614 - F::cast_from(0.54831135561607547884e-2_f64) * t89431 - F::cast_from(0.82246703342411321826e-2_f64) * t100390 + F::cast_from(0.54831135561607547884e-2_f64) * t89449 + F::cast_from(0.54831135561607547883e-2_f64) * t100399 + t21481 * t1953;
    t106375
}
