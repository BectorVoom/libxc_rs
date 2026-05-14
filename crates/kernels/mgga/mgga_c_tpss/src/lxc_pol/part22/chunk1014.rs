//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1014/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1014<F: Float>(t11910: F, t11942: F, t11932: F, t11938: F, t11952: F, t11955: F, t11960: F, t11963: F, t9221: F, t9223: F, t9226: F, t9228: F, t11848: F, t11850: F, t11853: F, t11896: F, t11899: F, t11904: F, t11908: F, t11913: F, t11916: F, t11919: F, t11922: F, t11925: F, t12024: F, t12040: F, t9183: F, t9192: F, t9194: F, t9196: F, t9429: F, t9438: F) -> (F,) {
    let t12046 = 0.27785333333333333334e0 * t11910;
    let t12060 = 0.34431666666666666666e0 * t11942;
    let t12064 = 0.45908888888888888888e0 * t9221 + 0.11477222222222222222e0 * t9223 - 0.34431666666666666666e0 * t9226 - 0.17215833333333333333e0 * t9228 + 0.46308888888888888889e-1 * t11932 + 0.3529725e1 * t11955 + 0.22954444444444444444e0 * t11938 - t12060 + 0.516475e0 * t11952 + 0.6311625e0 * t11960 + 0.46308888888888888889e-1 * t11963;
    let t12066 = -t9429 + 0.23154444444444444444e-1 * t9183 + 0.23154444444444444444e0 * t9192 - 0.69463333333333333333e-1 * t9194 - 0.13892666666666666667e0 * t9196 - t12024 + 0.104195e0 * t11848 + 0.11577222222222222222e0 * t11850 - t9438 + 0.264729375e1 * t11853 + t12040 - 0.34431666666666666667e0 * t11896 + 0.309885e1 * t11899 + 0.20659e1 * t11904 + 0.103295e1 * t11908 - t12046 - 0.69463333333333333334e-1 * t11913 - 0.34731666666666666667e-1 * t11916 - 0.20839e0 * t11919 + 0.41678e0 * t11922 + 0.20839e0 * t11925 + t12064;
    (t12066,)
}
