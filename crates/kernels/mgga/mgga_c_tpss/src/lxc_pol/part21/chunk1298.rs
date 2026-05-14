//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1298/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1298<F: Float>(t11008: F, t11022: F, t11031: F, t11035: F, t11468: F, t11497: F, t11598: F, t11663: F, t18069: F, t18098: F, t19849: F, t19850: F, t19854: F, t36839: F, t5620: F, t61322: F, t61324: F, t61329: F) -> (F,) {
    let t64386 = t19849 * t19850 * t11022 / 216.0 + 7.0 / 648.0 * t19849 * t36839 * t11008 - t19849 * t19854 * t11031 / 72.0 - t19849 * t19854 * t11035 / 144.0 - t61322 / 3456.0 + t61324 / 3456.0 + 5.0 / 3456.0 * t18069 * t11598 + t61329 - t18098 * t11497 / 1536.0 + 5.0 / 6912.0 * t5620 * t11468 + 5.0 / 2592.0 * t5620 * t11663;
    (t64386,)
}
