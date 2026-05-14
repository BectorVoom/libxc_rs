//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 600/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk600<F: Float>(t3096: F, t66: F, t1134: F, t219: F, t1137: F, t471: F, t73: F, t2711: F, t2712: F, t3048: F, t2785: F, t3054: F, t1107: F, t450: F, t475: F, t1183: F, t177: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3097 = t66 * t3096;
    let t3113 = t1134 * t219;
    let t3117 = 1.0 / t1137 / t471;
    let t3118 = t73 * t3117;
    let t3124 = t2711 * t2712 * t3048;
    let t3126 = t2785 * t3054;
    let t3137 = t2712 * t1107;
    let t3138 = t2711 * t3137;
    let t3139 = t2785 * t450;
    let t3153 = t475 * t475;
    let t3154 = 1.0 / t3153;
    let t3178 = t1183 * t177;
    (t3097, t3113, t3118, t3124, t3126, t3138, t3139, t3153, t3154, t3178)
}
