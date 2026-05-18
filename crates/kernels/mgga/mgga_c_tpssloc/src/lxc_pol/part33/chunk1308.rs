//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1308/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1308<F: Float>(t28827: F, t7685: F, t23035: F, t25224: F, t28298: F, t20756: F, t6553: F, t6554: F, t81984: F, t1527: F, t22986: F, t23270: F, t98253: F) -> (F, F, F, F) {
    let t105213 = F::new(18.0) * t7685 * t28827;
    let t105223 = t23035 * t25224 * t28298;
    let t105232 = t81984 * t6553 * t6554 * t20756;
    let t105240 = t22986 * t23270 * t98253 * t1527;
    (t105213, t105223, t105232, t105240)
}
