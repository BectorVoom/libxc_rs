//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1396/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1396<F: Float>(t11832: F, t456: F, t1197: F, t698: F, t1174: F, t135: F, t3551: F, t3556: F, t3493: F, t3612: F, t11812: F, t1243: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11834 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t456 * t11832;
    let t11835 = t698 * t1197;
    let t11836 = t1174 * t11835;
    let t11838 = t135 * t3551;
    let t11839 = t1174 * t11838;
    let t11841 = t135 * t3556;
    let t11842 = t1174 * t11841;
    let t11871 = t3612 * t3493;
    let t11877 = t11812 * t1243;
    (t11834, t11835, t11836, t11838, t11839, t11841, t11842, t11871, t11877)
}
