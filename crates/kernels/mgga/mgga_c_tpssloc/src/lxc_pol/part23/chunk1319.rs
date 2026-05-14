//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1319/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1319<F: Float>(t63332: F, t63334: F, t63888: F, t63893: F, t63911: F, t71142: F, t71144: F, t71146: F, t71152: F, t71154: F, t71156: F, t71408: F, t78002: F, t78005: F, t44249: F, t50846: F, t71470: F, t71472: F, t71474: F, t78026: F, t78029: F, t78033: F, t78037: F, t78041: F, t78045: F, t78049: F, t78078: F, t78080: F) -> (F, F) {
    let t78824 = 0.68863333333333333334e1 * t78002 - 0.103295e1 * t78005 - 0.91817777777777777776e0 * t63332 + 0.13772666666666666666e1 * t63334 - 0.23154444444444444445e0 * t63888 + 0.13892666666666666667e1 * t63893 + 0.13772666666666666666e1 * t71142 - 0.41318e1 * t71144 + 0.69463333333333333334e0 * t63911 - 0.27785333333333333333e0 * t71408 - 0.76514814814814814814e0 * t71146 - 0.41318e1 * t71152 - 0.68863333333333333332e0 * t71154 + 0.27545333333333333332e1 * t71156;
    let t78839 = -0.12349037037037037037e1 * t50846 - 0.12349037037037037037e0 * t71470 + 0.55570666666666666668e0 * t71472 - 0.166712e1 * t71474 + t44249 - 0.52945875e1 * t78026 + 0.2366859375e0 * t78029 - 0.13772666666666666667e1 * t78033 + 0.34431666666666666667e1 * t78037 - 0.123954e2 * t78041 + 0.185931e2 * t78045 + 0.41318e1 * t78049 + 0.6311625e0 * t78078 - 0.6618234375e1 * t78080;
    (t78824, t78839)
}
