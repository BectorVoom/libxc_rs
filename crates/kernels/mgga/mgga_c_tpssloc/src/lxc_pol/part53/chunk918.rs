//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 918/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk918<F: Float>(t120393: F, t120416: F, t115458: F, t115463: F, t115464: F, t115467: F, t117231: F, t117232: F, t117235: F, t120388: F, t120395: F, t120397: F, t120399: F, t120401: F, t120405: F, t120408: F, t120410: F, t120413: F, t120419: F) -> (F,) {
    let t124154 = 0.32298204875312312682e-2 * t120393;
    let t124163 = 7.0 / 576.0 * t120416;
    let t124165 = 0.64596409750624625364e-2 * t120388 + t115458 + t124154 + t120395 / 96.0 - t120397 / 384.0 + t120399 / 96.0 + t120401 / 192.0 + t117231 - 0.19378922925187387609e-1 * t120405 - 0.32298204875312312682e-2 * t120408 + 0.22608743412718618877e-1 * t120410 + t120413 / 384.0 - t124163 + 0.13565246047631171326e0 * t120419 + t117232 + t115463 - t115464 + t117235 + t115467;
    (t124165,)
}
