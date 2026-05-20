//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3156/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3156<F: Float>(t18710: F, t300: F, t1166: F, t1164: F, t3396: F, t6105: F, t18933: F, t63763: F, t63765: F, t63767: F, t63769: F, t63771: F, t63829: F, t64100: F, t64253: F, t64259: F, t64433: F) -> (F, F, F, F) {
    let t65288 = t300 * t18710;
    let t65290 = F::cast_from(0.11696447245269292414e1_f64) * t65288 * t1166;
    let t65293 = F::cast_from(0.35089341735807877242e1_f64) * t1164 * t6105 * t3396;
    let t65296 = F::cast_from(0.11696447245269292414e1_f64) * t1164 * t18933 * t3396;
    let t65297 = t63763 + t63765 - t63767 + t63769 + t63771 - t63829 + t64100 + t64253 - t64259 - t65290 - t65293 + t64433 + t65296;
    (t65290, t65293, t65296, t65297)
}
