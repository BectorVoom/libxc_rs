//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 700/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk700<F: Float>(t1176: F, t698: F, t135: F, t3439: F, t3247: F, t405: F, t974: F, t11147: F, t461: F, t457: F, t63: F, t221: F, t456: F, t3242: F, t460: F, t134: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11529 = t698 * t1176;
    let t11539 = t135 * t3439;
    let t11545 = 1.0 / t405 / t3247;
    let t11546 = t974 * t11545;
    let t11547 = t461 * t11147;
    let t11552 = t63 * t457;
    let t11553 = t11552 * t461;
    let t11554 = t221 * t11553;
    let t11556 = 0.3086419753086419753e-3 * t456 * t11554;
    let t11570 = t460 * t3242;
    let t11583 = t460 * t3247;
    let t11588 = t134 * t1176;
    (t11529, t11539, t11545, t11546, t11547, t11552, t11556, t11570, t11583, t11588)
}
