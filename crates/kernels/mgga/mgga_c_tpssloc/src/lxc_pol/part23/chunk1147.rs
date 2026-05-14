//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1147/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1147<F: Float>(t14511: F, t17655: F, t10883: F, t21403: F, t248: F, t3101: F, t1041: F, t21130: F, t42592: F, t21594: F, t376: F, t10422: F, t21519: F, t3070: F, t135: F, t21561: F, t973: F) -> (F, F, F, F, F, F) {
    let t70351 = t14511 * t17655;
    let t70363 = t10883 * t248 * t3101 * t21403;
    let t70389 = t1041 * t248 * t42592 * t21130;
    let t70391 = t376 * t21594;
    let t70404 = t3070 * t10422 * t21519;
    let t70497 = t973 * t135 * t21561;
    (t70351, t70363, t70389, t70391, t70404, t70497)
}
