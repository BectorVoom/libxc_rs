//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 936/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk936<F: Float>(t11004: F, t10982: F, t3800: F, t673: F, t3797: F, t11002: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11005 = F::new(4.0) / F::new(9.0) * t11004;
    let t11006 = F::new(2.0) / F::new(9.0) * t10982;
    let t11049 = t673 * t3800;
    let t11050 = F::new(0.21908444444444444444e0) * t11049;
    let t11051 = t673 * t3797;
    let t11071 = F::new(0.39862222222222222222e0) * t11004;
    let t11109 = F::new(0.41203703703703703704e-2) * t11002;
    let t11110 = F::new(0.12361111111111111111e-1) * t11004;
    let t11111 = F::new(0.61805555555555555556e-2) * t10982;
    let t11134 = F::new(0.23744444444444444444e-1) * t11004;
    let t11135 = F::new(0.11872222222222222222e-1) * t10982;
    let t11169 = F::new(0.20128333333333333334e0) * t10982;
    (t11005, t11006, t11049, t11050, t11051, t11071, t11109, t11110, t11111, t11134, t11135, t11169)
}
