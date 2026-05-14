//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1275/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1275<F: Float>(t3665: F, t818: F, t1379: F, t2425: F, t19733: F, t5570: F, t6130: F, t768: F, t1702: F, t8275: F, t2161: F, t35764: F, t10584: F, t10895: F, t1378: F, t1396: F, t17981: F, t17993: F, t18000: F, t18006: F, t18007: F, t18009: F, t18037: F, t18040: F, t19724: F, t19736: F, t19744: F, t19767: F, t19768: F, t19770: F, t19791: F, t226: F, t2364: F, t2407: F, t3721: F, t5562: F, t5565: F, t5571: F, t5572: F, t5577: F, t5580: F, t61190: F, t6135: F, t6137: F, t64034: F, t782: F) -> (F, F) {
    let t64118 = t3665 * t818;
    let t64122 = t1379 * t2425;
    let t64135 = t19733 * t5570;
    let t64159 = t768 * t6130;
    let t64163 = t8275 * t1702;
    let t64164 = t35764 * t2161;
    let t64168 = t10584 * t2161;
    let t64174 = 2.0 * t17993 * t19791 + t5571 * t5577 * t17981 * t1378 * t226 - t61190 * t1396 - 4.0 * t18006 * t18007 * t64118 - 2.0 * t18006 * t18007 * t64122 - t6135 * t18040 - 6.0 * t5571 * t18000 * t6137 * t2425 + 4.0 * t5571 * t5572 * t5562 * t3721 + 2.0 * t64135 * t5580 - 12.0 * t17993 * t19744 - t5565 * t10895 - 6.0 * t5571 * t18000 * t6130 * t2407 + 4.0 * t5571 * t5572 * t19724 * t818 + t19736 * t18037 + 2.0 * t5571 * t5577 * t19724 * t782 * t226 + t5571 * t5577 * t6130 * t2364 * t226 - 4.0 * t18006 * t64159 * t18009 + 6.0 * t19767 * t64163 * t64164 - 6.0 * t19767 * t19768 * t64168 - 4.0 * t64034 * t19770;
    (t64135, t64174)
}
