//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1013/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1013<F: Float>(t57: F, t13335: F, t14016: F, t14021: F, t3431: F, t3602: F, t581: F, t745: F, t14015: F, zeta_threshold: F) -> F {
    let t155 = t57 <= zeta_threshold;
    let t14027 = piecewise3::<F>(t155, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t14016 * t581 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3602 * t3431 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14021 * t581 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t745 * t13335);
    let t14029 = t14015 / F::cast_from(2.0_f64) + t14027 / F::cast_from(2.0_f64);
    t14029
}
