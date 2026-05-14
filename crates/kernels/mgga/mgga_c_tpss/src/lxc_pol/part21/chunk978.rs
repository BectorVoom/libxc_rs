//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 978/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk978<F: Float>(t57: F, t3431: F, t83: F, t10353: F, t1311: F, t1985: F, t1992: F, t3602: F, t581: F, t745: F, t10539: F, zeta_threshold: F) -> (F,) {
    let t155 = t57 <= zeta_threshold;
    let t10542 = t83 * t3431;
    let t10550 = piecewise3(t155, 0.0, -8.0 / 27.0 * t1311 * t1985 - 4.0 / 9.0 * t10542 * t581 - 2.0 / 9.0 * t3602 * t1992 - 2.0 / 3.0 * t745 * t10353);
    let t10552 = t10539 / 2.0 + t10550 / 2.0;
    (t10552,)
}
