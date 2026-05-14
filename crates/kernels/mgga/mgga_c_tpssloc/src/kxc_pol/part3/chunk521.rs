//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 521/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk521<F: Float>(t676: F, t739: F, t172: F, t2368: F, t2369: F, t746: F, t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F, t738: F) -> (F, F, F, F, F, F, F) {
    let t2490 = t676 * t739;
    let t2494 = t172 * t2368;
    let t2495 = t2369 * t746;
    let t2504 = -0.57538888888888888889e0 * t2388 + 0.11507777777777777778e1 * t2391 + 0.40256666666666666667e0 * t2394 + 0.366775e-1 * t2398 + 0.73355e-1 * t2400 + 0.137975e0 * t2403;
    let t2505 = t2504 * t746;
    let t2508 = t738 * t738;
    let t2509 = 1.0 / t2508;
    (t2490, t2494, t2495, t2504, t2505, t2508, t2509)
}
