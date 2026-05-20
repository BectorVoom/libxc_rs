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
    let t43315 = F::new(5.0) / F::new(108.0) * t3048 * t10501 - F::new(2.0) / F::new(9.0) * t2960 * t10998 + t43273 / F::new(36.0) + t43277 / F::new(192.0) - t43281 / F::new(192.0) + t43285 / F::new(1152.0) + t43291 * t248 * t1021 * t42348 * t43292 / F::new(128.0) - t43298 * t1046 / F::new(72.0) + t43301 / F::new(384.0) + F::new(19.0) / F::new(216.0) * t43303 - t43307 + F::new(1309.0) / F::new(486.0) * t42759 * t350 - F::new(154.0) / F::new(243.0) * t43310 - t43313 / F::new(27.0);
    t43315
}
