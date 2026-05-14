//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1259/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1259<F: Float>(t6013: F, t9668: F, t6005: F, t8550: F, t9605: F, t6007: F, t9542: F, t6002: F, t9657: F, t19082: F, t8507: F, t3138: F, t3124: F, t1877: F, t9699: F, t1883: F, t24476: F) -> (F, F, F, F, F, F, F, F) {
    let t63273 = t6013 * t9668;
    let t63282 = t8550 * t6005 * t9605;
    let t63285 = t6007 * t9542;
    let t63292 = t6002 * t9657;
    let t63308 = t19082 * t8507;
    let t63309 = t3138 * t63308;
    let t63314 = t3124 * t63308;
    let t63318 = t1877 * t9699 / 5184.0;
    let t63357 = t1883 * t24476;
    (t63273, t63282, t63285, t63292, t63309, t63314, t63318, t63357)
}
