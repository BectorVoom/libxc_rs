//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1115/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1115<F: Float>(t5: F, t33118: F, t8513: F, t31004: F, t31010: F, t31017: F, t31022: F, t33103: F, t33107: F, t33111: F, t33115: F, t8309: F, t112: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t33119 = t8513 * t33118;
    let t33123 = piecewise3::<F>(t8, F::cast_from(0.0_f64), F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t33103 * t8309 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t31004 * t33107 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t31010 * t33111 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t31017 * t33115 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t31022 * t33119);
    let t33124 = t33123 * t112;
    (t33119, t33123, t33124)
}
