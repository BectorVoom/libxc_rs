//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1765/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1765<F: Float>(t19404: F, t33: F, t5392: F, t9321: F, t2291: F, t5398: F, t9330: F, t2298: F, t16558: F, t3966: F, t4007: F, t4012: F, t607: F, t634: F, t638: F) -> (F, F, F, F) {
    let t19405 = t33 * t19404;
    let t19420 = t9321 * t5392;
    let t19425 = t2291 * t5398;
    let t19430 = t9330 * t5392;
    let t19435 = t2298 * t5398;
    let t19440 = -F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t19420 * t607 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t4007 * t3966 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t19425 * t607 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t634 * t16558 + F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t19430 * t607 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t4012 * t3966 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t19435 * t607 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t638 * t16558;
    (t19405, t19420, t19430, t19440)
}
