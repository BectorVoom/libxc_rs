//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1961/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1961<F: Float>(t3215: F, t1406: F, t9238: F, t2239: F, t3951: F, t193: F, t776: F, t111: F, t5363: F, t6470: F, t19297: F, t604: F) -> (F, F, F, F, F, F, F) {
    let t43636 = t3215 * t3215;
    let t43637 = F::new(1.0) / t43636;
    let t45844 = t1406 * t9238;
    let t46104 = t3951 * t2239;
    let t46341 = t193 * t776;
    let t55353 = t5363 * t111;
    let t55388 = t6470 * t111;
    let t55880 = t19297 * t604;
    (t43637, t45844, t46104, t46341, t55353, t55388, t55880)
}
