//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1301/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1301<F: Float>(t11548: F, t5620: F, t11691: F, t11701: F, t5610: F, t11584: F, t18069: F, t11013: F, t11026: F, t11552: F, t19849: F, t19850: F, t19854: F, t3963: F, t61377: F, t61393: F, t61395: F, t61422: F) -> (F,) {
    let t64430 = t5620 * t11548 / 1728.0;
    let t64433 = t5620 * t11691;
    let t64436 = t5610 * t11701 / 1152.0;
    let t64447 = t18069 * t11584 / 1728.0;
    let t64450 = t64430 + t5620 * t11552 / 2304.0 - t64433 / 10368.0 + t64436 + t61377 / 648.0 - t19849 * t19850 * t11013 / 36.0 + t19849 * t19854 * t11026 / 48.0 - t61393 / 162.0 - t61395 / 648.0 + t64447 - t61422 * t3963 / 216.0;
    (t64450,)
}
