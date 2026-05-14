//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1098/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1098<F: Float>(t102386: F, t106731: F, t106935: F, t106956: F, t107634: F, t108004: F, t108767: F, t1268: F, t1458: F, t19451: F, t20347: F, t2039: F, t27188: F, t28002: F, t28007: F, t28951: F, t33234: F, t4028: F, t5493: F, t67001: F, t7042: F, t7676: F, t7801: F) -> (F,) {
    let t108844 = 6.0 * t102386 * t1458 + 6.0 * t106731 * t2039 + 2.0 * t106935 * t2039 + 6.0 * t106956 * t2039 + 2.0 * t107634 * t1268 + 6.0 * t19451 * t7801 + 2.0 * t20347 * t7042 + 2.0 * t2039 * t67001 + 6.0 * t27188 * t5493 + 12.0 * t28002 * t7801 + 6.0 * t28007 * t7801 + 6.0 * t28951 * t4028 + 6.0 * t28951 * t7676 + 6.0 * t33234 * t5493 + 6.0 * t108004 + t108767;
    (t108844,)
}
