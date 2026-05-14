//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1078/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1078<F: Float>(t10482: F, t23508: F, t42340: F, t42341: F, t43288: F, t43292: F, t10163: F, t386: F, t68: F, t3215: F, t3399: F, t3402: F, t3639: F, t11545: F, t241: F, t3241: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t43554 = t23508 * t10482;
    let t43576 = t42340 * t42341 * t43288;
    let t43577 = t23508 * t43292;
    let t43603 = 1.0 / t10163 / t386;
    let t43604 = t68 * t43603;
    let t43636 = t3215 * t3215;
    let t43637 = 1.0 / t43636;
    let t43688 = t3399 * t3399;
    let t43689 = 1.0 / t43688;
    let t43691 = t3402 * t3402;
    let t43692 = 1.0 / t43691;
    let t43705 = t3639 * t3639;
    let t43706 = 1.0 / t43705;
    let t43761 = t241 * t11545;
    let t43762 = t3241 * t3241;
    (t43554, t43576, t43577, t43604, t43637, t43689, t43692, t43706, t43761, t43762)
}
