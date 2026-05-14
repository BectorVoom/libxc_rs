//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 947/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk947<F: Float>(t294: F, t3857: F, t11004: F, t10982: F, t3819: F, t876: F, t1429: F, t2574: F, t10989: F, t11049: F, t11002: F, t895: F, t1441: F, t2618: F, t2593: F, t2549: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11222 = t294 * t3857;
    let t11276 = 0.2283111111111111111e-1 * t11004;
    let t11277 = 0.11415555555555555555e-1 * t10982;
    let t11289 = t3819 * t876;
    let t11294 = t1429 * t2574;
    let t11309 = 0.34431666666666666666e0 * t10982;
    let t11312 = 0.13892666666666666667e0 * t10989;
    let t11319 = 0.27785333333333333334e0 * t11049;
    let t11328 = 0.22954444444444444444e0 * t11002;
    let t11351 = t3857 * t895;
    let t11356 = t1441 * t2618;
    let t11362 = t1441 * t2593;
    let t11366 = t1429 * t2549;
    (t11222, t11276, t11277, t11289, t11294, t11309, t11312, t11319, t11328, t11351, t11356, t11362, t11366)
}
