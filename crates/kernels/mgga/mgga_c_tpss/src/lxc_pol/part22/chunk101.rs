//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 101/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk101<F: Float>(t246: F, t219: F, t73: F, t220: F, t229: F) -> (F, F, F, F) {
    let t247 = param_beta * t246;
    let t248 = t219 * t73;
    let t251 = t220 * t229 * t246 + F::new(1.0);
    let t252 = F::new(1.0) / t251;
    let t253 = t248 * t252;
    (t247, t248, t251, t253)
}
