//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1288/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1288<F: Float>(t63332: F, t63334: F, t63361: F, t71142: F, t71144: F, t71146: F, t71152: F, t77989: F, t77992: F, t77995: F, t78057: F, t43820: F, t50834: F, t71154: F, t71156: F, t77998: F, t78002: F, t78005: F, t78033: F, t78037: F, t78041: F, t78045: F, t78049: F) -> (F, F) {
    let t78064 = -16.0 / 27.0 * t63332 + 8.0 / 9.0 * t63334 + 8.0 / 9.0 * t71142 - 8.0 / 3.0 * t71144 + 16.0 / 9.0 * t63361 - 8.0 * t78057 - 40.0 / 81.0 * t71146 + 8.0 * t77989 + t77992 / 3.0 - 80.0 / 81.0 * t77995 - 8.0 / 3.0 * t71152;
    let t78076 = -4.0 / 9.0 * t71154 + 2.0 * t77998 + 16.0 / 9.0 * t71156 + 40.0 / 9.0 * t78002 - 8.0 / 9.0 * t78033 - 112.0 / 81.0 * t50834 + t43820 + 20.0 / 9.0 * t78037 - 8.0 * t78041 + 12.0 * t78045 + 8.0 / 3.0 * t78049 - 2.0 / 3.0 * t78005;
    (t78064, t78076)
}
