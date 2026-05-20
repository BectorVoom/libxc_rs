//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 776/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk776<F: Float>(t1118: F, t3265: F, t3264: F, t407: F, t410: F, t1102: F, t3236: F, t3238: F, t3245: F, t3250: F, t3254: F, t1100: F) -> (F, F, F, F, F, F, F, F) {
    let t3266 = t3265 * t1118;
    let t3268 = F::new(2.0) * t3264 * t3266;
    let t3270 = F::new(1.0) / t410 / t407;
    let t3271 = t1102 * t1102;
    let t3272 = t3270 * t3271;
    let t3274 = F::new(4.0) / F::new(9.0) * t3236;
    let t3279 = t3274 - F::new(2.0) / F::new(9.0) * t3238 - F::new(2.0) / F::new(9.0) * t3245 + F::new(2.0) / F::new(3.0) * t3250 + t3254 / F::new(3.0);
    let t3280 = t1100 * t3279;
    (t3266, t3268, t3270, t3271, t3272, t3274, t3279, t3280)
}
