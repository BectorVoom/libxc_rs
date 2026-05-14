//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 344/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk344<F: Float>(t1015: F, t1128: F, t242: F, t1097: F, t1098: F, t1103: F, t1111: F, t1116: F, t1122: F, t1125: F) -> (F, F) {
    let t1129 = t1128 * t1015;
    let t1130 = t242 * t1129;
    let t1133 = t1097 - t1098 * t1103 / 288.0 + t1111 * t1116 / 3072.0 + t1122 - t1125 * t1130 / 4608.0;
    (t1130, t1133)
}
