//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 826/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk826<F: Float>(t20264: F, t33: F, t20217: F, t20234: F, t4007: F, t4012: F, t5398: F, t634: F, t638: F, t9321: F, t9330: F, t72: F) -> (F, F, F) {
    let t20265 = t33 * t20264;
    let t20284 = -F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t9321 * t20234 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t4007 * t5398 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t634 * t20217 + F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t9330 * t20234 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t4012 * t5398 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t638 * t20217;
    let t20285 = t72 * t20284;
    (t20265, t20284, t20285)
}
