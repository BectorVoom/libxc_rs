//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 831/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk831<F: Float>(t3377: F, t3403: F, t1129: F, t1138: F, t1148: F, t1157: F, t3258: F, t3261: F, t3268: F, t3310: F, t3318: F, t3324: F, t3327: F, t3332: F, t3334: F, t3352: F, t3357: F, t3360: F, t3369: F, t3371: F, t3376: F, t3378: F, t3396: F, t3401: F, t436: F) -> (F, F) {
    let t3404 = t3377 * t3403;
    let t3407 = -F::new(0.310907e-1) * t3324 * t436 + F::new(2.0) * t3327 * t1138 - F::new(2.0) * t3332 * t3334 + F::new(1.0) * t1129 * t3352 + F::cast_from(0.32163958997385070134e2_f64) * t3357 * t3360 + t3258 - t3261 + t3268 - t3310 - t3318 - F::cast_from(0.19751673498613801407e-1_f64) * t3369 + F::cast_from(0.11696447245269292414e1_f64) * t3371 * t1157 - F::cast_from(0.11696447245269292414e1_f64) * t3376 * t3378 + F::cast_from(0.5848223622634646207e0_f64) * t1148 * t3396 + F::cast_from(0.17315859105681463759e2_f64) * t3401 * t3404;
    (t3404, t3407)
}
