//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1430/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1430<F: Float>(t44348: F, t50834: F, t71154: F, t71156: F, t77998: F, t78002: F, t78005: F, t78033: F, t78037: F, t78041: F, t78045: F, t78049: F) -> F {
    let t78278 = -F::cast_from(0.23744444444444444444e-1_f64) * t71154 + F::new(0.10685e0) * t77998 + F::cast_from(0.94977777777777777776e-1_f64) * t71156 + F::cast_from(0.23744444444444444444e0_f64) * t78002 - F::cast_from(0.47488888888888888888e-1_f64) * t78033 - F::cast_from(0.73871604938271604937e-1_f64) * t50834 + t44348 + F::cast_from(0.11872222222222222222e0_f64) * t78037 - F::cast_from(0.42739999999999999999e0_f64) * t78041 + F::new(0.6411e0) * t78045 + F::cast_from(0.14246666666666666667e0_f64) * t78049 - F::cast_from(0.35616666666666666666e-1_f64) * t78005;
    t78278
}
