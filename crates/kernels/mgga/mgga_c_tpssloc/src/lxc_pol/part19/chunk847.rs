//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 847/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk847<F: Float>(t10426: F, t4594: F, t4582: F, t10283: F, t10361: F, t10364: F, t10367: F, t10370: F, t10372: F, t10377: F, t10378: F, t10381: F, t10385: F, t10388: F, t10390: F, t10394: F, t10398: F, t10403: F, t10405: F, t10410: F, t10413: F, t10415: F, t10419: F, t10424: F, t3070: F, t3073: F, t3130: F, t350: F, t378: F, t973: F) -> (F, F, F) {
    let t10427 = t10426 * t4594;
    let t10428 = t4582 * t10427;
    let t10431 = t10361 * t378 / 3072.0 + t973 * t10364 / 72.0 - t10367 * t378 / 192.0 + t10370 / 1536.0 + t10372 / 864.0 + t10377 - t973 * t10378 / 48.0 + t10381 / 54.0 + t10385 - 77.0 / 162.0 * t10283 * t350 + 11.0 / 108.0 * t10388 + t10390 * t3073 / 768.0 + t3070 * t10394 / 1536.0 + t3070 * t10398 / 1536.0 + t10403 * t10405 / 768.0 + 5.0 / 4608.0 * t3070 * t10410 - t10413 * t10415 / 1536.0 - t3070 * t10419 / 768.0 + t10424 / 1152.0 + t3130 * t10428 / 512.0;
    (t10427, t10428, t10431)
}
