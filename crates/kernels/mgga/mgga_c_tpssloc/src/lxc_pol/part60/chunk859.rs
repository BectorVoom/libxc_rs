//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 859/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk859<F: Float>(t118602: F, t120350: F, t120363: F, t120375: F, t120393: F, t120416: F, t2105: F, t8110: F, t112: F, t34175: F, t111: F, t34136: F, t1437: F, t63: F, t117496: F, t1409: F, t31864: F, t8308: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t123578 = 7.0 / 576.0 * t118602;
    let t124139 = 7.0 / 576.0 * t120350;
    let t124142 = 0.5383034145885385447e-3 * t120363;
    let t124146 = 7.0 / 144.0 * t120375;
    let t124154 = 0.32298204875312312682e-2 * t120393;
    let t124163 = 7.0 / 576.0 * t120416;
    let t124673 = t8110 * t2105;
    let t124676 = t34175 * t112;
    let t124728 = t34136 * t111;
    let t124755 = t63 * t1437;
    let t124803 = t31864 * t8308 * t117496 * t1409;
    (t123578, t124139, t124142, t124146, t124154, t124163, t124673, t124676, t124728, t124755, t124803)
}
