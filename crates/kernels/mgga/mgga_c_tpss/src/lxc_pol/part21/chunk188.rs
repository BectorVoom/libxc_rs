//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 188/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk188<F: Float>(t25: F, t571: F, t553: F, t557: F, t561: F, t565: F, t569: F, t88: F, t90: F) -> (F, F, F) {
    let t573 = 6.0 * t25 * t571;
    let t574 = t553 - t557 + t561 - t565 + t569 - t573;
    let t577 = 1.0 / t90 / t88;
    (t573, t574, t577)
}
