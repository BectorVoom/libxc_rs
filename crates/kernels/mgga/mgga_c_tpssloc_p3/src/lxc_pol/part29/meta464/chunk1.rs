//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1792/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1792<F: Float>(t23110: F, t6648: F, t23185: F, t226: F, t23026: F, t23029: F, t23032: F, t23038: F, t23151: F, t23156: F, t23160: F, t23167: F, t23170: F, t23174: F, t23178: F, t23182: F) -> (F, F, F) {
    let t23186 = t23110 * t6648;
    let t23187 = t23185 * t23186;
    let t23189 = -F::cast_from(0.82246703342411321824e-2_f64) * t23026 - t23029 + t23032 + F::cast_from(0.49348022005446793095e-1_f64) * t23038 + t226 * t23151 - F::cast_from(0.3289868133696452873e-1_f64) * t23156 - F::cast_from(0.16449340668482264365e-1_f64) * t23160 + t23167 + t23170 - t23174 - F::cast_from(0.16449340668482264365e-1_f64) * t23178 - F::cast_from(0.82246703342411321825e-2_f64) * t23182 + F::cast_from(0.82246703342411321824e-2_f64) * t23187;
    (t23186, t23187, t23189)
}
