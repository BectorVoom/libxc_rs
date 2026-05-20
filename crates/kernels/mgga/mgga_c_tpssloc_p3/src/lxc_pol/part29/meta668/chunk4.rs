//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2234/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2234<F: Float>(t16215: F, t221: F, t91194: F, t6604: F, t80893: F, t1361: F, t6925: F, t6976: F, t22828: F, t26243: F, t26271: F, t80779: F) -> (F, F, F, F) {
    let t91196 = t91194 * t221 * t16215;
    let t91198 = t80893 * t6604;
    let t91200 = t91198 * t1361 * t16215;
    let t91202 = t6925 * t6976;
    let t91204 = t91202 * t26243 * t22828;
    let t91206 = t80779 * t26271;
    (t91196, t91200, t91204, t91206)
}
