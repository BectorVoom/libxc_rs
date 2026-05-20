//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 883/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk883<F: Float>(t10422: F, t3072: F, t3070: F, t3120: F, t376: F, t4594: F, t4582: F, t10283: F, t10361: F, t10364: F, t10367: F, t10370: F, t10372: F, t10377: F, t10378: F, t10381: F, t10385: F, t10388: F, t10390: F, t10394: F, t10398: F, t10403: F, t10405: F, t10410: F, t10413: F, t10415: F, t10419: F, t3073: F, t3130: F, t350: F, t378: F, t973: F) -> (F, F) {
    let t10423 = t10422 * t3072;
    let t10424 = t3070 * t10423;
    let t10426 = t376 * t3120;
    let t10427 = t10426 * t4594;
    let t10428 = t4582 * t10427;
    let t10431 = t10361 * t378 / F::new(3072.0) + t973 * t10364 / F::new(72.0) - t10367 * t378 / F::new(192.0) + t10370 / F::new(1536.0) + t10372 / F::new(864.0) + t10377 - t973 * t10378 / F::new(48.0) + t10381 / F::new(54.0) + t10385 - F::new(77.0) / F::new(162.0) * t10283 * t350 + F::new(11.0) / F::new(108.0) * t10388 + t10390 * t3073 / F::new(768.0) + t3070 * t10394 / F::new(1536.0) + t3070 * t10398 / F::new(1536.0) + t10403 * t10405 / F::new(768.0) + F::new(5.0) / F::new(4608.0) * t3070 * t10410 - t10413 * t10415 / F::new(1536.0) - t3070 * t10419 / F::new(768.0) + t10424 / F::new(1152.0) + t3130 * t10428 / F::new(512.0);
    (t10426, t10431)
}
