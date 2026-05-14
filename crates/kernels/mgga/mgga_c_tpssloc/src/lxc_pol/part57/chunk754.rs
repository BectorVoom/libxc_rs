//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 754/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk754<F: Float>(t6547: F, t8357: F, t1902: F, t234: F, t794: F, t8356: F, t6562: F, t6585: F, t8339: F, t6600: F, t6599: F, t240: F, t241: F, t814: F, t812: F, t235: F, t835: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30675 = 0.38381794893125283518e-1 * t6547 * t8357;
    let t30676 = t234 * t1902;
    let t30681 = t794 * t8356;
    let t30683 = 0.82246703342411321825e-2 * t6562 * t30681;
    let t30697 = t6585 * t8339;
    let t30703 = t6600 * t8339;
    let t30704 = t6599 * t30703;
    let t30713 = t814 * t240 * t241;
    let t30714 = t812 * t30713;
    let t30719 = t235 * t835;
    (t30675, t30676, t30681, t30683, t30697, t30703, t30704, t30713, t30714, t30719)
}
