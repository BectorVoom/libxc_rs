//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1418/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1418<F: Float>(t136: F, t3297: F, t78031: F, t78039: F, t1113: F, t78047: F, t78043: F, t1100: F, t78077: F, t3287: F, t78025: F, t11219: F, t78035: F) -> (F, F, F, F, F, F, F) {
    let t78084 = t136 * t3297 * t78031;
    let t78087 = t136 * t3297 * t78039;
    let t78090 = t136 * t1113 * t78047;
    let t78093 = t136 * t1113 * t78043;
    let t78095 = t1100 * t78077;
    let t78097 = t3287 * t78025;
    let t78100 = t136 * t11219 * t78035;
    (t78084, t78087, t78090, t78093, t78095, t78097, t78100)
}
