//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 843/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk843<F: Float>(t1103: F, t1116: F, t1130: F, t6001: F, t6002: F, t6007: F, t6011: F, t6013: F) -> (F,) {
    let t6016 = t6001 - t6002 * t1103 / 288.0 + t6007 * t1116 / 1536.0 + t6011 - t6013 * t1130 / 2304.0;
    (t6016,)
}
