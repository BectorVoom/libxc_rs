//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 665/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk665<F: Float>(t3110: F, t1134: F, t219: F, t1137: F, t471: F, t73: F, t1148: F, t2711: F, t2712: F, t3048: F, t3053: F, t466: F, param_beta: F) -> (F, F, F, F, F, F, F) {
    let t3111 = param_beta * t3110;
    let t3113 = t1134 * t219;
    let t3117 = F::new(1.0) / t1137 / t471;
    let t3118 = t73 * t3117;
    let t3119 = t1148 * t1148;
    let t3120 = t3118 * t3119;
    let t3124 = t2711 * t2712 * t3048;
    let t3125 = t466 * t3053;
    (t3111, t3113, t3118, t3119, t3120, t3124, t3125)
}
