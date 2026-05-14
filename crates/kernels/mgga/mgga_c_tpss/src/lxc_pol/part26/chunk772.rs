//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 772/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk772<F: Float>(t3038: F, t4573: F, t926: F, t3033: F, t1098: F, t1558: F, t1564: F, t3027: F, t3089: F, t4212: F, t4217: F, t4239: F, t4258: F, t4261: F, t4276: F, t5207: F) -> (F, F, F, F, F) {
    let t5210 = t3038 * t4573;
    let t5211 = t926 * t5210;
    let t5214 = t3033 * t4573;
    let t5215 = t926 * t5214;
    let t5222 = -t3027 - t4258 * t1564 / 288.0 + t4212 * t1558 / 54.0 - t1098 * t5207 / 288.0 - t1098 * t5211 / 144.0 + t1098 * t5215 / 216.0 - t3089 - t4261 / 432.0 - t4217 / 432.0 - t4276 / 3456.0 + t4239 / 2304.0;
    (t5210, t5211, t5214, t5215, t5222)
}
