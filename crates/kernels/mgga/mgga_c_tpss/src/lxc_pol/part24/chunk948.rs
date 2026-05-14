//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 948/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk948<F: Float>(t1523: F, t2954: F, t926: F, t9637: F, t140: F, t3032: F, t4047: F, t1098: F, t1100: F, t4052: F, t4241: F, t9561: F, t3067: F, t242: F, t3090: F, t4056: F) -> (F, F, F, F, F, F) {
    let t12269 = t1523 * t2954;
    let t12278 = t926 * t9637;
    let t12287 = t140 * t3032;
    let t12288 = t12287 * t4047;
    let t12290 = t1098 * t12288 / 324.0;
    let t12291 = t140 * t1100;
    let t12292 = t12291 * t4052;
    let t12294 = t1098 * t12292 / 216.0;
    let t12317 = t9561 * t4241;
    let t12319 = t3067 * t12317 / 3456.0;
    let t12359 = t242 * t3090 * t4056;
    (t12269, t12278, t12290, t12294, t12319, t12359)
}
