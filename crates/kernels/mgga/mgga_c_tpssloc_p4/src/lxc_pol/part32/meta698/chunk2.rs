//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2177/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2177<F: Float>(t26318: F, t7708: F, t91351: F, t19844: F, t6916: F, t22804: F, t28077: F, t22779: F, t28067: F, t1361: F, t19924: F, t26288: F) -> (F, F, F, F, F) {
    let t97435 = t91351 * t7708 * t26318;
    let t97437 = t6916 * t19844;
    let t97439 = t22804 * t28077;
    let t97444 = t22779 * t28067;
    let t97447 = t26288 * t1361 * t19924;
    (t97435, t97437, t97439, t97444, t97447)
}
