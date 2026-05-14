//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 946/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk946<F: Float>(t11004: F, t10982: F, t3800: F, t673: F, t3797: F, t11002: F, t10989: F, t1411: F, t2480: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11005 = 4.0 / 9.0 * t11004;
    let t11006 = 2.0 / 9.0 * t10982;
    let t11049 = t673 * t3800;
    let t11050 = 0.21908444444444444444e0 * t11049;
    let t11051 = t673 * t3797;
    let t11071 = 0.39862222222222222222e0 * t11004;
    let t11109 = 0.41203703703703703704e-2 * t11002;
    let t11110 = 0.12361111111111111111e-1 * t11004;
    let t11111 = 0.61805555555555555556e-2 * t10982;
    let t11134 = 0.23744444444444444444e-1 * t11004;
    let t11135 = 0.11872222222222222222e-1 * t10982;
    let t11169 = 0.20128333333333333334e0 * t10982;
    let t11172 = 0.11038e0 * t10989;
    let t11179 = 0.22076e0 * t11049;
    let t11188 = 0.13418888888888888889e0 * t11002;
    let t11216 = t1411 * t2480;
    (t11005, t11006, t11049, t11050, t11051, t11071, t11109, t11110, t11111, t11134, t11135, t11169, t11172, t11179, t11188, t11216)
}
