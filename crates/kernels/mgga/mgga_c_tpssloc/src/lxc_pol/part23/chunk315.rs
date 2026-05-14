//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 315/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk315<F: Float>(t1089: F, t461: F, t1169: F, t221: F, t456: F, t1176: F, t1009: F, t466: F, t1011: F, t476: F) -> (F, F, F, F, F, F, F, F) {
    let t1178 = t461 * t1089;
    let t1193 = t221 * t1169;
    let t1195 = t456 * t1193 / 288.0;
    let t1196 = t1176 * t1089;
    let t1206 = t466 * t1009;
    let t1207 = t1206 * t1011;
    let t1208 = t476 * t476;
    let t1209 = 1.0 / t1208;
    (t1178, t1193, t1195, t1196, t1206, t1207, t1208, t1209)
}
