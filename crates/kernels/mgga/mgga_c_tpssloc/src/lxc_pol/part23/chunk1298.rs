//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1298/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1298<F: Float>(t44348: F, t50834: F, t71154: F, t71156: F, t77998: F, t78002: F, t78005: F, t78033: F, t78037: F, t78041: F, t78045: F, t78049: F, t423: F, t78266: F, t21961: F, t51249: F) -> (F, F) {
    let t78278 = -0.23744444444444444444e-1 * t71154 + 0.10685e0 * t77998 + 0.94977777777777777776e-1 * t71156 + 0.23744444444444444444e0 * t78002 - 0.47488888888888888888e-1 * t78033 - 0.73871604938271604937e-1 * t50834 + t44348 + 0.11872222222222222222e0 * t78037 - 0.42739999999999999999e0 * t78041 + 0.6411e0 * t78045 + 0.14246666666666666667e0 * t78049 - 0.35616666666666666666e-1 * t78005;
    let t78281 = 0.621814e-1 * (t78266 + t78278) * t423;
    let t78283 = 0.3859675079686208416e3 * t51249 * t21961;
    (t78281, t78283)
}
