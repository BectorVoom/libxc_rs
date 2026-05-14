//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 813/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk813<F: Float>(t26193: F, t8621: F, t1985: F, t225: F, t567: F, t7918: F, t214: F, t1842: F, t31558: F, t22635: F, t1992: F, t1799: F, t31549: F, t22633: F, t31618: F, t6637: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33240 = t26193 * t8621;
    let t33241 = t1985 * t33240;
    let t33245 = t7918 * t225 * t567;
    let t33246 = t214 * t33245;
    let t33247 = t1985 * t33246;
    let t33249 = t31558 * t1842;
    let t33250 = t22635 * t33249;
    let t33251 = t1992 * t33250;
    let t33272 = t31549 * t1799;
    let t33273 = t22635 * t33272;
    let t33274 = t22633 * t33273;
    let t33276 = t31618 * t1799;
    let t33277 = t6637 * t33276;
    (t33240, t33241, t33245, t33246, t33247, t33249, t33250, t33251, t33272, t33273, t33274, t33276, t33277)
}
