//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 937/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk937<F: Float>(t10989: F, t11049: F, t11002: F, t1411: F, t2480: F, t294: F, t3857: F, t11004: F, t10982: F, t3819: F, t876: F, t1429: F, t2574: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11172 = F::new(0.11038e0) * t10989;
    let t11179 = F::new(0.22076e0) * t11049;
    let t11188 = F::new(0.13418888888888888889e0) * t11002;
    let t11216 = t1411 * t2480;
    let t11222 = t294 * t3857;
    let t11276 = F::new(0.2283111111111111111e-1) * t11004;
    let t11277 = F::new(0.11415555555555555555e-1) * t10982;
    let t11289 = t3819 * t876;
    let t11294 = t1429 * t2574;
    (t11172, t11179, t11188, t11216, t11222, t11276, t11277, t11289, t11294)
}
