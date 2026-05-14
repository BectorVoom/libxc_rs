//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 751/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk751<F: Float>(t29314: F, t29375: F, t533: F, t1390: F, t26905: F, t7687: F, t19451: F, t1983: F, t2036: F, t2040: F, t2079: F, t22574: F, t28002: F, t28030: F, t29211: F, t29214: F, t29219: F, t29222: F, t29241: F, t29243: F, t29247: F, t29252: F, t4028: F, t574: F, t6287: F, t6468: F, t652: F, t7458: F, t7685: F, t7796: F, t7802: F, t7904: F, t7943: F) -> (F, F, F, F) {
    let t29376 = t29314 + t29375;
    let t29377 = t533 * t29376;
    let t29378 = t29377 * t1390;
    let t29380 = t26905 * t7687;
    let t29394 = -2.0 * t19451 * t2040 - t1983 * t29222 + 2.0 * t1983 * t29243 + 6.0 * t1983 * t29252 + t1983 * t29378 + 6.0 * t1983 * t29380 - t2036 * t6287 - 4.0 * t2040 * t28002 - 2.0 * t2040 * t28030 + t2079 * t6468 - 6.0 * t22574 * t29247 - 2.0 * t29211 * t652 - 2.0 * t29214 * t652 - 4.0 * t29219 * t652 + t29241 * t574 - 4.0 * t4028 * t7796 - 4.0 * t4028 * t7802 - 4.0 * t7458 * t7796 + 6.0 * t7685 * t7904 - 2.0 * t7685 * t7943;
    (t29377, t29378, t29380, t29394)
}
