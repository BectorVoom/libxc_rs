//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 925/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk925<F: Float>(t10292: F, t281: F, t415: F, t1113: F, t11163: F, t136: F, t11172: F, t1114: F, t2403: F, t3298: F, t699: F, t3301: F, t3304: F, t241: F, t3439: F, t11148: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11203 = t281 * t10292 * t415;
    let t11204 = 0.36514074074074074075e0 * t11203;
    let t11205 = t1113 * t11163;
    let t11206 = t136 * t11205;
    let t11208 = t1113 * t11172;
    let t11209 = t136 * t11208;
    let t11211 = t2403 * t1114;
    let t11213 = t699 * t3298;
    let t11215 = t699 * t3301;
    let t11217 = t699 * t3304;
    let t11219 = t241 * t3439;
    let t11220 = t11219 * t11148;
    (t11203, t11204, t11205, t11206, t11208, t11209, t11211, t11213, t11215, t11217, t11219, t11220)
}
