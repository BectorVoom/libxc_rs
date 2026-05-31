//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1477/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1477<F: Float>(t6243: F, t1751: F, t22298: F, t491: F, t78757: F, t6224: F, t6238: F, t11914: F, t11915: F, t1244: F, t1246: F, t15245: F, t1734: F, t1755: F, t1756: F, t19201: F, t22243: F, t22327: F, t22354: F, t22355: F, t22389: F, t3610: F, t3612: F, t3624: F, t3625: F, t6218: F, t6252: F, t6253: F, t6257: F, t65254: F, t73630: F) -> (F, F, F, F, F) {
    let t79398 = t6243 * t6243;
    let t79410 = t1751 * t22298;
    let t79453 = t491 * t78757;
    let t79461 = t6238 * t6224;
    let t79467 = F::cast_from(6.0_f64) * t11914 * t11915 * t6218 * t6252 + F::cast_from(4.0_f64) * t1244 * t1246 * t1734 * t22327 + F::cast_from(8.0_f64) * t1755 * t22243 * t3610 * t3612 + F::cast_from(4.0_f64) * t11914 * t11915 * t79410 - F::cast_from(12.0_f64) * t22354 * t22389 * t3624 - F::cast_from(3.0_f64) * t3624 * t3625 * t79453 - F::cast_from(6.0_f64) * t3624 * t3625 * t79461 - F::cast_from(12.0_f64) * t15245 * t22355 + F::cast_from(4.0_f64) * t1756 * t73630 + F::cast_from(12.0_f64) * t19201 * t6257 + F::cast_from(12.0_f64) * t6253 * t65254;
    (t79398, t79410, t79453, t79461, t79467)
}
