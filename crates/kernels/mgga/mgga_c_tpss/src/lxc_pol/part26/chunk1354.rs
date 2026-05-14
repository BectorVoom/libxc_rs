//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1354/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1354<F: Float>(t15503: F, t6013: F, t15488: F, t19077: F, t15805: F, t6005: F, t938: F, t15491: F, t1875: F, t339: F, t20837: F, t4275: F, t1116: F, t1130: F, t15799: F, t6007: F, t68373: F, t68387: F, t68391: F, t68393: F, t68394: F) -> (F,) {
    let t73318 = t6013 * t15503;
    let t73320 = t19077 * t15488;
    let t73323 = t938 * t6005 * t15805;
    let t73327 = t339 * t1875 * t15491;
    let t73330 = t20837 * t4275;
    let t73333 = t6007 * t15799 / 1536.0 - t68373 + t68387 - t73318 / 3456.0 + t73320 / 1152.0 + 19.0 / 864.0 * t73323 * t1116 - 19.0 / 1296.0 * t73327 * t1130 + t73330 / 324.0 - t68391 - t68393 - t68394 / 3456.0;
    (t73333,)
}
