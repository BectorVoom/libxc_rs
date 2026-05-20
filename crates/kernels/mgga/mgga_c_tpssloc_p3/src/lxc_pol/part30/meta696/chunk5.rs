//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2232/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2232<F: Float>(t1902: F, t5584: F, t1888: F, t232: F, t58226: F, t6646: F, t23110: F, t23185: F, t28418: F, t59331: F, t23168: F, t28330: F) -> (F, F, F, F, F) {
    let t98541 = t1902 * t5584;
    let t98546 = t1888 * t6646 * t58226 * t232;
    let t98549 = t23185 * t23110 * t28418;
    let t98553 = t1888 * t6646 * t59331 * t232;
    let t98564 = t23168 * t28330;
    (t98541, t98546, t98549, t98553, t98564)
}
