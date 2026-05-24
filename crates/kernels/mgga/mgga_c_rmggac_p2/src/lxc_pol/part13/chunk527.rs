//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 527/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk527<F: Float>(t2069: F, t333: F, t262: F, t7198: F, t2144: F, t22: F, t507: F) -> (F, F, F, F, F) {
    let t7199 = t2069 * t333;
    let t7200 = t262 * t7199;
    let t7201 = t7198 * t7200;
    let t7203 = t2144 * t22;
    let t7204 = t507 * t7203;
    (t7199, t7200, t7201, t7203, t7204)
}
