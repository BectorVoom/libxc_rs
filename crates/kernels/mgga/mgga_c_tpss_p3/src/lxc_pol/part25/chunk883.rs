//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 883/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk883<F: Float>(t202: F, t57: F, t37: F, t691: F, t157: F, t36: F, t2435: F, t255: F) -> (F, F, F, F) {
    let t8061 = F::cast_from(1.0_f64) / t202 / t57;
    let t8082 = t37 * t691;
    let t8087 = t36 * t157;
    let t8096 = F::cast_from(1.0_f64) / t2435 / t255;
    (t8061, t8082, t8087, t8096)
}
