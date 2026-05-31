//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 941/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk941<F: Float>(t159: F, t3096: F, t9213: F, t395: F, t402: F, t392: F, t2909: F, t404: F, t394: F, t9181: F, t2997: F, t430: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9230 = t159 * t3096;
    let t9243 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t9213;
    let t9267 = F::cast_from(1.0_f64) / t395 / t402 / F::cast_from(4.0_f64);
    let t9271 = F::cast_from(1.0_f64)/pow_3_2::<F>(t392);
    let t9291 = F::cast_from(1.0_f64) / t2909 / t404;
    let t9292 = t394 * t9291;
    let t9297 = F::cast_from(0.36514074074074074075e0_f64) * t9181;
    let t9306 = F::cast_from(0.93011851851851851854e0_f64) * t9213;
    let t9331 = F::cast_from(0.28842592592592592592e-1_f64) * t9213;
    let t9347 = F::cast_from(1.0_f64) / t2997 / t430;
    (t9230, t9243, t9267, t9271, t9292, t9297, t9306, t9331, t9347)
}
