//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1041/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1041<F: Float>(t31195: F, t6888: F, t794: F, t8479: F, t6897: F, t1351: F, t2006: F, t550: F, t6976: F, t1992: F, t1998: F, t6955: F, t214: F, t1985: F, t2314: F, t8326: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31197 = 0.3289868133696452873e-1 * t6888 * t31195;
    let t31198 = t794 * t8479;
    let t31200 = 0.82246703342411321825e-2 * t6897 * t31198;
    let t31201 = t2006 * t1351;
    let t31202 = t31201 * t550;
    let t31203 = t6976 * t31202;
    let t31205 = 0.16449340668482264365e-1 * t1992 * t31203;
    let t31206 = t1998 * t6955;
    let t31207 = t214 * t31206;
    let t31209 = 0.16449340668482264365e-1 * t1985 * t31207;
    let t31236 = t2314 * t8326;
    (t31197, t31198, t31200, t31202, t31203, t31205, t31206, t31207, t31209, t31236)
}
