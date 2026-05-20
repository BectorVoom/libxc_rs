//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2002/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2002<F: Float>(t13036: F, t225: F, t4119: F, t828: F, t1484: F, t2678: F, t1509: F, t2631: F, t9975: F, t2710: F, t4233: F, t852: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46508 = t13036 * t225;
    let t46565 = t4119 * t828;
    let t46644 = t1484 * t2678;
    let t46693 = t1509 * t2678;
    let t47012 = t1484 * t2631;
    let t47262 = t1509 * t2631;
    let t47285 = t1509 * t9975;
    let t47425 = t2710 * t1509;
    let t47439 = t852 * t4233;
    (t46508, t46565, t46644, t46693, t47012, t47262, t47285, t47425, t47439)
}
