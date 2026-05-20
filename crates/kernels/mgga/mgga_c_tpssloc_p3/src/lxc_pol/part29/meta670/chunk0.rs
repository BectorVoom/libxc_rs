//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2240/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2240<F: Float>(t1354: F, t91278: F, t1827: F, t80991: F, t22765: F, t5289: F, t22764: F, t5234: F, t26298: F, t80958: F, t1307: F, t1339: F, t22827: F, t5287: F) -> (F, F, F, F, F, F) {
    let t91279 = t91278 * t1354;
    let t91281 = t80991 * t1827;
    let t91282 = F::new(7.0) / F::new(1152.0) * t91281;
    let t91283 = t22765 * t5289;
    let t91284 = F::new(7.0) / F::new(1152.0) * t91283;
    let t91285 = t5234 * t22764;
    let t91286 = t91285 * t1354;
    let t91287 = F::new(7.0) / F::new(1152.0) * t91286;
    let t91290 = t80958 * t26298;
    let t91294 = t22827 * t1339 * t5287 * t1307;
    (t91279, t91282, t91284, t91287, t91290, t91294)
}
