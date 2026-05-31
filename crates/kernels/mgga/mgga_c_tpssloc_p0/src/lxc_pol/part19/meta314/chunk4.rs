//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1120/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1120<F: Float>(t25: F, t11985: F, t526: F, t3665: F, t2249: F, t12061: F, t12064: F, t3664: F, t39109: F, t514: F, t9257: F, t11998: F, t528: F, zeta_threshold: F) -> (F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t39419 = F::cast_from(1.0_f64) / t526 / t11985;
    let t39420 = t3665 * t3665;
    let t39426 = t2249 * t2249;
    let t39434 = piecewise3::<F>(t26, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t39419 * t39420 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t12061 * t3665 * t2249 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3664 * t39426 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t12064 * t9257 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t514 * t39109);
    let t39436 = F::cast_from(1.0_f64) / t528 / t11998;
    (t39420, t39426, t39434, t39436)
}
