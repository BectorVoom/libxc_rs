//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 863/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk863<F: Float>(t11203: F, t1114: F, t2403: F, t3298: F, t699: F, t3301: F, t3304: F, t241: F, t3439: F, t407: F, t11135: F, t410: F, t417: F, t1097: F, t3311: F, t409: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11204 = 0.36514074074074074075e0 * t11203;
    let t11211 = t2403 * t1114;
    let t11213 = t699 * t3298;
    let t11215 = t699 * t3301;
    let t11217 = t699 * t3304;
    let t11219 = t241 * t3439;
    let t11243 = 1.0/pow_3_2(t407);
    let t11247 = 28.0 / 27.0 * t11135;
    let t11265 = 1.0 / t410 / t417 / 4.0;
    let t11274 = 1.0 / t3311 / t1097;
    let t11275 = t409 * t11274;
    (t11204, t11211, t11213, t11215, t11217, t11219, t11243, t11247, t11265, t11275)
}
