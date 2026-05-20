//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1984/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1984<F: Float>(t12566: F, t604: F, t2239: F, t3951: F, t13034: F, t225: F, t10109: F, t1527: F, t13036: F, t4119: F, t828: F, t1484: F, t2678: F) -> (F, F, F, F, F, F, F) {
    let t46099 = t12566 * t604;
    let t46104 = t3951 * t2239;
    let t46452 = t13034 * t225;
    let t46488 = t10109 * t1527;
    let t46508 = t13036 * t225;
    let t46565 = t4119 * t828;
    let t46644 = t1484 * t2678;
    (t46099, t46104, t46452, t46488, t46508, t46565, t46644)
}
