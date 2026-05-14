//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1261/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1261<F: Float>(t71259: F, t71303: F, t71343: F, t71878: F, t72593: F, t72637: F, t72682: F, t72721: F, t1279: F, t1338: F, t13546: F, t16064: F, t16073: F, t1668: F, t1851: F, t19040: F, t20682: F, t20690: F, t21972: F, t21978: F, t25315: F, t3537: F, t4559: F, t4674: F, t547: F, t5470: F, t5474: F, t5477: F, t548: F, t5947: F, t5953: F, t5954: F, t5957: F, t6446: F, t645: F, t67816: F, t71184: F, t71212: F) -> (F, F) {
    let t72724 = t71259 + t71303 + t71343 + t71878 + t72593 + t72637 + t72682 + t72721;
    let t72733 = 6.0 * t5947 * t5474 + 12.0 * t547 * t71184 * t1338 + 12.0 * t547 * t67816 * t1338 + 12.0 * t547 * t20690 * t3537 + 6.0 * t1279 * t21972 + 3.0 * t5947 * t5477 + 6.0 * t6446 * t4559 + 6.0 * t1851 * t16064 + 3.0 * t5470 * t5957 + 6.0 * t547 * t19040 * t4674 + 6.0 * t547 * t5953 * t13546 + 6.0 * t1851 * t16073 + 6.0 * t547 * t71212 * t645 + 12.0 * t547 * t25315 * t3537 + param_d * t72724 * t548 + 12.0 * t1668 * t20682 + 6.0 * t1279 * t21978 + 6.0 * t5470 * t5954;
    (t72724, t72733)
}
