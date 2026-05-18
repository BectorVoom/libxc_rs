//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1309/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1309<F: Float>(t1880: F, t21033: F, t6553: F, t6571: F, t21049: F, t82252: F, t1484: F, t22986: F, t23270: F, t98161: F, t7488: F, t98133: F) -> (F, F, F, F) {
    let t105250 = t1880 * t6553 * t6571 * t21033;
    let t105254 = t1880 * t6553 * t82252 * t21049;
    let t105258 = t22986 * t23270 * t98161 * t1484;
    let t105267 = t1880 * t98133 * t7488;
    (t105250, t105254, t105258, t105267)
}
