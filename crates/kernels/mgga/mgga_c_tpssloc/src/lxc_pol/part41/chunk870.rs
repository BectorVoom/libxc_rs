//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 870/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk870<F: Float>(t154: F, t3584: F, t3241: F, t636: F, t52: F, t1094: F, t3312: F, t3311: F, t419: F, t409: F, t11135: F, t10292: F, t281: F, t415: F, t1114: F, t2403: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11145 = t154 * t3584;
    let t11147 = 1.0 / t3241 / t636;
    let t11152 = t3241 * t52;
    let t11153 = 1.0 / t11152;
    let t11185 = t1094 * t3312;
    let t11189 = 1.0 / t3311 / t419;
    let t11190 = t409 * t11189;
    let t11195 = 0.93011851851851851854e0 * t11135;
    let t11203 = t281 * t10292 * t415;
    let t11204 = 0.36514074074074074075e0 * t11203;
    let t11211 = t2403 * t1114;
    (t11145, t11147, t11153, t11185, t11190, t11195, t11203, t11204, t11211)
}
