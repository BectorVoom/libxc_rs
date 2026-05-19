//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 656/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk656<F: Float>(t3237: F, t3238: F, t3245: F, t3250: F, t3254: F, t423: F, t1094: F, t1098: F, t1119: F, t1097: F, t419: F, t409: F) -> (F, F, F, F, F, F) {
    let t3256 = t3237 - F::cast_from(0.11872222222222222222e-1_f64) * t3238 - F::cast_from(0.11872222222222222222e-1_f64) * t3245 + F::cast_from(0.35616666666666666666e-1_f64) * t3250 + F::cast_from(0.17808333333333333333e-1_f64) * t3254;
    let t3258 = F::new(0.621814e-1) * t3256 * t423;
    let t3259 = t1094 * t1098;
    let t3261 = F::new(2.0) * t3259 * t1119;
    let t3262 = t1097 * t419;
    let t3263 = F::new(1.0) / t3262;
    let t3264 = t409 * t3263;
    (t3256, t3258, t3259, t3261, t3263, t3264)
}
