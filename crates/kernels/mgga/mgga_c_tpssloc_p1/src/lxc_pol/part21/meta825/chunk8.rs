//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2908/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2908<F: Float>(t59661: F, t59663: F, t59665: F, t59670: F, t59674: F, t59678: F, t60186: F, t60189: F, t60192: F, t60194: F, t60197: F, t60200: F, t60202: F, t60204: F, t60207: F) -> F {
    let t60634 = F::new(0.6311625e0) * t60186 + F::new(0.250068e1) * t60189 + F::new(0.123954e2) * t59661 + F::cast_from(0.83356000000000000001e0_f64) * t60192 - F::cast_from(0.55570666666666666667e0_f64) * t60194 - F::new(0.62517e0) * t60197 + F::new(0.41678e0) * t60200 - F::cast_from(0.27785333333333333334e0_f64) * t60202 - F::cast_from(0.38590740740740740742e-1_f64) * t60204 - F::cast_from(0.69463333333333333334e-1_f64) * t60207 - F::cast_from(0.68863333333333333333e0_f64) * t59663 + F::cast_from(0.22954444444444444444e0_f64) * t59665 - F::cast_from(0.68863333333333333334e0_f64) * t59670 - F::cast_from(0.34431666666666666667e0_f64) * t59674 - F::cast_from(0.68863333333333333334e0_f64) * t59678;
    t60634
}
