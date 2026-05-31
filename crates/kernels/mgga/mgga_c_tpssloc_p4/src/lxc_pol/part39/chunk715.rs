//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 715/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk715<F: Float>(t3590: F, t466: F, t1236: F, t225: F, t1239: F, t496: F, t68: F, t1251: F, t1243: F, t3534: F) -> (F, F, F, F, F, F) {
    let t3591 = t466 * t3590;
    let t3593 = t1236 * t225;
    let t3597 = F::cast_from(1.0_f64) / t1239 / t496;
    let t3598 = t68 * t3597;
    let t3599 = t1251 * t1251;
    let t3600 = t3598 * t3599;
    let t3604 = t3534 * t1243;
    (t3591, t3593, t3598, t3599, t3600, t3604)
}
