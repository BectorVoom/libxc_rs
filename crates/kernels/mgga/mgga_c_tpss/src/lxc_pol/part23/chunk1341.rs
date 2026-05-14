//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1341/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1341<F: Float>(t20920: F, t3154: F, t6527: F, t9519: F, t1151: F, t1153: F, t12639: F, t1589: F, t19168: F, t19172: F, t198: F, t20924: F, t20929: F, t3147: F, t3151: F, t330: F, t4023: F, t4325: F, t50434: F, t6044: F, t63437: F, t63441: F, t63448: F, t68243: F, t68308: F, t68530: F, t68591: F) -> (F,) {
    let t68597 = t20920 * t3154;
    let t68601 = t6527 * t9519;
    let t68628 = t198 * t330 * (t68243 + t68308 + t68530 + t68591) * t1153 - 2.0 * t4023 * t68597 * t1151 + 2.0 * t4023 * t68601 * t3151 - t4023 * t20924 * t3147 - t4023 * t63437 * t1589 + 4.0 * t4023 * t63441 * t20929 - 2.0 * t4023 * t19168 * t4325 - 6.0 * t4023 * t63448 * t1589 * t3151 + 4.0 * t4023 * t19172 * t50434 + 2.0 * t4023 * t19172 * t1589 * t3147 - t4023 * t6044 * t12639;
    (t68628,)
}
