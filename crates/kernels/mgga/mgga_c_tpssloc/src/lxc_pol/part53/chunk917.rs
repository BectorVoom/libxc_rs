//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 917/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk917<F: Float>(t225: F, t33815: F, t120350: F, t120363: F, t120375: F, t113966: F, t114000: F, t115450: F, t117217: F, t120342: F, t120344: F, t120348: F, t120357: F, t120366: F, t120369: F, t120372: F, t120377: F, t120379: F, t120381: F, t120383: F) -> (F, F) {
    let t124124 = t33815 * t225;
    let t124139 = 7.0 / 576.0 * t120350;
    let t124142 = 0.5383034145885385447e-3 * t120363;
    let t124146 = 7.0 / 144.0 * t120375;
    let t124152 = -t120342 / 384.0 - t120344 / 384.0 - t120348 / 384.0 + t124139 + 5.0 / 96.0 * t120357 + 0.22608743412718618877e-1 * t113966 + t124142 - t117217 + 0.19378922925187387609e-1 * t120366 + 0.19378922925187387609e-1 * t120369 - 0.32298204875312312682e-2 * t120372 + t115450 + t124146 - t120377 / 96.0 - t120379 / 96.0 - t120381 / 96.0 + 0.13565246047631171326e0 * t120383 + 0.13565246047631171326e0 * t114000;
    (t124124, t124152)
}
