//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1296/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1296<F: Float>(t63810: F, t63855: F, t64255: F, t64308: F, t11454: F, t18098: F, t11572: F, t11594: F, t11609: F, t11637: F, t11671: F, t18069: F, t18083: F, t18094: F, t3935: F, t3956: F, t3974: F, t3979: F, t5620: F, t61308: F, t61344: F, t61372: F, t61449: F) -> (F, F) {
    let t64310 = t63810 + t63855 + t64255 + t64308;
    let t64325 = t18098 * t11454 / 1152.0;
    let t64334 = -t61308 / 2304.0 + t61344 * t3956 / 144.0 + t18083 * t3979 / 108.0 - 5.0 / 648.0 * t18083 * t3974 + t18094 * t11637 / 384.0 + t18094 * t11609 / 768.0 - t64325 - t5620 * t11671 / 1152.0 - t61372 * t3935 / 72.0 - t61449 * t11572 / 1152.0 - t18069 * t11594 / 576.0;
    (t64310, t64334)
}
