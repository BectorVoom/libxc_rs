//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1203/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1203<F: Float>(t10165: F, t1052: F, t105840: F, t1409: F, t17575: F, t21118: F, t21510: F, t21692: F, t23327: F, t23329: F, t23330: F, t23593: F, t25406: F, t25423: F, t25429: F, t25430: F, t25442: F, t28480: F, t28499: F, t28713: F, t4660: F, t5919: F, t5943: F, t6687: F, t6690: F, t6771: F, t7624: F, t7625: F, t82411: F, t88076: F) -> (F,) {
    let t105934 = -3.0 * t17575 * t7625 - 18.0 * t1052 * t10165 * t7624 * t5919 - 0.16449340668482264365e-1 * t23327 * t23329 * t25423 * t21510 + 0.16449340668482264365e-1 * t23327 * t25442 * t28499 + 0.16449340668482264365e-1 * t23327 * t23329 * t88076 * t1409 * t5919 - 0.10966227112321509577e-1 * t25429 * t23329 * t82411 * t105840 + 0.10966227112321509577e-1 * t25429 * t23329 * t25430 * t21510 - 0.24674011002723396548e-1 * t6687 * t25406 * t28480 + 6.0 * t4660 * t28713 + 6.0 * t6771 * t21692 - 0.21932454224643019154e-1 * t6687 * t23593 * t6690 * t21118 - 0.82246703342411321826e-2 * t23327 * t23329 * t23330 * t1409 * t5943;
    (t105934,)
}
