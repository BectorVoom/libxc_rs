//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 764/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk764<F: Float>(t159: F, t9738: F, t2479: F, t9731: F, t9689: F, t9692: F, t9695: F, t9698: F, t9702: F, t9704: F, t9706: F, t9709: F, t731: F, t746: F, t9490: F, t172: F, t9489: F) -> (F, F, F, F, F) {
    let t9739 = t159 * t9738;
    let t9740 = t9731 * t2479;
    let t9751 = -0.47063e1 * t9689 + 0.31375333333333333334e1 * t9692 - 0.36604555555555555556e1 * t9695 - 0.16068111111111111111e1 * t9698 + 0.28051666666666666666e0 * t9702 - 0.56103333333333333332e0 * t9704 - 0.6545388888888888889e0 * t9706 - 0.46308888888888888888e0 * t9709;
    let t9752 = t9751 * t731;
    let t9755 = t9490 * t746;
    let t9758 = t172 * t9489;
    (t9739, t9740, t9752, t9755, t9758)
}
