//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 538/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk538<F: Float>(t45: F, t57: F, t1985: F, t1992: F, t741: F, t80: F, t745: F, t83: F, zeta_threshold: F) -> (F,) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t2125 = piecewise3(t151, 0.0, -2.0 / 9.0 * t80 * t1985 + 2.0 / 3.0 * t741 * t1992);
    let t2131 = piecewise3(t155, 0.0, -2.0 / 9.0 * t83 * t1985 - 2.0 / 3.0 * t745 * t1992);
    let t2133 = t2125 / 2.0 + t2131 / 2.0;
    (t2133,)
}
