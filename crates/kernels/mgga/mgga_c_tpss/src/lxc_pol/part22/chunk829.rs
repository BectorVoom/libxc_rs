//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 829/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk829<F: Float>(t1270: F, t1844: F, t64: F, t789: F, t112: F, t2023: F, t641: F, t2075: F, t600: F, t2101: F, t629: F, t98: F, t99: F, t2: F, t22: F, t106: F, t107: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7383 = t1844 * t1270;
    let t7585 = t64 * t789;
    let t7587 = 154.0 / 27.0 * t7585 * t112;
    let t7588 = t2023 * t641;
    let t7590 = t600 * t2075;
    let t7592 = t600 * t2101;
    let t7593 = t629 * t629;
    let t7594 = 1.0 / t7593;
    let t7612 = t99 * t98;
    let t7613 = 1.0 / t7612;
    let t7622 = t2 * t22;
    let t7628 = t107 * t106;
    (t7383, t7585, t7587, t7588, t7590, t7592, t7594, t7613, t7622, t7628)
}
