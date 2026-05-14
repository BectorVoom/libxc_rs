//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1075/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1075<F: Float>(t926: F, t9637: F, t11878: F, t140: F, t3032: F, t4047: F, t1098: F, t1100: F, t4052: F, t11888: F, t4219: F, t11894: F, t11883: F, t11902: F, t4223: F, t11906: F) -> (F, F, F, F, F, F, F, F) {
    let t12278 = t926 * t9637;
    let t12279 = t12278 * t11878;
    let t12287 = t140 * t3032;
    let t12288 = t12287 * t4047;
    let t12290 = t1098 * t12288 / 324.0;
    let t12291 = t140 * t1100;
    let t12292 = t12291 * t4052;
    let t12294 = t1098 * t12292 / 216.0;
    let t12295 = t4219 * t11888;
    let t12298 = t4219 * t11894;
    let t12301 = t4219 * t11883;
    let t12304 = t4223 * t11902;
    let t12307 = t4223 * t11906;
    (t12279, t12290, t12294, t12295, t12298, t12301, t12304, t12307)
}
