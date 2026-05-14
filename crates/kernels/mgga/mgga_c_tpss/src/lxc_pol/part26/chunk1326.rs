//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1326/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1326<F: Float>(t18246: F, t52639: F, t69810: F, t20047: F, t69847: F, t14245: F, t20011: F, t14029: F, t33: F, t52613: F, t1692: F, t1713: F, t17929: F, t18047: F, t19670: F, t19798: F, t19802: F, t20050: F, t21486: F, t21492: F, t21516: F, t2439: F, t36547: F, t61269: F, t6207: F, t6214: F, t64277: F, t69857: F, t69870: F) -> (F,) {
    let t70887 = t18246 * t52639;
    let t70890 = t18246 * t69810;
    let t70893 = t20047 * t69847;
    let t70906 = t20011 * t14245;
    let t70909 = t33 * t14029;
    let t70915 = t18246 * t52613;
    let t70920 = -3.0 * t17929 * t70887 - 3.0 * t17929 * t70890 + t69857 + 3.0 * t17929 * t70893 - t1692 * t18047 * t21516 / 2.0 + 3.0 * t2439 * t19798 * t6207 + 3.0 * t36547 * t21486 - 3.0 * t61269 * t21492 - t69870 + 6.0 * t19670 * t70906 + 3.0 / 2.0 * t2439 * t1713 * t70909 - t1692 * t64277 * t6214 - 3.0 / 2.0 * t17929 * t70915 - t1692 * t19802 * t20050;
    (t70920,)
}
