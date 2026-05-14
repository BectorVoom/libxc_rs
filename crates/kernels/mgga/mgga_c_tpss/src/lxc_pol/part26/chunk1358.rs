//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1358/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1358<F: Float>(t20802: F, t4216: F, t15834: F, t6002: F, t15281: F, t15839: F, t15855: F, t15873: F, t15895: F, t19077: F, t19084: F, t20808: F, t20809: F, t20810: F, t4234: F, t4253: F, t6013: F, t63292: F, t63309: F, t68413: F, t68417: F, t73360: F) -> (F,) {
    let t73413 = t20802 * t4216;
    let t73415 = t6002 * t15834;
    let t73419 = -2.0 / 81.0 * t73360 * t20810 + t20808 * t20809 * t15281 / 216.0 - t19084 * t15895 / 1152.0 + t63292 / 1296.0 + 5.0 / 3456.0 * t6013 * t15873 - t68413 * t4234 / 72.0 + t68417 * t4253 / 144.0 + t19077 * t15839 / 384.0 + t73413 / 162.0 - t73415 / 864.0 + t63309 * t15855 / 2304.0;
    (t73419,)
}
