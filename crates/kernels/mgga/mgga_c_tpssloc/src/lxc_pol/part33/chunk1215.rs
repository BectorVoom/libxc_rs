//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1215/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1215<F: Float>(t154: F, t21: F, t6896: F, t6898: F, t213: F, t6924: F, t9223: F, t22715: F, t547: F, t22822: F, t281: F, t120: F, t22816: F) -> (F, F, F, F, F, F, F) {
    let t80741 = t21 * t154;
    let t80742 = t80741 * t6896;
    let t80743 = t80742 * t6898;
    let t80744 = F::new(0.16220877603642232915e0) * t80743;
    let t80766 = t9223 * t6924 * t213;
    let t80775 = t22715 * t547;
    let t80779 = t22822 * t6924 * t281;
    let t80782 = t22816 * t120;
    (t80741, t80742, t80744, t80766, t80775, t80779, t80782)
}
