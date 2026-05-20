//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1977/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1977<F: Float>(t2239: F, t3951: F, t193: F, t776: F, t111: F, t5363: F, t6470: F, t19297: F, t604: F, t5385: F, t19449: F, t19644: F, t225: F) -> (F, F, F, F, F, F, F, F) {
    let t46104 = t3951 * t2239;
    let t46341 = t193 * t776;
    let t55353 = t5363 * t111;
    let t55388 = t6470 * t111;
    let t55880 = t19297 * t604;
    let t55921 = t5385 * t2239;
    let t55943 = t19449 * t111;
    let t56422 = t19644 * t225;
    (t46104, t46341, t55353, t55388, t55880, t55921, t55943, t56422)
}
