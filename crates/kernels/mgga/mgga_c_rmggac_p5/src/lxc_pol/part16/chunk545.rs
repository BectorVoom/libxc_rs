//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 545/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk545<F: Float>(t31: F, t7352: F, t7351: F, t7349: F, t1179: F, t214: F, t1968: F, t1966: F) -> (F, F, F, F, F, F) {
    let t7353 = t7352 * t31;
    let t7354 = t7351 * t7353;
    let t7355 = t7349 * t7354;
    let t7363 = t1179 * t214;
    let t7364 = t7363 * t1968;
    let t7365 = t1966 * t7364;
    (t7353, t7354, t7355, t7363, t7364, t7365)
}
