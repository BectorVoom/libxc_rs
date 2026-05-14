//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 830/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk830<F: Float>(t457: F, t63: F, t461: F, t221: F, t456: F, t1186: F, t698: F, t1174: F, t1184: F, t4899: F, t3242: F, t460: F, t3247: F, t1176: F, t134: F, t1239: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11552 = t63 * t457;
    let t11553 = t11552 * t461;
    let t11554 = t221 * t11553;
    let t11556 = 0.3086419753086419753e-3 * t456 * t11554;
    let t11557 = t698 * t1186;
    let t11558 = t1174 * t11557;
    let t11569 = t4899 * t1184;
    let t11570 = t460 * t3242;
    let t11583 = t460 * t3247;
    let t11588 = t134 * t1176;
    let t11589 = t11588 * t1184;
    let t11604 = t1239 * t1239;
    (t11552, t11556, t11558, t11569, t11570, t11583, t11588, t11589, t11604)
}
