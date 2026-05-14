//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 651/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk651<F: Float>(t6612: F, t835: F, t812: F, t2627: F, t59: F, t240: F, t1878: F, t244: F, t2230: F, t6589: F, t213: F, t229: F, t6546: F, t243: F, t598: F, t6584: F, t6604: F) -> (F, F, F, F, F, F, F, F) {
    let t23040 = t6612 * t835;
    let t23041 = t812 * t23040;
    let t23046 = t2627 * t59;
    let t23047 = t23046 * t240;
    let t23048 = t812 * t23047;
    let t23056 = t1878 * t244;
    let t23061 = t2230 * t6589;
    let t23062 = t23061 * t213;
    let t23069 = t6546 * t229;
    let t23075 = t243 * t243;
    let t23076 = 1.0 / t23075;
    let t23077 = t598 * t23076;
    let t23078 = t23077 * t213;
    let t23083 = t6584 * t6604;
    (t23041, t23046, t23048, t23056, t23062, t23069, t23078, t23083)
}
