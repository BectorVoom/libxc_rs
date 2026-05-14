//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1297/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1297<F: Float>(t18083: F, t3969: F, t1695: F, t5599: F, t139: F, t19849: F, t3754: F, t928: F, t2697: F, t3749: F, t11018: F, t11439: F, t11579: F, t1471: F, t18107: F, t19850: F, t19851: F, t19855: F, t3952: F, t5610: F, t61318: F, t61406: F, t61432: F) -> (F,) {
    let t64343 = t18083 * t3969 / 324.0;
    let t64346 = t5599 * t1695;
    let t64354 = t19849 * t139 * t928 * t3754 / 216.0;
    let t64358 = t19849 * t139 * t2697 * t3749 / 324.0;
    let t64362 = -t61318 / 216.0 + t5610 * t11439 / 1536.0 - t18107 * t3952 / 144.0 + 19.0 / 1296.0 * t61406 * t1471 - t64343 + t61432 * t11579 / 576.0 + t64346 * t19855 / 27.0 - 2.0 / 81.0 * t64346 * t19851 - t64354 + t64358 + t19849 * t19850 * t11018 / 108.0;
    (t64362,)
}
