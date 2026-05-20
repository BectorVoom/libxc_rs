//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2172/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2172<F: Float>(t97233: F, t97268: F, t97309: F, t97349: F, t97376: F, t97392: F, t97433: F, t97465: F, t19661: F, t1992: F, t22897: F, t19736: F) -> (F, F, F) {
    let t97468 = t97233 + t97268 + t97309 + t97349 + t97376 + t97392 + t97433 + t97465;
    let t97488 = t1992 * t22897 * t19661;
    let t97491 = t1992 * t22897 * t19736;
    (t97468, t97488, t97491)
}
