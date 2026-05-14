//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1314/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1314<F: Float>(t69943: F, t69970: F, t69988: F, t70004: F, t1395: F, t226: F, t3664: F, t1378: F, t14297: F, t1702: F, t17984: F, t17993: F, t18000: F, t18006: F, t18007: F, t18021: F, t19724: F, t19727: F, t19736: F, t19754: F, t19758: F, t19768: F, t19769: F, t19781: F, t21290: F, t21316: F, t21326: F, t21336: F, t2162: F, t253: F, t3699: F, t3721: F, t4758: F, t4800: F, t5562: F, t5571: F, t5572: F, t5574: F, t5577: F, t6130: F, t64028: F, t69897: F, t69912: F, t782: F, t818: F) -> (F, F) {
    let t70006 = t69943 + t69970 + t69988 + t70004;
    let t70030 = t1395 * t3664 * t226;
    let t70038 = -2.0 * t17993 * t21326 - 2.0 * t5571 * t18021 * t69897 * t2162 + 2.0 * t5571 * t5577 * t19724 * t1378 * t226 + 2.0 * t5571 * t5577 * t6130 * t3664 * t226 + 2.0 * t69912 * t5574 + 4.0 * t5571 * t5572 * t6130 * t3721 - t17984 * t4800 + 4.0 * t19736 * t19754 + 8.0 * t18006 * t19768 * t1395 * t19769 + param_beta * t70006 * t253 + 4.0 * t19736 * t19758 + 4.0 * t19727 * t3699 - 4.0 * t18006 * t64028 * t19781 - 12.0 * t5571 * t18000 * t21316 * t818 + t17993 * t21336 + t5571 * t5577 * t5562 * t4758 * t226 + t5571 * t5577 * t1702 * t14297 * t226 - 4.0 * t18006 * t18007 * t70030 + t5571 * t5577 * t21290 * t782 * t226;
    (t70006, t70038)
}
