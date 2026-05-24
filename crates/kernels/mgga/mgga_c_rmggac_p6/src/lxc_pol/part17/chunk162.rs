//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 162/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk162<F: Float>(t60: F, t525: F, t62: F, t524: F, zeta_threshold: F) -> F {
    let t61 = t60 <= zeta_threshold;
    let t528 = piecewise3::<F>(t61, F::new(0.0), F::new(4.0) / F::new(3.0) * t62 * t525);
    let t529 = t524 + t528;
    t529
}
