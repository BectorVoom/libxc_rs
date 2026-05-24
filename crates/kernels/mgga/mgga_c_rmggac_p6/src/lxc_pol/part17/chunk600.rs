//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 600/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk600<F: Float>(t7756: F, t8465: F, t2010: F, t2415: F, t7760: F, t7349: F, t270: F, t575: F, t2039: F, t638: F, t31: F, t2046: F, t2050: F) -> (F, F, F, F, F, F, F, F) {
    let t8466 = t8465 * t7756;
    let t8467 = t2010 * t8466;
    let t8469 = t2415 * t7760;
    let t8470 = t7349 * t8469;
    let t8475 = t575 * t270;
    let t8477 = t638 * t2039 * t8475;
    let t8482 = t575 * t31;
    let t8484 = t2046 * t2050 * t8482;
    (t8466, t8467, t8469, t8470, t8475, t8477, t8482, t8484)
}
