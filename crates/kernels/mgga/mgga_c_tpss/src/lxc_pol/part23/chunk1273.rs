//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1273/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1273<F: Float>(t1395: F, t2161: F, t226: F, t19725: F, t219: F, t18007: F, t19766: F, t5567: F, t36098: F, t1379: F, t2407: F, t10818: F, t1705: F, t935: F, t10833: F, t10841: F, t1378: F, t1710: F, t18006: F, t18009: F, t19727: F, t19763: F, t19767: F, t19768: F, t19769: F, t19782: F, t19794: F, t2157: F, t2408: F, t2425: F, t253: F, t44584: F, t44610: F, t5562: F, t5565: F, t5568: F, t5571: F, t5572: F, t61222: F, t61226: F, t6130: F, t63893: F, t64002: F, t819: F) -> (F, F, F) {
    let t64007 = t1395 * t2161;
    let t64008 = t64007 * t226;
    let t64016 = t19725 * t219;
    let t64028 = t18007 * t1395;
    let t64034 = t5567 * t19766;
    let t64039 = t36098 * t226;
    let t64042 = t1379 * t2407;
    let t64050 = t1705 * t10818 * t935;
    let t64056 = 8.0 * t18006 * t19768 * t1378 * t63893 + 2.0 * t19727 * t2408 + param_beta * t64002 * t253 + 2.0 * t5565 * t10841 - 2.0 * t18006 * t18007 * t64008 + 2.0 * t5571 * t5572 * t6130 * t2425 - 2.0 * t64016 * t819 - 4.0 * t19767 * t2157 * t5562 * t19769 - 4.0 * t19767 * t19768 * t44584 - 2.0 * t5568 * t19794 + 12.0 * t61226 * t64028 * t18009 - 4.0 * t61222 * t19763 + 2.0 * t64034 * t19782 - 6.0 * t5565 * t10833 + t19767 * t18007 * t64039 + 6.0 * t61226 * t18007 * t64042 + 2.0 * t19767 * t18007 * t44610 - t64050 * t1710 - 4.0 * t18006 * t18007 * t1378 * t18009;
    (t64007, t64034, t64056)
}
