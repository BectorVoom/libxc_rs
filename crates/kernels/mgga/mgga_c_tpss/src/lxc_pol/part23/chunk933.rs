//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 933/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk933<F: Float>(t159: F, t3096: F, t9213: F, t395: F, t402: F, t392: F, t2909: F, t404: F, t394: F, t9181: F, t2997: F, t430: F, t1068: F, t2973: F, t1072: F, t2966: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9230 = t159 * t3096;
    let t9243 = 28.0 / 27.0 * t9213;
    let t9267 = 1.0 / t395 / t402 / 4.0;
    let t9271 = 1.0/pow_3_2(t392);
    let t9291 = 1.0 / t2909 / t404;
    let t9292 = t394 * t9291;
    let t9297 = 0.36514074074074074075e0 * t9181;
    let t9306 = 0.93011851851851851854e0 * t9213;
    let t9331 = 0.28842592592592592592e-1 * t9213;
    let t9347 = 1.0 / t2997 / t430;
    let t9359 = t1068 * t2973;
    let t9365 = t2966 * t1072;
    (t9230, t9243, t9267, t9271, t9292, t9297, t9306, t9331, t9347, t9359, t9365)
}
