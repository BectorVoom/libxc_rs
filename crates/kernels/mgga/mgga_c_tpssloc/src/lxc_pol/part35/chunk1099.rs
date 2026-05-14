//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1099/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1099<F: Float>(t214: F, t28333: F, t1880: F, t1510: F, t25249: F, t6646: F, t22986: F, t5527: F, t6638: F, t6637: F, t23035: F, t1484: F, t25319: F, t6552: F, t25255: F, t1499: F, t23014: F, t23032: F, t25246: F, t25259: F, t28323: F, t28331: F, t4166: F, t7533: F, t7535: F, t812: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28334 = t214 * t28333;
    let t28335 = t1880 * t28334;
    let t28337 = t25249 * t1510;
    let t28338 = t6646 * t28337;
    let t28339 = t22986 * t28338;
    let t28341 = t6638 * t5527;
    let t28342 = t6637 * t28341;
    let t28343 = t23035 * t28342;
    let t28345 = t25319 * t1484;
    let t28346 = t6637 * t28345;
    let t28347 = t6552 * t28346;
    let t28351 = t25255 * t1510;
    let t28354 = -0.82246703342411321825e-2 * t28323 + 0.82246703342411321824e-2 * t25246 + 2.0 * t1499 * t7535 - 0.82246703342411321824e-2 * t25259 - 0.16449340668482264365e-1 * t28331 + t23014 + t23032 + 0.82246703342411321825e-2 * t28335 + 0.3289868133696452873e-1 * t28339 + 0.49348022005446793095e-1 * t28343 - 0.3289868133696452873e-1 * t28347 - 2.0 * t4166 * t7533 - 2.0 * t812 * t28351;
    (t28334, t28337, t28338, t28341, t28342, t28345, t28346, t28351, t28354)
}
