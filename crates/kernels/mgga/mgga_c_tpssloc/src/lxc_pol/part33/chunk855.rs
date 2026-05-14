//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 855/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk855<F: Float>(t109: F, t20342: F, t656: F, t12747: F, t19471: F, t19480: F, t20305: F, t20308: F, t64: F, t9358: F) -> (F,) {
    let t110 = 1.0 < t109;
    let t20343 = t656 * t20342;
    let t20347 = piecewise3(t110, 0.0, -t9358 - 11.0 / 3.0 * t12747 - 2.0 * t19471 + t19480 - 3.0 / 4.0 * t64 * t20305 + 3.0 / 4.0 * t64 * t20308 - t64 * t20343 / 8.0);
    (t20347,)
}
