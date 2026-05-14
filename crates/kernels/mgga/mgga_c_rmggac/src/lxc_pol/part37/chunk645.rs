//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 645/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk645<F: Float>(t14019: F, t14027: F, t70554: F, t14267: F, t2165: F, t3056: F, t2169: F, t2046: F, t2049: F, t1322: F, t235: F, t36632: F, t20: F, t1311: F, t1325: F, t3054: F, t641: F, t70383: F) -> (F, F, F, F, F, F) {
    let t70556 = t14019 * t70554 * t14027;
    let t70573 = t3056 * t14267 * t2165;
    let t70574 = 0.17347588262831798124e-4 * t70573;
    let t70577 = t3056 * t14267 * t2169;
    let t70578 = 0.17347588262831798124e-4 * t70577;
    let t70582 = t2046 * t2049 * t2165;
    let t70585 = t235 * t36632 * t1322;
    let t70604 = t20 * t20;
    let t70610 = t1311 * t70604 * t3054 * t1322 * t1325 * t70383 * t641;
    (t70556, t70574, t70578, t70582, t70585, t70610)
}
