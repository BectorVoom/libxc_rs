//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1366/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1366<F: Float>(t43398: F, t61: F, t10309: F, t1041: F, t10457: F, t248: F, t10444: F, t354: F, t364: F, t372: F, t1021: F, t10364: F, t10408: F, t10413: F, t1046: F, t10482: F, t10962: F, t10965: F, t10972: F, t2771: F, t2960: F, t3041: F, t3057: F, t3064: F, t3117: F, t3123: F, t41667: F, t41715: F, t42348: F, t43374: F, t43377: F, t43382: F, t43385: F, t973: F, t977: F) -> F {
    let t43399 = t61 * t43398;
    let t43406 = t1041 * t248 * t10457 * t10309;
    let t43410 = t354 * t364 * t10444 * t372;
    let t43415 = -F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t10413 * t10408 * t3041 * t2771 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t2960 * t10364 - t43374 / F::cast_from(36.0_f64) + t43377 / F::cast_from(54.0_f64) + t973 * t977 * t41715 / F::cast_from(8.0_f64) + t43382 / F::cast_from(2592.0_f64) - F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t43385 * t248 * t1021 * t42348 * t10482 + t10965 * t3057 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t10965 * t3064 + F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t3117 * t10972 + F::cast_from(55.0_f64) / F::cast_from(15552.0_f64) * t1041 * t248 * t43399 * t41667 - F::cast_from(5.0_f64) / F::cast_from(864.0_f64) * t43406 - F::cast_from(209.0_f64) / F::cast_from(972.0_f64) * t43410 * t1046 + t10962 * t3123 / F::cast_from(512.0_f64);
    t43415
}
