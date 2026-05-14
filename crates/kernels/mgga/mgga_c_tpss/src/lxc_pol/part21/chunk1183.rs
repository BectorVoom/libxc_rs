//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1183/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1183<F: Float>(t114: F, t18397: F, t2074: F, t2100: F, t5527: F, t18393: F, t18396: F) -> (F,) {
    let t115 = 1.0 < t114;
    let t18398 = t18397 * t2074;
    let t18400 = t5527 * t2100;
    let t18403 = piecewise3(t115, 0.0, t18393 + t18396 + t18398 / 4.0 - t18400 / 8.0);
    (t18403,)
}
