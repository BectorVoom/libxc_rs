//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 779/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk779<F: Float>(t3: F, t8110: F, t1458: F, t577: F, t7423: F, t7768: F, t7771: F, t7773: F, t2018: F, t3701: F, t590: F, t60: F, t192: F, t533: F, t1390: F, t584: F) -> (F, F, F, F, F, F, F) {
    let t8111 = t3 * t8110;
    let t8119 = 0.45e1 * t8110 * t577 + 0.135e2 * t7423 * t1458 + t7768 + t7771 + t7773;
    let t8643 = t3701 * t2018;
    let t8705 = 1.0 / t60 / t590;
    let t8944 = t192 * t533;
    let t8945 = t2018 * t1390;
    let t9211 = 0.1044e2 * t584;
    (t8111, t8119, t8643, t8705, t8944, t8945, t9211)
}
