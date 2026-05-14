//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1332/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1332<F: Float>(t1561: F, t19150: F, t1586: F, t19144: F, t20862: F, t5570: F, t1883: F, t8547: F, t28778: F, t3048: F, t1880: F, t2785: F, t4245: F, t6032: F, t1113: F, t1148: F, t12580: F, t12590: F, t12614: F, t12618: F, t19115: F, t19129: F, t19139: F, t19143: F, t19146: F, t19155: F, t19156: F, t20865: F, t20868: F, t20874: F, t20891: F, t20892: F, t20896: F, t20897: F, t20906: F, t20913: F, t3054: F, t4303: F, t4314: F, t6027: F, t6031: F, t63200: F, t63357: F, t63371: F, t63383: F, t63396: F, t9759: F) -> (F, F, F) {
    let t68263 = t19150 * t1561;
    let t68269 = t19144 * t1586;
    let t68273 = t20862 * t5570;
    let t68276 = t1883 * t8547;
    let t68278 = t68276 * t28778 * t3048;
    let t68280 = t1880 * t1561 * t2785;
    let t68290 = t6032 * t4245;
    let t68308 = -t20868 * t19156 - 4.0 * t19143 * t63396 * t1561 * t4303 - 4.0 * t19143 * t19144 * t4245 * t4303 - 2.0 * t19143 * t20896 * t9759 - 6.0 * t63200 * t63383 * t1561 * t12580 + 6.0 * t63200 * t20896 * t12590 + 2.0 * t19143 * t68263 * t4314 - 12.0 * t19115 * t20874 + 4.0 * t63357 * t68269 * t19146 + 4.0 * t68273 * t6027 + 8.0 * t68278 * t68280 * t3054 * t1148 * t1113 + 2.0 * t20865 * t19139 - t6031 * t20913 * t19155 + 2.0 * t19143 * t68290 * t4314 + t19143 * t20891 * t12614 - t63200 * t20891 * t12618 - 4.0 * t63371 * t20897 + 2.0 * t63371 * t20906 + 4.0 * t19129 * t68263 * t20892 + 4.0 * t19129 * t68290 * t20892;
    (t68276, t68280, t68308)
}
