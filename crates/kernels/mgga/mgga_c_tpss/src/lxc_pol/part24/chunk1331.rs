//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1331/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1331<F: Float>(t20047: F, t70240: F, t69881: F, t1006: F, t4806: F, t18246: F, t69863: F, t4802: F, t64879: F, t70243: F, t1692: F, t17929: F, t18047: F, t18052: F, t19670: F, t19802: F, t19816: F, t20041: F, t20048: F, t20058: F, t20065: F, t21510: F, t21513: F, t2439: F, t5590: F, t5678: F, t60996: F, t6149: F, t64284: F, t69851: F, t70247: F, t70800: F) -> (F,) {
    let t70803 = t20047 * t70240;
    let t70805 = t20047 * t69881;
    let t70808 = t1006 * t4806;
    let t70813 = t18246 * t69863;
    let t70816 = t1006 * t4802;
    let t70828 = t64879 * t70243;
    let t70835 = 2.0 * t69851 * t20048 - 6.0 * t19670 * t70800 + t19816 * t70803 + 2.0 * t19816 * t70805 + t1692 * t18052 * t70808 - t1692 * t18047 * t21513 - 3.0 / 2.0 * t17929 * t70813 - t1692 * t5590 * t70816 / 2.0 - 3.0 * t64284 * t20041 - t1692 * t70247 * t5678 / 2.0 + 3.0 * t2439 * t6149 * t20058 - 3.0 * t19816 * t70828 - t1692 * t19802 * t20065 + t1692 * t60996 * t21510;
    (t70835,)
}
