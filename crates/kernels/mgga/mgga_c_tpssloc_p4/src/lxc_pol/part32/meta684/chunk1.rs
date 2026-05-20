//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2126/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2126<F: Float>(t5445: F, t641: F, t72: F, t19445: F, t79: F, t19299: F, t608: F, t3966: F, t2235: F, t5399: F, t17635: F, t605: F) -> (F, F, F, F, F, F) {
    let t96517 = t72 * t641 * t5445;
    let t96521 = t72 * t79 * t19445;
    let t96535 = t19299 * t608;
    let t96553 = t72 * t79 * t3966;
    let t96556 = t2235 * t5399;
    let t96559 = t605 * t17635;
    (t96517, t96521, t96535, t96553, t96556, t96559)
}
