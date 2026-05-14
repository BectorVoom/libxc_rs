//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1277/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1277<F: Float>(t64056: F, t64109: F, t64174: F, t64234: F, t823: F, t10667: F, t19671: F, t30: F, t31814: F, t1398: F, t2433: F, t1692: F, t1713: F, t17921: F, t17929: F, t17934: F, t18047: F, t19670: F, t19685: F, t19798: F, t19816: F, t19825: F, t1991: F, t2439: F, t3552: F, t5539: F, t5586: F, t6149: F, t63860: F, t63864: F, t63873: F, t63877: F, t63881: F, t63885: F) -> (F, F, F, F) {
    let t64236 = t64056 + t64109 + t64174 + t64234;
    let t64237 = t64236 * t823;
    let t64241 = t19671 * t10667;
    let t64247 = t31814 * t30;
    let t64248 = t1398 * t2433;
    let t64249 = t64247 * t64248;
    let t64255 = -t1692 * t18047 * t19825 - 6.0 * t19670 * t63860 - 3.0 * t19670 * t63864 + 3.0 * t2439 * t6149 * t17934 + 3.0 * t3552 * t6149 * t17921 + 3.0 / 2.0 * t2439 * t1713 * t63873 + 3.0 * t2439 * t1713 * t63877 - 3.0 * t17929 * t63881 + 6.0 * t17929 * t63885 + 3.0 * t2439 * t19798 * t5539 + t1692 * t64237 * t30 / 2.0 + 3.0 * t19670 * t64241 + 3.0 * t2439 * t5586 * t19685 - 3.0 * t19816 * t64249 + t1692 * t6149 * t1991 / 2.0;
    (t64236, t64237, t64248, t64255)
}
