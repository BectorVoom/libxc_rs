//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 465/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk465<F: Float>(t2461: F, t731: F, t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F) -> (F, F) {
    let t2462 = t2461 * t731;
    let t2471 = -0.78438333333333333333e0 * t2388 + 0.15687666666666666667e1 * t2391 + 0.68863333333333333333e0 * t2394 + 0.14025833333333333333e0 * t2398 + 0.28051666666666666667e0 * t2400 + 0.17365833333333333333e0 * t2403;
    (t2462, t2471)
}
