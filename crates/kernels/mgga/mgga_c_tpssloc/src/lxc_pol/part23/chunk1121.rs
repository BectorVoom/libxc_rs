//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1121/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1121<F: Float>(t3375: F, t6063: F, t3400: F, t3312: F, t5983: F, t2403: F, t6011: F, t6014: F, t6017: F, t3356: F, t6031: F, t3263: F, t3331: F, t11282: F, t6084: F, t11292: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t63454 = t6063 * t3375;
    let t63602 = t6063 * t3400;
    let t63755 = t5983 * t3312;
    let t63888 = t2403 * t6011;
    let t63893 = t2403 * t6014;
    let t63911 = t2403 * t6017;
    let t64103 = t6031 * t3356;
    let t64257 = t5983 * t3263;
    let t64292 = t6031 * t3331;
    let t64451 = t11282 * t6084;
    let t64537 = t11292 * t6084;
    (t63454, t63602, t63755, t63888, t63893, t63911, t64103, t64257, t64292, t64451, t64537)
}
