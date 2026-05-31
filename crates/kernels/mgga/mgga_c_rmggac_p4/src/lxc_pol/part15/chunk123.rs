//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 123/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk123<F: Float>(t155: F, t389: F, t163: F, t158: F, t247: F, t250: F, t369: F, t374: F, t166: F) -> (F, F, F, F, F, F) {
    let t390 = t155 * t389;
    let t394 = t163 * t163;
    let t395 = F::cast_from(1.0_f64) / t394;
    let t396 = t158 * t395;
    let t401 = -F::cast_from(0.1176575e1_f64) * t247 - F::cast_from(0.516475e0_f64) * t250 - F::cast_from(0.2103875e0_f64) * t369 - F::cast_from(0.104195e0_f64) * t374;
    let t402 = F::cast_from(1.0_f64) / t166;
    (t390, t394, t395, t396, t401, t402)
}
