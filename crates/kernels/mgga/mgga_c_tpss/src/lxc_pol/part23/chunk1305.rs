//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1305/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1305<F: Float>(t60731: F, t12869: F, t19476: F, t4473: F, t60738: F, t12873: F, t18454: F, t18464: F, t4484: F, t13015: F, t5728: F, t1646: F, t60749: F, t60750: F, t60725: F, t60733: F, t60739: F, t60744: F, t60752: F) -> (F,) {
    let t65634 = 35.0 / 108.0 * t60731;
    let t65636 = t19476 * t12869;
    let t65639 = t60738 * t4473;
    let t65640 = 7.0 / 288.0 * t65639;
    let t65641 = t18454 * t12873;
    let t65643 = t18464 * t4484;
    let t65644 = 7.0 / 288.0 * t65643;
    let t65645 = t5728 * t13015;
    let t65647 = t60749 * t1646;
    let t65650 = 119.0 / 864.0 * t60750;
    let t65652 = -7.0 / 48.0 * t60725 - t65634 + 7.0 / 144.0 * t60733 + t65636 / 384.0 - 7.0 / 288.0 * t60739 - t65640 + t65641 / 384.0 + t65644 - t65645 / 384.0 - 119.0 / 1728.0 * t65647 - 35.0 / 576.0 * t60744 - t65650 + 7.0 / 576.0 * t60752;
    (t65652,)
}
