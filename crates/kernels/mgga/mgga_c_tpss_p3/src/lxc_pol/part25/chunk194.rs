//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 194/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk194<F: Float>(t615: F, t77: F, t583: F, t603: F, t71: F, t85: F) -> (F, F) {
    let t616 = t77 * t615;
    let t619 = -t583 * t85 / F::cast_from(12.0_f64) + t603 * t85 / F::cast_from(24.0_f64) + t71 * t616 / F::cast_from(24.0_f64);
    (t616, t619)
}
