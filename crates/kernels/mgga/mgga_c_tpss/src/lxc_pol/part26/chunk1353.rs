//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1353/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1353<F: Float>(t1561: F, t6516: F, t5275: F, t6032: F, t20913: F, t5242: F, t15569: F, t15574: F, t15578: F, t15843: F, t15869: F, t15886: F, t15919: F, t15924: F, t19077: F, t19084: F, t19090: F, t20837: F, t4280: F, t6013: F, t63282: F, t63309: F, t63314: F, t68361: F, t68365: F) -> (F, F, F, F, F) {
    let t73264 = t6516 * t1561;
    let t73278 = t6032 * t5275;
    let t73285 = t20913 * t1561;
    let t73289 = t6032 * t5242;
    let t73315 = t19077 * t15924 / 768.0 - t19090 * t15919 / 1536.0 - t19090 * t15843 / 768.0 + t63282 * t15869 / 1536.0 - 5.0 / 2592.0 * t6013 * t15886 - 5.0 / 648.0 * t20837 * t4280 - t63314 * t15569 / 576.0 + t63309 * t15574 / 1152.0 - t19084 * t15578 / 576.0 + t68361 - t68365;
    (t73264, t73278, t73285, t73289, t73315)
}
