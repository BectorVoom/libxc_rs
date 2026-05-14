//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1284/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1284<F: Float>(t22480: F, t2314: F, t22947: F, t532: F, t1983: F, t6879: F, t111: F, t22558: F, t1874: F, t39235: F, t1873: F, t45637: F, t12734: F, t6534: F, t45602: F, t9348: F) -> (F, F, F, F, F, F, F, F, F) {
    let t83928 = 6.0 * t2314 * t22480;
    let t83929 = t532 * t22947;
    let t83932 = 9.0 * t1983 * t83929 * t6879;
    let t83935 = t22558 * t111;
    let t83939 = 2.0 * t39235 * t1874;
    let t83946 = 6.0 * t45637 * t1873;
    let t83948 = 12.0 * t12734 * t6534;
    let t83952 = 2.0 * t39235 * t1873;
    let t83956 = 6.0 * t45602 * t1873;
    let t83958 = 6.0 * t9348 * t6534;
    (t83928, t83932, t83935, t83939, t83946, t83948, t83952, t83956, t83958)
}
