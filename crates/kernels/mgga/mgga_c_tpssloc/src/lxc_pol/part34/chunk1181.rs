//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1181/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1181<F: Float>(t20800: F, t25: F, t20947: F, t25014: F, t1408: F, t5527: F, t5664: F, t5660: F, t20778: F, t105769: F, t25927: F, t105754: F, t23788: F) -> (F, F, F, F, F, F, F, F) {
    let t105797 = t25 * t20800;
    let t105801 = t25014 * t20947;
    let t105810 = t1408 * t5527;
    let t105814 = t1408 * t5664;
    let t105818 = t1408 * t5660;
    let t105822 = t25 * t20778;
    let t106618 = t25927 * t105769;
    let t106621 = t23788 * t105754;
    (t105797, t105801, t105810, t105814, t105818, t105822, t106618, t106621)
}
