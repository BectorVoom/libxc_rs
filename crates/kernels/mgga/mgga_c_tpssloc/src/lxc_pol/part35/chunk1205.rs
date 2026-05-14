//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1205/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1205<F: Float>(t1409: F, t2136: F, t8027: F, t29643: F, t3503: F, t86264: F, t1210: F, t29647: F, t8040: F, t95332: F, t29561: F, t6739: F, t7325: F, t27628: F, t95648: F, t104118: F, t24682: F, t460: F) -> (F, F, F, F, F, F, F) {
    let t104153 = t8027 * t1409 * t2136;
    let t104181 = t86264 * t3503 * t29643;
    let t104184 = t86264 * t1210 * t29647;
    let t104187 = t95332 * t8040;
    let t104190 = t29561 * t6739 * t7325;
    let t104231 = t95648 * t27628;
    let t104235 = t24682 * t104118 * t460;
    (t104153, t104181, t104184, t104187, t104190, t104231, t104235)
}
