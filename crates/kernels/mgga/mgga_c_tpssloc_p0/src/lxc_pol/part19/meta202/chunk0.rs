//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 871/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk871<F: Float>(t10321: F, t908: F, t136: F, t10295: F, t10296: F, t10298: F, t10300: F, t10302: F, t10307: F, t10311: F, t10314: F, t10318: F, t10320: F) -> (F, F, F) {
    let t10322 = t908 * t10321;
    let t10323 = t136 * t10322;
    let t10325 = t10295 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t10296 - t10298 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10300 - t10302 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t10307 - t10311 / F::cast_from(3.0_f64) + t10314 / F::cast_from(6.0_f64) + t10318 - t10320 + t10323 / F::cast_from(6.0_f64);
    (t10322, t10323, t10325)
}
