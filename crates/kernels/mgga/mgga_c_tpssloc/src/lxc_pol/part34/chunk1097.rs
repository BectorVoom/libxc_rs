//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1097/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1097<F: Float>(t5: F, t108708: F, t108727: F, t108743: F, t108763: F, t112: F, t105159: F, t105201: F, t106902: F, t107504: F, t19596: F, t1983: F, t20085: F, t20296: F, t2036: F, t2075: F, t2095: F, t22425: F, t22574: F, t24432: F, t26558: F, t28030: F, t28969: F, t29205: F, t29211: F, t29222: F, t29247: F, t29377: F, t29380: F, t4028: F, t510: F, t5161: F, t5450: F, t5493: F, t652: F, t7170: F, t74064: F, t7458: F, t7685: F, t7802: F, t7890: F, t7940: F, t91655: F) -> (F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t108766 = piecewise3(t8, 0.0, t108708 + t108727 + t108743 + t108763);
    let t108767 = t108766 * t112;
    let t108780 = -9.0 * t22574 * t24432 * t107504 + 6.0 * t1983 * t7940 * t20085 + 9.0 * t7685 * t28969 - 12.0 * t4028 * t29205 - 6.0 * t28030 * t7802 - 6.0 * t7458 * t29211 - 3.0 * t7685 * t29222 - 9.0 * t22574 * t24432 * t106902 - 6.0 * t1983 * t2095 * t74064 + 3.0 * t1983 * t7170 * t105159 - 18.0 * t91655 * t29247 + 18.0 * t22574 * t26558 * t105201 - 6.0 * t652 * t7890 * t5493 - 6.0 * t20296 * t2075 - t108767 * t510 - 3.0 * t5450 * t7890 - t2036 * t22425 - 3.0 * t1983 * t7940 * t19596 + 18.0 * t7685 * t29380 - 3.0 * t1983 * t29377 * t5161;
    (t108767, t108780)
}
