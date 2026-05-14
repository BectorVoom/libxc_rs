//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1120/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1120<F: Float>(t15599: F, t15795: F, t450: F, t1112: F, t242: F, t1098: F, t1111: F, t12406: F, t12409: F, t15569: F, t15574: F, t15578: F, t15582: F, t15586: F, t15590: F, t15596: F, t3067: F, t4212: F, t4228: F, t9556: F, t9573: F) -> (F, F, F) {
    let t15796 = t15599 + t15795;
    let t15797 = t15796 * t450;
    let t15799 = t242 * t1112 * t15797;
    let t15802 = -t9556 * t15569 / 1152.0 + t9573 * t15574 / 2304.0 - t3067 * t15578 / 1152.0 + 5.0 / 6912.0 * t3067 * t15582 + t12406 + t12409 - t3067 * t15586 / 2304.0 - t3067 * t15590 / 4608.0 + t4212 * t4228 / 54.0 - t1098 * t15596 / 288.0 + t1111 * t15799 / 3072.0;
    (t15796, t15799, t15802)
}
