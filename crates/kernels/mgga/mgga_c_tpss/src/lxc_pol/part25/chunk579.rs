//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 579/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk579<F: Float>(t1107: F, t2712: F, t2711: F, t2785: F, t450: F, t475: F, t1183: F, t177: F, t737: F, t1193: F, t2206: F, t198: F, t508: F) -> (F, F, F, F, F, F, F, F) {
    let t3137 = t2712 * t1107;
    let t3138 = t2711 * t3137;
    let t3139 = t2785 * t450;
    let t3153 = t475 * t475;
    let t3154 = F::new(1.0) / t3153;
    let t3178 = t1183 * t177;
    let t3179 = t3178 * t737;
    let t3182 = F::new(0.5848223622634646207e0) * t1193 * t2206;
    let t3183 = t198 * t508;
    (t3138, t3139, t3153, t3154, t3178, t3179, t3182, t3183)
}
