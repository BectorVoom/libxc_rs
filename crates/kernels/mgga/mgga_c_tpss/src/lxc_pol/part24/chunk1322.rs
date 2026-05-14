//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1322/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1322<F: Float>(t14986: F, t5605: F, t14959: F, t14964: F, t14482: F, t14487: F, t14922: F, t15032: F, t15036: F, t15048: F, t15071: F, t15108: F, t18069: F, t18110: F, t19849: F, t19850: F, t19854: F, t4966: F, t4970: F, t4974: F, t61354: F, t61439: F, t61442: F, t64477: F, t64478: F, t64483: F, t64487: F) -> (F,) {
    let t70412 = t5605 * t14986;
    let t70416 = t5605 * t14959;
    let t70427 = t5605 * t14964;
    let t70432 = 5.0 / 6912.0 * t18069 * t15048 - t18069 * t15108 / 1152.0 + t18069 * t15071 / 2304.0 + t61354 * t14922 / 1536.0 + t18069 * t15036 / 1152.0 - t18110 * t4966 / 81.0 + t70412 / 648.0 + t18110 * t4970 / 54.0 - t70416 / 432.0 - t18110 * t4974 / 108.0 + t61439 / 1296.0 + t61442 - t19849 * t19854 * t14482 / 144.0 + t19849 * t19850 * t14487 / 216.0 + t70427 / 864.0 - t64477 - t64478 / 648.0 + t64483 + t64487 + t18069 * t15032 / 1152.0;
    (t70432,)
}
