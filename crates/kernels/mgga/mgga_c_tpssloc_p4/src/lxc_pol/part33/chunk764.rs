//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 764/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk764<F: Float>(t6689: F, t7553: F, t1599: F, t1922: F, t1625: F, t225: F, t387: F) -> (F, F, F, F) {
    let t7554 = t6689 * t7553;
    let t7557 = t1599 * t1922;
    let t7560 = t1625 * t225;
    let t7561 = t7560 * t387;
    (t7554, t7557, t7560, t7561)
}
