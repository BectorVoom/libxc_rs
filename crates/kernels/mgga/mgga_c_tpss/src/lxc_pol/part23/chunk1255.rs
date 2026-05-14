//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1255/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1255<F: Float>(t19082: F, t8507: F, t3138: F, t3124: F, t1877: F, t9699: F, t6002: F, t9663: F, t9632: F, t19104: F, t219: F, t1883: F, t24476: F, t19142: F, t6021: F, t1880: F, t9615: F) -> (F, F, F, F, F, F, F, F, F) {
    let t63308 = t19082 * t8507;
    let t63309 = t3138 * t63308;
    let t63314 = t3124 * t63308;
    let t63318 = t1877 * t9699 / 5184.0;
    let t63319 = t6002 * t9663;
    let t63327 = t6002 * t9632;
    let t63339 = t19104 * t219;
    let t63357 = t1883 * t24476;
    let t63371 = t6021 * t19142;
    let t63383 = t9615 * t1880;
    (t63309, t63314, t63318, t63319, t63327, t63339, t63357, t63371, t63383)
}
