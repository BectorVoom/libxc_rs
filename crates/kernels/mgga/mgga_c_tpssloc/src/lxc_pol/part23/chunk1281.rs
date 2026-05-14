//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1281/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1281<F: Float>(t63332: F, t63334: F, t63888: F, t63893: F, t63911: F, t71142: F, t71144: F, t71146: F, t71152: F, t71154: F, t71156: F, t71408: F, t78002: F, t78005: F, t5999: F, t3270: F) -> (F, F, F) {
    let t78019 = 0.40256666666666666666e1 * t78002 - 0.60384999999999999999e0 * t78005 - 0.53675555555555555556e0 * t63332 + 0.80513333333333333336e0 * t63334 - 0.18396666666666666667e0 * t63888 + 0.11038e1 * t63893 + 0.80513333333333333333e0 * t71142 - 0.24154e1 * t71144 + 0.5519e0 * t63911 - 0.22076e0 * t71408 - 0.44729629629629629629e0 * t71146 - 0.24154e1 * t71152 - 0.40256666666666666668e0 * t71154 + 0.16102666666666666667e1 * t71156;
    let t78025 = t5999 * t5999;
    let t78026 = t3270 * t78025;
    (t78019, t78025, t78026)
}
