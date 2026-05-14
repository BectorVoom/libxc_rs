//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1276/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1276<F: Float>(t10584: F, t2364: F, t1395: F, t226: F, t3721: F, t782: F, t36075: F, t10764: F, t10894: F, t1702: F, t17993: F, t18000: F, t18006: F, t18007: F, t18021: F, t18024: F, t19734: F, t19736: F, t19748: F, t19749: F, t19753: F, t19758: F, t19767: F, t19768: F, t19775: F, t19779: F, t19786: F, t2162: F, t2407: F, t5571: F, t5572: F, t5574: F, t5577: F, t5583: F, t61183: F, t61195: F, t61222: F, t61232: F, t6137: F, t6138: F, t6143: F, t64077: F, t64135: F, t818: F) -> (F,) {
    let t64183 = t10584 * t2364;
    let t64190 = t1395 * t2364 * t226;
    let t64198 = t3721 * t782 * t226;
    let t64204 = t36075 * t226;
    let t64234 = 2.0 * t17993 * t19775 + 2.0 * t17993 * t19779 + 4.0 * t64135 * t5574 + 2.0 * t61183 * t6138 - 2.0 * t19767 * t19768 * t64183 + 4.0 * t17993 * t19758 - 2.0 * t18006 * t18007 * t64190 - 4.0 * t18006 * t61232 * t19748 - 4.0 * t18006 * t18007 * t64198 - 4.0 * t61222 * t19749 + t19767 * t18007 * t64204 - 2.0 * t19734 * t5583 + t5571 * t5577 * t1702 * t10764 * t226 + 4.0 * t17993 * t19786 - 2.0 * t5571 * t18021 * t64077 * t2162 + 24.0 * t5571 * t61195 * t6137 * t2407 - 2.0 * t19736 * t18024 - 12.0 * t5571 * t18000 * t19753 * t818 + t61183 * t6143 + 2.0 * t5571 * t5572 * t1702 * t10894;
    (t64234,)
}
