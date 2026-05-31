//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2899/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2899<F: Float>(t59661: F, t59663: F, t59665: F, t59670: F, t59674: F, t59678: F, t60186: F, t60189: F, t60192: F, t60194: F, t60197: F, t60200: F, t60202: F, t60204: F, t60207: F) -> F {
    let t60498 = F::cast_from(0.3071625e0_f64) * t60186 + F::cast_from(0.197176e1_f64) * t60189 + F::cast_from(0.71752e1_f64) * t59661 + F::cast_from(0.65725333333333333332e0_f64) * t60192 - F::cast_from(0.43816888888888888888e0_f64) * t60194 - F::cast_from(0.49293999999999999999e0_f64) * t60197 + F::cast_from(0.32862666666666666666e0_f64) * t60200 - F::cast_from(0.21908444444444444444e0_f64) * t60202 - F::cast_from(0.30428395061728395062e-1_f64) * t60204 - F::cast_from(0.54771111111111111112e-1_f64) * t60207 - F::cast_from(0.39862222222222222222e0_f64) * t59663 + F::cast_from(0.13287407407407407408e0_f64) * t59665 - F::cast_from(0.39862222222222222222e0_f64) * t59670 - F::cast_from(0.19931111111111111111e0_f64) * t59674 - F::cast_from(0.39862222222222222222e0_f64) * t59678;
    t60498
}
