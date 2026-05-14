//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1230/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1230<F: Float>(t10336: F, t1920: F, t1922: F, t23391: F, t6680: F, t10305: F, t1956: F, t23327: F, t23329: F, t23332: F, t23333: F, t23336: F, t23346: F, t23369: F, t23396: F, t23402: F, t23581: F, t23594: F, t23728: F, t23729: F, t25429: F, t3207: F, t43431: F, t6687: F, t6690: F, t82343: F, t82391: F, t82400: F, t82402: F, t82411: F, t82417: F, t82426: F, t82432: F) -> (F,) {
    let t82436 = 0.30461741978670859935e-2 * t1920 * t10336 * t1922;
    let t82437 = t6680 * t23391;
    let t82439 = 0.8529287754027840782e-2 * t6687 * t82391 * t6690 * t10305 - 3.0 * t23369 * t3207 - 0.13159472534785811492e0 * t23346 * t23396 + 0.16449340668482264365e-1 * t82400 + 0.43864908449286038307e-1 * t82402 * t23333 + 0.82246703342411321826e-2 * t6687 * t23581 * t23728 + 0.16449340668482264365e-1 * t23327 * t23336 * t23402 - 0.10966227112321509577e-1 * t25429 * t23329 * t82411 * t82343 - 0.16449340668482264365e-1 * t23327 * t82417 * t23332 - 0.10966227112321509577e-1 * t25429 * t23336 * t23594 - 0.21932454224643019154e-1 * t23346 * t23729 + 0.27415567780803773942e-2 * t82426 - 3.0 * t43431 * t1956 - 0.54831135561607547883e-2 * t82432 + t82436 - 0.43864908449286038307e-1 * t82437;
    (t82439,)
}
