//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1165/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1165<F: Float>(t1351: F, t1992: F, t3879: F, t550: F, t6976: F, t22704: F, t22705: F, t22741: F, t22696: F, t6914: F, t552: F, t1307: F, t6637: F, t6888: F, t22747: F, t22893: F, t80681: F) -> (F, F, F, F, F) {
    let t81122 = t1992 * t6976 * t3879 * t1351 * t550;
    let t81125 = t22704 * t22705 * t22741;
    let t81127 = t6914 * t22696;
    let t81129 = t552 * t3879;
    let t81132 = t6888 * t6637 * t81129 * t1307;
    let t81140 = t80681 * t22893 * t22747;
    (t81122, t81125, t81127, t81132, t81140)
}
