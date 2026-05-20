//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2693/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2693<F: Float>(t19783: F, t54670: F, t16081: F, t19787: F, t5187: F, t5308: F, t16018: F, t16101: F, t19781: F, t221: F, t3719: F, t46838: F, t5195: F, t5196: F, t54673: F, t54676: F, t54690: F, t54698: F, t54701: F, t54705: F, t54711: F, t54721: F, t54725: F) -> (F, F) {
    let t56548 = t54670 * t19783;
    let t56550 = t16081 * t19787;
    let t56560 = t5308 * t5187;
    let t56568 = -F::cast_from(0.23333333333333333332e-1_f64) * t54673 + F::cast_from(0.6333333333333333333e-1_f64) * t54676 - F::cast_from(0.99999999999999999996e-2_f64) * t54690 - F::cast_from(0.49999999999999999998e-2_f64) * t54698 + F::cast_from(0.15555555555555555555e0_f64) * t54701 + F::cast_from(0.93333333333333333328e-1_f64) * t56548 - F::cast_from(0.46666666666666666664e-1_f64) * t56550 - F::cast_from(0.19999999999999999999e-1_f64) * t16101 * t221 * t19781 * t3719 + F::cast_from(0.99999999999999999996e-2_f64) * t5195 * t221 * t5196 * t16018 - F::cast_from(0.79999999999999999996e-1_f64) * t16101 * t46838 * t56560 + F::cast_from(0.16666666666666666666e-2_f64) * t54705 - F::cast_from(0.46666666666666666664e-1_f64) * t54711 + F::cast_from(0.19999999999999999999e-1_f64) * t54721 + F::cast_from(0.55555555555555555553e-3_f64) * t54725;
    (t56560, t56568)
}
