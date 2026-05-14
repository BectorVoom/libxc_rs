//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1274/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1274<F: Float>(t18005: F, t6134: F, t2162: F, t64007: F, t2161: F, t6130: F, t10837: F, t1395: F, t1707: F, t1708: F, t17981: F, t17984: F, t17991: F, t17993: F, t18000: F, t18002: F, t18006: F, t18010: F, t18014: F, t18018: F, t18029: F, t18034: F, t19727: F, t19736: F, t19754: F, t19757: F, t19762: F, t19767: F, t19768: F, t19781: F, t226: F, t228: F, t2426: F, t3664: F, t3699: F, t3722: F, t5562: F, t5565: F, t5571: F, t5572: F, t5577: F, t61232: F, t6146: F, t64002: F, t818: F) -> (F, F) {
    let t64060 = t6134 * t18005;
    let t64063 = t64007 * t2162;
    let t64077 = t6130 * t2161;
    let t64109 = 2.0 * t19767 * t61232 * t19781 - 4.0 * t64060 * t18010 + 4.0 * t18006 * t19768 * t64063 - 4.0 * t18006 * t61232 * t19762 + 4.0 * t19736 * t18014 + 4.0 * t17993 * t19754 - t19727 * t2426 + 4.0 * t5565 * t10837 + t5571 * t5577 * t64077 * t226 + 2.0 * t19736 * t18029 + t19736 * t18034 + 2.0 * t5571 * t5572 * t17981 * t1395 + 2.0 * t19736 * t18018 - 2.0 * t17984 * t3722 - t17991 * t6146 + 2.0 * t5571 * t5577 * t5562 * t3664 * t226 + 4.0 * t17984 * t3699 - t1707 * t1708 * t228 * t64002 - 6.0 * t19736 * t18002 - 12.0 * t5571 * t18000 * t19757 * t818;
    (t64077, t64109)
}
