//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 973/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk973<F: Float>(t10416: F, t70: F, t3426: F, t602: F, t10353: F, t36: F, t3432: F, t1290: F, t2025: F, t10409: F, t10413: F, t1314: F, t2026: F, t3427: F, t3433: F, t3436: F, t3483: F, t603: F, t616: F, t71: F, t85: F) -> (F, F) {
    let t10417 = t10416 * t70;
    let t10420 = t3426 * t602;
    let t10425 = t36 * t10353;
    let t10426 = t10425 * t70;
    let t10429 = t3432 * t602;
    let t10434 = t1290 * t2025;
    let t10439 = t2026 * t1314 / F::cast_from(24.0_f64) + t603 * t3483 / F::cast_from(12.0_f64) + t71 * t10409 / F::cast_from(24.0_f64) - t10413 * t85 / F::cast_from(12.0_f64) - t10417 * t85 / F::cast_from(6.0_f64) - t10420 * t85 / F::cast_from(6.0_f64) - t3427 * t616 / F::cast_from(6.0_f64) - t10426 * t85 / F::cast_from(12.0_f64) - t10429 * t85 / F::cast_from(6.0_f64) - t3433 * t616 / F::cast_from(6.0_f64) - t10434 * t85 / F::cast_from(12.0_f64) - t3436 * t616 / F::cast_from(6.0_f64);
    (t10425, t10439)
}
