//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 367/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk367<F: Float>(t1172: F, t1175: F, t1185: F, t1187: F, t1189: F, t1192: F, t1195: F, t219: F, t654: F, t679: F, t527: F, t73: F) -> (F, F) {
    let t1226 = (t654 + t679 + t1172 - t1175 + t1185 + t1187 + t1189 - t1192 - t1195) * t219;
    let t1228 = t73 * t527;
    (t1226, t1228)
}
