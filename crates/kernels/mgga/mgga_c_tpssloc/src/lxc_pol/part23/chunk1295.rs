//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1295/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1295<F: Float>(t44075: F, t44077: F, t78129: F, t63332: F, t63334: F, t63361: F, t71142: F, t71144: F, t71146: F, t71152: F, t77989: F, t77992: F, t77995: F, t78057: F, t43942: F, t50834: F, t71154: F, t71156: F, t77998: F, t78002: F, t78005: F, t78033: F, t78037: F, t78041: F, t78045: F, t78049: F) -> (F, F, F) {
    let t78199 = 0.24955700379505800916e5 * t44075 * t78129 * t44077;
    let t78211 = -0.16481481481481481482e-1 * t63332 + 0.24722222222222222222e-1 * t63334 + 0.24722222222222222222e-1 * t71142 - 0.74166666666666666668e-1 * t71144 + 0.49444444444444444445e-1 * t63361 - 0.22249999999999999999e0 * t78057 - 0.13734567901234567901e-1 * t71146 + 0.2225e0 * t77989 + 0.92708333333333333333e-2 * t77992 - 0.27469135802469135803e-1 * t77995 - 0.74166666666666666668e-1 * t71152;
    let t78223 = -0.12361111111111111111e-1 * t71154 + 0.55625000000000000001e-1 * t77998 + 0.49444444444444444444e-1 * t71156 + 0.12361111111111111111e0 * t78002 - 0.24722222222222222222e-1 * t78033 - 0.38456790123456790123e-1 * t50834 + t43942 + 0.61805555555555555555e-1 * t78037 - 0.22249999999999999999e0 * t78041 + 0.33375e0 * t78045 + 0.74166666666666666668e-1 * t78049 - 0.18541666666666666666e-1 * t78005;
    (t78199, t78211, t78223)
}
