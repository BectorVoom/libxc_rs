//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 638/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk638<F: Float>(t7473: F, t8450: F, t7478: F, t209: F, t476: F, t615: F, t236: F, t1971: F, t7453: F, t529: F, t7754: F, t7756: F) -> (F, F, F, F, F, F) {
    let t8451 = t8450 * t7473;
    let t8452 = t8451 * t7478;
    let t8455 = t615 * t476 * t209;
    let t8456 = t236 * t8455;
    let t8457 = t1971 * t8456;
    let t8458 = t7453 * t8457;
    let t8465 = t7754 * t529;
    let t8466 = t8465 * t7756;
    (t8451, t8452, t8457, t8458, t8465, t8466)
}
