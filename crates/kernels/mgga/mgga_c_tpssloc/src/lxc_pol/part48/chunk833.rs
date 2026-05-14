//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 833/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk833<F: Float>(t12739: F, t8326: F, t1388: F, t6995: F, t31283: F, t16535: F, t2363: F, t3941: F, t12524: F, t31285: F, t12521: F, t31286: F, t23893: F, t24465: F, t23896: F, t55571: F, t8657: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t114415 = 2.0 * t12739 * t8326;
    let t114422 = t1388 * t6995;
    let t114456 = 27.0 * t31283;
    let t114472 = 27.0 * t16535 * t8326;
    let t114483 = 27.0 * t3941 * t8326 * t2363;
    let t114489 = 54.0 * t12524 * t31285;
    let t114494 = 0.135e2 * t12521 * t8326;
    let t114500 = 54.0 * t31286;
    let t114513 = 54.0 * t24465 * t23893;
    let t114515 = 27.0 * t24465 * t23896;
    let t114517 = 27.0 * t55571 * t8657;
    (t114415, t114422, t114456, t114472, t114483, t114489, t114494, t114500, t114513, t114515, t114517)
}
