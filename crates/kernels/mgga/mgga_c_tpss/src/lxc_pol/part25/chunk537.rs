//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 537/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk537<F: Float>(t2593: F, t309: F, t2453: F, t2511: F, t894: F) -> (F, F, F, F, F) {
    let t2594 = t309 * t2593;
    let t2601 = 0.40256666666666666667e0 * t2453;
    let t2608 = 0.137975e0 * t2511;
    let t2617 = t894 * t894;
    let t2618 = 1.0 / t2617;
    (t2594, t2601, t2608, t2617, t2618)
}
