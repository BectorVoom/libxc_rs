//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1321/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1321<F: Float>(t14991: F, t18069: F, t14457: F, t14464: F, t14469: F, t14473: F, t14477: F, t14908: F, t14913: F, t15040: F, t15044: F, t15089: F, t15112: F, t19849: F, t19850: F, t19854: F, t36839: F, t5620: F, t61387: F, t61395: F, t61432: F, t61449: F, t64420: F, t64427: F, t64430: F, t64433: F, t64436: F, t64447: F, t64455: F) -> (F,) {
    let t70397 = t18069 * t14991;
    let t70399 = -t64420 / 3456.0 - t64427 + t64430 - t64433 / 5184.0 + t64436 + t5620 * t14908 / 384.0 - t5620 * t14913 / 576.0 - t61395 / 1296.0 + t64447 + t61432 * t15112 / 1152.0 + t18069 * t15040 / 2304.0 - t19849 * t19850 * t14457 / 36.0 + t19849 * t19850 * t14464 / 108.0 + 7.0 / 648.0 * t19849 * t36839 * t14469 + t19849 * t19854 * t14473 / 48.0 - t19849 * t19854 * t14477 / 72.0 + t64455 - t61449 * t15044 / 2304.0 + t61387 * t15089 / 256.0 + t70397 / 1728.0;
    (t70399,)
}
