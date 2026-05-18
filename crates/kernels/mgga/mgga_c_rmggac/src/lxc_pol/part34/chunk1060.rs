//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1060/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1060<F: Float>(t25854: F, t78223: F, t72087: F, t76415: F, t76416: F, t76425: F, t76427: F, t76429: F, t76476: F, t78214: F, t78215: F, t78216: F, t78219: F, t78222: F) -> F {
    let t78225 = F::new(0.35922725105591425692e0) * t25854 * t78223;
    let t78226 = t76415 - t76416 - t78214 + t76425 - t76427 - t76429 + t78215 + t78216 + t78219 + t72087 - t76476 - t78222 + t78225;
    t78226
}
