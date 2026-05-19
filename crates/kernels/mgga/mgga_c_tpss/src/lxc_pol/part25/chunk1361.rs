//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1361/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1361<F: Float>(t72044: F, t72057: F, t72069: F, t72077: F, t4715: F, t5831: F, t1395: F, t14423: F, t17993: F, t18000: F, t18006: F, t18021: F, t1805: F, t19736: F, t19762: F, t19769: F, t20449: F, t20482: F, t20503: F, t21608: F, t2162: F, t21623: F, t21624: F, t21630: F, t21634: F, t21635: F, t21645: F, t21653: F, t226: F, t253: F, t3699: F, t3721: F, t5571: F, t5572: F, t5577: F, t5843: F, t61195: F, t61226: F, t6342: F, t6343: F, t6348: F, t64135: F, t66480: F, t69912: F, t782: F, t818: F, param_beta: F) -> (F, F) {
    let t72079 = t72044 + t72057 + t72069 + t72077;
    let t72111 = t5831 * t4715;
    let t72129 = F::new(24.0) * t5571 * t61195 * t21623 * t818 + F::new(2.0) * t5571 * t5572 * t1805 * t14423 + param_beta * t72079 * t253 + F::new(2.0) * t17993 * t21635 - F::new(6.0) * t5571 * t18000 * t21634 * t818 + F::new(2.0) * t64135 * t6348 - F::new(6.0) * t17993 * t21624 + F::new(8.0) * t18006 * t20482 * t1395 * t19769 - F::new(12.0) * t5571 * t18000 * t21630 * t818 - F::new(12.0) * t5571 * t18000 * t6342 * t3721 + F::new(2.0) * t19736 * t20503 + t5571 * t5577 * t21608 * t782 * t226 + t17993 * t21653 + t5571 * t5577 * t72111 * t226 - F::new(2.0) * t5571 * t18021 * t72111 * t2162 + F::new(2.0) * t17993 * t21645 + F::new(4.0) * t64135 * t6343 + F::new(12.0) * t61226 * t66480 * t19762 + t69912 * t5843 + F::new(4.0) * t20449 * t3699;
    (t72079, t72129)
}
