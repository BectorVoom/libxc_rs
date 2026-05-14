//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1360/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1360<F: Float>(t15560: F, t6002: F, t15557: F, t1095: F, t22012: F, t15266: F, t15590: F, t15910: F, t19084: F, t20808: F, t20809: F, t4271: F, t68469: F, t68472: F, t68476: F, t68489: F, t68511: F, t68522: F) -> (F,) {
    let t73443 = t6002 * t15560;
    let t73453 = t6002 * t15557;
    let t73455 = t22012 * t1095;
    let t73459 = t73443 / 648.0 - t68469 + t20808 * t20809 * t15266 / 108.0 + t68472 / 648.0 - t68476 + t68522 * t4271 / 216.0 - t19084 * t15590 / 2304.0 - t73453 / 432.0 + 11.0 / 324.0 * t73455 - t68489 + t68511 - t19084 * t15910 / 1152.0;
    (t73459,)
}
