//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 782/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk782<F: Float>(t31668: F, t533: F, t1390: F, t1983: F, t1873: F, t23938: F, t26977: F, t6534: F, t7042: F, t2039: F, t31537: F, t88: F, t7056: F, t8601: F, t650: F, t8595: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31669 = t533 * t31668;
    let t31670 = t31669 * t1390;
    let t31671 = t1983 * t31670;
    let t31704 = 2.0 * t23938 * t1873;
    let t31706 = 2.0 * t26977 * t1873;
    let t31708 = 2.0 * t7042 * t6534;
    let t31716 = 2.0 * t31537 * t2039;
    let t31717 = t88 * t6534;
    let t31719 = 2.0 * t31717 * t2039;
    let t31721 = 2.0 * t8601 * t7056;
    let t31733 = t650 * t8595;
    (t31669, t31670, t31671, t31704, t31706, t31708, t31716, t31717, t31719, t31721, t31733)
}
