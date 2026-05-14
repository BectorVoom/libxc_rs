//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 780/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk780<F: Float>(t8444: F, t8446: F, t8684: F, t8860: F, t113: F, t2114: F, t2165: F, t510: F, t574: F, t8322: F, t8329: F, t8491: F, t8495: F, t8669: F, t8676: F, t8691: F, t8913: F) -> (F, F) {
    let t8916 = t8860 + 4.0 * t8684 + t8444 + t8446;
    let t8919 = -t113 * t8913 - 2.0 * t2114 * t2165 - t510 * t8860 + t574 * t8916 - t8322 - t8329 + t8491 - t8495 - 4.0 * t8669 - 4.0 * t8676 + 2.0 * t8691;
    (t8916, t8919)
}
