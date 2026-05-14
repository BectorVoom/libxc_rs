//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1211/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1211<F: Float>(t1642: F, t60706: F, t12853: F, t5728: F, t18450: F, t4462: F, t12960: F, t5721: F, t12869: F, t19476: F, t4473: F, t60738: F, t12873: F, t18454: F, t18464: F, t4484: F) -> (F, F, F, F, F, F, F, F) {
    let t65624 = t60706 * t1642;
    let t65626 = t5728 * t12853;
    let t65628 = t18450 * t4462;
    let t65630 = t5721 * t12960;
    let t65636 = t19476 * t12869;
    let t65639 = t60738 * t4473;
    let t65641 = t18454 * t12873;
    let t65643 = t18464 * t4484;
    (t65624, t65626, t65628, t65630, t65636, t65639, t65641, t65643)
}
