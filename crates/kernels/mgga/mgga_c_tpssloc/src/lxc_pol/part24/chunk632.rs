//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 632/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk632<F: Float>(t3403: F, t3422: F, t1164: F, t457: F, t697: F, t461: F, t221: F, t456: F, t1176: F, t135: F, t1179: F, t1174: F) -> (F, F, F, F, F, F) {
    let t3423 = t3422 * t3403;
    let t3425 = F::cast_from(0.17315859105681463759e2_f64) * t1164 * t3423;
    let t3426 = t697 * t457;
    let t3427 = t3426 * t461;
    let t3428 = t221 * t3427;
    let t3430 = F::cast_from(0.18518518518518518518e-3_f64) * t456 * t3428;
    let t3431 = t135 * t1176;
    let t3432 = t3431 * t1179;
    let t3433 = t1174 * t3432;
    (t3423, t3425, t3426, t3430, t3431, t3433)
}
