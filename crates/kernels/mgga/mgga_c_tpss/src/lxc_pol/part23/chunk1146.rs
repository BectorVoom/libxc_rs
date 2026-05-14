//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1146/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1146<F: Float>(t2433: F, t30: F, t580: F, t821: F, t2428: F, t1692: F, t1713: F, t17921: F, t17929: F, t17931: F, t17934: F, t17938: F, t18043: F, t18047: F, t18052: F, t1991: F, t2439: F, t3552: F, t5539: F, t5586: F, t5590: F, t5591: F) -> (F, F, F, F) {
    let t18053 = t30 * t2433;
    let t18056 = t580 * t821;
    let t18059 = t30 * t2428;
    let t18066 = 3.0 * t3552 * t1713 * t17921 + 3.0 * t2439 * t5586 * t5539 - 3.0 * t17929 * t17931 + 3.0 * t2439 * t1713 * t17934 + 3.0 / 2.0 * t2439 * t1713 * t17938 + t1692 * t18043 * t30 / 2.0 - t1692 * t18047 * t5591 + t1692 * t5586 * t580 + t1692 * t18052 * t18053 - t1692 * t5590 * t18056 - t1692 * t5590 * t18059 / 2.0 + t1692 * t1713 * t1991 / 2.0;
    (t18053, t18056, t18059, t18066)
}
