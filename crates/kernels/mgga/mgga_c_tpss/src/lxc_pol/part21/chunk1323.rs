//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1323/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1323<F: Float>(t18544: F, t6277: F, t19605: F, t5706: F, t13235: F, t19315: F, t19448: F, t2056: F, t2105: F, t3499: F, t3537: F, t485: F, t5692: F, t6117: F, t6228: F, t626: F, t65082: F, t65088: F, t65091: F, t65093: F, t65096: F, t65099: F, t65101: F, t65106: F, t65109: F, t65115: F, t65125: F, t65128: F) -> (F,) {
    let t65129 = t18544 * t6277;
    let t65131 = 6.0 * t5706 * t19605;
    let t65132 = -2.0 * t2105 * t6228 * t626 - 4.0 * t3537 * t5692 * t626 - 2.0 * t13235 * t6117 - 4.0 * t19315 * t2056 - 4.0 * t19448 * t3499 - 2.0 * t485 * t65082 + t65088 - t65091 - t65093 - t65096 - t65099 - t65101 - t65106 + t65109 - t65115 + t65125 + t65128 - t65129 + t65131;
    (t65132,)
}
