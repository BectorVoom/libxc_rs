//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 930/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk930<F: Float>(t486: F, t676: F, t1216: F, t248: F, t1213: F, t1226: F, t3566: F, t11552: F, t221: F, t456: F, t1197: F, t698: F, t1174: F, t135: F, t3551: F, t3556: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11818 = t676 * t486;
    let t11820 = t248 * t11818 * t1216;
    let t11821 = t1213 * t11820;
    let t11825 = t3566 * t1226;
    let t11832 = t221 * t11552;
    let t11834 = 5.0 / 1296.0 * t456 * t11832;
    let t11835 = t698 * t1197;
    let t11836 = t1174 * t11835;
    let t11838 = t135 * t3551;
    let t11839 = t1174 * t11838;
    let t11841 = t135 * t3556;
    (t11820, t11821, t11825, t11832, t11834, t11835, t11836, t11838, t11839, t11841)
}
