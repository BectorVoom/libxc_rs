//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 588/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk588<F: Float>(t3187: F, t3188: F, t1022: F, t1049: F, t1060: F, t3120: F, t381: F, t1014: F, t3032: F, t3031: F) -> (F, F, F, F, F, F, F) {
    let t3189 = t3187 * t3188;
    let t3192 = t1049 * t1022;
    let t3193 = t3192 * t1060;
    let t3196 = t381 * t3120;
    let t3197 = t3196 * t1060;
    let t3199 = t3032 * t1014;
    let t3200 = t3031 * t3199;
    (t3189, t3192, t3193, t3196, t3197, t3199, t3200)
}
