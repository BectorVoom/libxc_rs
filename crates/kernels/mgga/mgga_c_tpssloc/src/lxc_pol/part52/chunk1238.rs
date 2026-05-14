//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1238/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1238<F: Float>(t27921: F, t6534: F, t24972: F, t26542: F, t26545: F, t105108: F, t7769: F, t120792: F, t120793: F, t120795: F, t120800: F, t120803: F, t120804: F, t120807: F, t7015: F, t96334: F) -> (F, F) {
    let t123282 = t27921 * t6534;
    let t123285 = t24972 * t26542;
    let t123287 = t24972 * t26545;
    let t123290 = t105108 * t7769;
    let t123292 = t120792 + 0.135e2 * t120793 + 0.135e2 * t123282 + 27.0 * t120795 + t120800 + t120803 + 27.0 * t123285 + 27.0 * t123287 + 27.0 * t120804 + t120807 + 27.0 * t123290;
    let t123294 = t96334 * t7015;
    (t123292, t123294)
}
