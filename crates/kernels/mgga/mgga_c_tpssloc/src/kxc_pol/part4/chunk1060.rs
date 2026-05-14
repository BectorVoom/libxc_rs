//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1060/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1060<F: Float>(t1409: F, t65: F, t67: F, t1864: F, t3966: F, t5392: F, t628: F, t17635: F, t16558: F, t31: F, t5399: F, t1426: F, t3961: F, t3967: F, t1410: F, t3997: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19322 = t1409 * t65 * t67;
    let t19323 = t1864 * t3966;
    let t19326 = t5392 * t628;
    let t19331 = t17635 * t65;
    let t19334 = t31 * t16558;
    let t19335 = t19334 * t65;
    let t19338 = t5399 * t628;
    let t19343 = t3961 * t1426;
    let t19346 = t3967 * t1426;
    let t19349 = t1410 * t3997;
    (t19322, t19323, t19326, t19331, t19335, t19338, t19343, t19346, t19349)
}
