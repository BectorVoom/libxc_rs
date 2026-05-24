//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 225/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk225<F: Float>(t730: F, t732: F, t177: F, t192: F) -> (F, F) {
    let t734 = F::cast_from(0.18311447306006545054e-3_f64) * t730 * t732;
    let t735 = t192 * t177;
    (t734, t735)
}
