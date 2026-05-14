//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1070/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1070<F: Float>(t1998: F, t6955: F, t214: F, t1985: F, t1338: F, t8470: F, t1352: F, t31181: F, t553: F, t1332: F, t1336: F, t31192: F, t31197: F, t31200: F, t31205: F, t544: F, t8483: F) -> (F, F, F, F, F, F) {
    let t31206 = t1998 * t6955;
    let t31207 = t214 * t31206;
    let t31209 = 0.16449340668482264365e-1 * t1985 * t31207;
    let t31211 = t1338 * t8470;
    let t31212 = t31211 * t1352;
    let t31214 = t553 * t31181;
    let t31216 = t1332 * t8483 - t1336 * t31212 + t31214 * t544 - t31192 - t31197 - t31200 - t31205 + t31209;
    (t31206, t31207, t31211, t31212, t31214, t31216)
}
