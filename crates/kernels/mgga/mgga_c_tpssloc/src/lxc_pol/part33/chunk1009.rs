//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1009/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1009<F: Float>(t23030: F, t6643: F, t131: F, t244: F, t209: F, t1878: F, t6612: F, t835: F, t812: F, t2627: F, t59: F, t240: F, t2230: F, t6589: F, t213: F, t229: F, t6546: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23031 = t23030 * t6643;
    let t23032 = 0.26044789391763585244e-1 * t23031;
    let t23033 = t244 * t131;
    let t23034 = t23033 * t209;
    let t23035 = t1878 * t23034;
    let t23040 = t6612 * t835;
    let t23041 = t812 * t23040;
    let t23046 = t2627 * t59;
    let t23047 = t23046 * t240;
    let t23048 = t812 * t23047;
    let t23056 = t1878 * t244;
    let t23061 = t2230 * t6589;
    let t23062 = t23061 * t213;
    let t23069 = t6546 * t229;
    (t23032, t23033, t23034, t23035, t23040, t23041, t23046, t23047, t23048, t23056, t23061, t23062, t23069)
}
