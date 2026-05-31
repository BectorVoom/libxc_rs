//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 483/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk483<F: Float>(t676: F, t739: F, t172: F, t2368: F, t2369: F, t746: F, t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F) -> (F, F, F, F) {
    let t2490 = t676 * t739;
    let t2494 = t172 * t2368;
    let t2495 = t2369 * t746;
    let t2504 = -F::cast_from(0.57538888888888888889e0_f64) * t2388 + F::cast_from(0.11507777777777777778e1_f64) * t2391 + F::cast_from(0.40256666666666666667e0_f64) * t2394 + F::cast_from(0.366775e-1_f64) * t2398 + F::cast_from(0.73355e-1_f64) * t2400 + F::cast_from(0.137975e0_f64) * t2403;
    (t2490, t2494, t2495, t2504)
}
