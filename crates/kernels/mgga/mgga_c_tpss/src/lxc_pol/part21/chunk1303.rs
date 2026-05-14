//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1303/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1303<F: Float>(t11640: F, t5605: F, t11645: F, t11588: F, t18069: F, t11513: F, t1461: F, t18110: F, t3928: F, t3945: F, t61390: F, t61422: F, t61425: F, t61437: F, t61439: F, t61442: F, t64477: F) -> (F,) {
    let t64478 = t5605 * t11640;
    let t64483 = t5605 * t11645 / 432.0;
    let t64487 = t18069 * t11588 / 1728.0;
    let t64490 = 11.0 / 324.0 * t61390 * t1461 + t61425 / 1728.0 + 19.0 / 1296.0 * t61437 + t61439 / 648.0 + t61442 - t64477 - t64478 / 1296.0 - t18110 * t3928 / 54.0 + t64483 + t5605 * t11513 / 288.0 + t64487 - t61422 * t3945 / 216.0;
    (t64490,)
}
