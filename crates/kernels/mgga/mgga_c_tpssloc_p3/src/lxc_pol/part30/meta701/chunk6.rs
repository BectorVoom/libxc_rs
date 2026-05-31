//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2271/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2271<F: Float>(t23384: F, t28481: F, t14529: F, t1539: F, t17582: F, t18061: F, t1927: F, t1956: F, t23327: F, t23346: F, t23394: F, t25784: F, t28705: F, t4548: F, t4664: F, t60971: F, t61058: F, t6687: F, t6704: F, t7625: F, t82402: F, t88100: F, t88102: F, t88152: F, t88772: F) -> F {
    let t99151 = t23384 * t28481;
    let t99172 = t88100 + t88102 + F::cast_from(0.14621636149762012769e-1_f64) * t82402 * t28705 + F::cast_from(0.21932454224643019153e-1_f64) * t23346 * t28481 - F::cast_from(2.0_f64) * t61058 * t1956 - F::cast_from(0.27415567780803773942e-2_f64) * t99151 + F::cast_from(0.16449340668482264365e-1_f64) * t1927 * t4548 * t25784 + F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t88772 * t1539 * t4664 - F::cast_from(2.0_f64) * t14529 * t7625 - F::cast_from(2.0_f64) * t60971 * t1956 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t6704 * t23394 * t18061 + F::cast_from(0.3289868133696452873e-1_f64) * t6687 * t6704 * t23394 * t17582 - t88152;
    t99172
}
