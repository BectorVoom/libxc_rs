//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 648/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk648<F: Float>(t3073: F, t466: F, t1107: F, t2712: F, t2711: F, t2785: F, t450: F, t1141: F, t1143: F, t220: F, t3110: F, t3124: F, t3125: F, t3126: F, t3130: F, t468: F) -> (F, F, F) {
    let t3134 = t466 * t3073;
    let t3137 = t2712 * t1107;
    let t3138 = t2711 * t3137;
    let t3139 = t2785 * t450;
    let t3144 = 2.0 * t1141 * t1143 * t3130 + t1141 * t1143 * t3134 + t220 * t3110 * t468 + 2.0 * t3124 * t3125 * t3126 - t3125 * t3138 * t3139;
    (t3138, t3139, t3144)
}
