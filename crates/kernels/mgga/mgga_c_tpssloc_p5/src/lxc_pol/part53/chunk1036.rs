//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1036/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1036<F: Float>(t225: F, t33815: F, t120350: F, t120363: F, t120375: F, t113966: F, t114000: F, t115450: F, t117217: F, t120342: F, t120344: F, t120348: F, t120357: F, t120366: F, t120369: F, t120372: F, t120377: F, t120379: F, t120381: F, t120383: F) -> (F, F) {
    let t124124 = t33815 * t225;
    let t124139 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t120350;
    let t124142 = F::cast_from(0.5383034145885385447e-3_f64) * t120363;
    let t124146 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t120375;
    let t124152 = -t120342 / F::cast_from(384.0_f64) - t120344 / F::cast_from(384.0_f64) - t120348 / F::cast_from(384.0_f64) + t124139 + F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t120357 + F::cast_from(0.22608743412718618877e-1_f64) * t113966 + t124142 - t117217 + F::cast_from(0.19378922925187387609e-1_f64) * t120366 + F::cast_from(0.19378922925187387609e-1_f64) * t120369 - F::cast_from(0.32298204875312312682e-2_f64) * t120372 + t115450 + t124146 - t120377 / F::cast_from(96.0_f64) - t120379 / F::cast_from(96.0_f64) - t120381 / F::cast_from(96.0_f64) + F::cast_from(0.13565246047631171326e0_f64) * t120383 + F::cast_from(0.13565246047631171326e0_f64) * t114000;
    (t124124, t124152)
}
