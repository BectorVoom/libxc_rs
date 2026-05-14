//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1197/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1197<F: Float>(t2161: F, t35764: F, t10584: F, t2364: F, t1395: F, t226: F, t3721: F, t782: F, t36075: F, t10667: F, t19671: F, t30: F, t31814: F, t1398: F, t2433: F, t17930: F, t35525: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t64164 = t35764 * t2161;
    let t64168 = t10584 * t2161;
    let t64183 = t10584 * t2364;
    let t64190 = t1395 * t2364 * t226;
    let t64198 = t3721 * t782 * t226;
    let t64204 = t36075 * t226;
    let t64241 = t19671 * t10667;
    let t64247 = t31814 * t30;
    let t64248 = t1398 * t2433;
    let t64249 = t64247 * t64248;
    let t64256 = t17930 * t35525;
    (t64164, t64168, t64183, t64190, t64198, t64204, t64241, t64248, t64249, t64256)
}
