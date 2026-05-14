//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 789/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk789<F: Float>(t159: F, t9729: F, t2461: F, t730: F, t167: F, t2478: F, t164: F, t2475: F, t2479: F, t9689: F, t9692: F, t9695: F, t9698: F, t9702: F, t9704: F, t9706: F, t9709: F) -> (F, F, F, F, F, F, F, F) {
    let t9730 = t159 * t9729;
    let t9731 = t2461 * t730;
    let t9733 = 1.0 / t2478 / t167;
    let t9734 = t9731 * t9733;
    let t9738 = 1.0 / t2475 / t164;
    let t9739 = t159 * t9738;
    let t9740 = t9731 * t2479;
    let t9751 = -0.47063e1 * t9689 + 0.31375333333333333334e1 * t9692 - 0.36604555555555555556e1 * t9695 - 0.16068111111111111111e1 * t9698 + 0.28051666666666666666e0 * t9702 - 0.56103333333333333332e0 * t9704 - 0.6545388888888888889e0 * t9706 - 0.46308888888888888888e0 * t9709;
    (t9730, t9731, t9733, t9734, t9738, t9739, t9740, t9751)
}
