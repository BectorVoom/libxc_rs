//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 813/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk813<F: Float>(t599: F, t49: F, t5497: F, t581: F, t588: F, t72: F) -> (F, F, F) {
    let t5500 = 8.0 / 3.0 * t599;
    let t5501 = -8.0 / 3.0 * t588 * t49 + 5.0 / 6.0 * t5497 * t581 + t5500;
    let t5502 = t5501 * t72;
    (t5500, t5501, t5502)
}
