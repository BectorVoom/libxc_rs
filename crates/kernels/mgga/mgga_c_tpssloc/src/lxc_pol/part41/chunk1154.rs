//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1154/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1154<F: Float>(t1453: F, t662: F, t8184: F, t4067: F, t8180: F, t626: F, t8266: F, t104: F, t50: F, t666: F, t103: F) -> (F, F, F, F, F, F, F) {
    let t30284 = t1453 * t662;
    let t30285 = t8184 * t30284;
    let t30288 = t8180 * t4067;
    let t30291 = t626 * t8266;
    let t30293 = t50 * t104;
    let t30294 = t30293 * t666;
    let t30297 = t50 * t103;
    (t30284, t30285, t30288, t30291, t30293, t30294, t30297)
}
