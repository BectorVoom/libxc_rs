//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1247/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1247<F: Float>(t28002: F, t7467: F, t1873: F, t67001: F, t28017: F, t4028: F, t20347: F, t88: F, t28007: F, t7676: F, t20304: F, t81446: F, t22473: F, t75603: F, t20342: F, t6530: F) -> (F, F, F, F, F, F, F, F, F) {
    let t106923 = 12.0 * t28002 * t7467;
    let t106932 = 2.0 * t67001 * t1873;
    let t106934 = 6.0 * t4028 * t28017;
    let t106935 = t88 * t20347;
    let t106937 = 2.0 * t106935 * t1873;
    let t106939 = 6.0 * t28007 * t7467;
    let t106941 = 6.0 * t7676 * t28017;
    let t106944 = t81446 * t20304;
    let t106946 = t22473 * t75603;
    let t106948 = t6530 * t20342;
    (t106923, t106932, t106934, t106937, t106939, t106941, t106944, t106946, t106948)
}
