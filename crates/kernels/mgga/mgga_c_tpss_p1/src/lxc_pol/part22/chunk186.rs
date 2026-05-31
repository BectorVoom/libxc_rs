//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 186/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk186<F: Float>(t582: F, t70: F, t39: F, t41: F, rho0: F, sigma0: F) -> (F, F, F, F) {
    let t583 = t582 * t70;
    let t586 = t39 * rho0;
    let t588 = F::cast_from(1.0_f64) / t41 / t586;
    let t589 = sigma0 * t588;
    (t583, t586, t588, t589)
}
