//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 522/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk522<F: Float>(t664: F, t7778: F, t739: F, t2046: F, t2165: F, t3047: F, t2169: F, t3056: F, t3057: F, t14161: F, t1968: F, t1966: F) -> (F, F, F, F, F, F, F, F) {
    let t14207 = t7778 * t664;
    let t14208 = t739 * t14207;
    let t14211 = t2046 * t3047 * t2165;
    let t14214 = t2046 * t3047 * t2169;
    let t14217 = t3056 * t3057 * t2165;
    let t14220 = t3056 * t3057 * t2169;
    let t14224 = t14161 * t1968;
    let t14225 = t1966 * t14224;
    (t14207, t14208, t14211, t14214, t14217, t14220, t14224, t14225)
}
