//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1768/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1768<F: Float>(t131: F, t1365: F, t22648: F, t6897: F, t794: F, t154: F, t21: F, t6896: F, t6898: F, t22797: F, t3770: F, t213: F, t6924: F, t9223: F) -> (F, F, F, F, F, F, F) {
    let t80730 = t1365 * t131;
    let t80738 = t6897 * t794 * t22648;
    let t80741 = t21 * t154;
    let t80742 = t80741 * t6896;
    let t80743 = t80742 * t6898;
    let t80761 = t22797 * t3770;
    let t80766 = t9223 * t6924 * t213;
    (t80730, t80738, t80741, t80742, t80743, t80761, t80766)
}
