//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1361/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1361<F: Float>(t10283: F, t995: F, t10931: F, t135: F, t973: F, t1021: F, t1046: F, t10501: F, t10998: F, t248: F, t2960: F, t3048: F, t350: F, t42348: F, t42759: F, t43273: F, t43277: F, t43281: F, t43285: F, t43291: F, t43292: F, t43298: F, t43301: F, t43303: F, t43307: F) -> F {
    let t43310 = t10283 * t995;
    let t43313 = t973 * t135 * t10931;
    let t43315 = F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t3048 * t10501 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2960 * t10998 + t43273 / F::cast_from(36.0_f64) + t43277 / F::cast_from(192.0_f64) - t43281 / F::cast_from(192.0_f64) + t43285 / F::cast_from(1152.0_f64) + t43291 * t248 * t1021 * t42348 * t43292 / F::cast_from(128.0_f64) - t43298 * t1046 / F::cast_from(72.0_f64) + t43301 / F::cast_from(384.0_f64) + F::cast_from(19.0_f64) / F::cast_from(216.0_f64) * t43303 - t43307 + F::cast_from(1309.0_f64) / F::cast_from(486.0_f64) * t42759 * t350 - F::cast_from(154.0_f64) / F::cast_from(243.0_f64) * t43310 - t43313 / F::cast_from(27.0_f64);
    t43315
}
