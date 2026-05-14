//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1320/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1320<F: Float>(t14917: F, t14956: F, t14970: F, t15062: F, t15066: F, t15084: F, t15093: F, t15097: F, t15102: F, t18069: F, t18094: F, t18098: F, t4985: F, t4996: F, t5605: F, t5620: F, t61322: F, t61329: F, t61341: F, t61344: F, t61350: F, t61363: F, t61422: F, t61432: F, t64343: F, t64354: F, t64358: F, t64401: F, t64403: F) -> (F,) {
    let t70365 = t61344 * t4996 / 288.0 - t5620 * t15062 / 1152.0 + 5.0 / 6912.0 * t5620 * t15066 + t5605 * t14956 / 288.0 - t64343 + 5.0 / 3456.0 * t18069 * t14970 - t64354 + t64358 - t61322 / 6912.0 + t18094 * t15084 / 384.0 + t61329 - t61350 * t15093 / 256.0 - t61341 / 10368.0 + t61363 / 162.0 - t64401 + t64403 - t18069 * t15097 / 576.0 + t61432 * t15102 / 576.0 - t18098 * t14917 / 768.0 - t61422 * t4985 / 216.0;
    (t70365,)
}
