//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1748/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1748<F: Float>(t131: F, t1365: F, t154: F, t21: F, t6896: F, t6898: F, t213: F, t6924: F, t9223: F, t6928: F, t22715: F, t547: F) -> (F, F, F, F, F, F, F) {
    let t80730 = t1365 * t131;
    let t80741 = t21 * t154;
    let t80742 = t80741 * t6896;
    let t80743 = t80742 * t6898;
    let t80766 = t9223 * t6924 * t213;
    let t80767 = t80766 * t6928;
    let t80775 = t22715 * t547;
    (t80730, t80741, t80742, t80743, t80766, t80767, t80775)
}
