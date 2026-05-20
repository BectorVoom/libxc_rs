//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 906/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk906<F: Float>(t3866: F, t6431: F, t120: F, t6414: F, t225: F, t6364: F, t6435: F, t6362: F, t1390: F, t6463: F, t3701: F, t6324: F) -> (F, F, F, F, F, F, F) {
    let t19942 = t3866 * t6431;
    let t19956 = t120 * t6414;
    let t20029 = t6364 * t225;
    let t20044 = t6435 * t225;
    let t20060 = t6362 * t225;
    let t20067 = t6463 * t1390;
    let t20077 = t6324 * t3701;
    (t19942, t19956, t20029, t20044, t20060, t20067, t20077)
}
