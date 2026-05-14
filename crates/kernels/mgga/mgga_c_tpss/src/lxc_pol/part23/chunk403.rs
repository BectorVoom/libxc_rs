//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 403/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk403<F: Float>(t1290: F, t70: F, t1289: F, t48: F, t51: F, t53: F, rho1: F) -> (F, F, F) {
    let t1291 = t1290 * t70;
    let t1294 = t48 * t1289;
    let t1297 = t51 * rho1;
    let t1299 = 1.0 / t53 / t1297;
    (t1291, t1294, t1299)
}
