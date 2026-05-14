//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 740/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk740<F: Float>(t5222: F, t5269: F, t1586: F, t3118: F, t466: F, t5248: F, t1561: F, t1578: F, t5242: F, t1141: F, t1143: F, t220: F, t3124: F, t3126: F, t3138: F, t3139: F, t468: F) -> (F, F, F, F, F, F, F, F) {
    let t5270 = t5222 + t5269;
    let t5271 = param_beta * t5270;
    let t5275 = t1586 * t1586;
    let t5276 = t3118 * t5275;
    let t5279 = t466 * t5248;
    let t5283 = t1578 * t1561;
    let t5287 = t466 * t5242;
    let t5294 = 2.0 * t1141 * t1143 * t5283 + t1141 * t1143 * t5287 + t220 * t468 * t5270 + 2.0 * t3124 * t3126 * t5279 - t3138 * t3139 * t5279;
    (t5270, t5271, t5275, t5276, t5279, t5283, t5287, t5294)
}
