//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1361/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1361<F: Float>(t10189: F, t1926: F, t221: F, t23337: F, t10336: F, t1920: F, t1922: F, t23391: F, t6680: F, t10305: F, t1956: F, t23327: F, t23329: F, t23332: F, t23333: F, t23336: F, t23346: F, t23369: F, t23396: F, t23402: F, t23581: F, t23594: F, t23728: F, t23729: F, t25429: F, t3207: F, t43431: F, t6687: F, t6690: F, t82343: F, t82391: F, t82400: F, t82402: F, t82411: F, t82417: F, t82426: F) -> (F, F) {
    let t82431 = t1926 * t221 * t10189;
    let t82432 = t82431 * t23337;
    let t82436 = F::cast_from(0.30461741978670859935e-2_f64) * t1920 * t10336 * t1922;
    let t82437 = t6680 * t23391;
    let t82439 = F::cast_from(0.8529287754027840782e-2_f64) * t6687 * t82391 * t6690 * t10305 - F::cast_from(3.0_f64) * t23369 * t3207 - F::cast_from(0.13159472534785811492e0_f64) * t23346 * t23396 + F::cast_from(0.16449340668482264365e-1_f64) * t82400 + F::cast_from(0.43864908449286038307e-1_f64) * t82402 * t23333 + F::cast_from(0.82246703342411321826e-2_f64) * t6687 * t23581 * t23728 + F::cast_from(0.16449340668482264365e-1_f64) * t23327 * t23336 * t23402 - F::cast_from(0.10966227112321509577e-1_f64) * t25429 * t23329 * t82411 * t82343 - F::cast_from(0.16449340668482264365e-1_f64) * t23327 * t82417 * t23332 - F::cast_from(0.10966227112321509577e-1_f64) * t25429 * t23336 * t23594 - F::cast_from(0.21932454224643019154e-1_f64) * t23346 * t23729 + F::cast_from(0.27415567780803773942e-2_f64) * t82426 - F::cast_from(3.0_f64) * t43431 * t1956 - F::cast_from(0.54831135561607547883e-2_f64) * t82432 + t82436 - F::cast_from(0.43864908449286038307e-1_f64) * t82437;
    (t82431, t82439)
}
