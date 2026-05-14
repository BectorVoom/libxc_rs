//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 389/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk389<F: Float>(t1163: F, t1168: F, t118: F, t1273: F, t485: F, t488: F, t544: F, t624: F, t626: F, t646: F, t3: F, t546: F) -> (F, F, F, F) {
    let t1275 = -t1163 * t118 + t1168 * t544 + t1273 * t488 - t485 * t624 - 2.0 * t626 * t646;
    let t1276 = t3 * t1275;
    let t1278 = t3 * t546;
    let t1279 = param_d * t1275;
    (t1275, t1276, t1278, t1279)
}
