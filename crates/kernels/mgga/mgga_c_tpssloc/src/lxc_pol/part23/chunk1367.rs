//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1367/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1367<F: Float>(t550: F, t80150: F, t1336: F, t1380: F, t19654: F, t19739: F, t19743: F, t19810: F, t19815: F, t20473: F, t20554: F, t20568: F, t20632: F, t20638: F, t20643: F, t20645: F, t3897: F, t5234: F, t5334: F, t5344: F, t5348: F, t6415: F, t6454: F, t80085: F) -> (F, F) {
    let t80151 = t80150 * t550;
    let t80164 = -t1336 * t1380 * t80151 - 4.0 * t1336 * t20554 * t5348 - 4.0 * t1336 * t20568 * t5348 + 6.0 * t1336 * t3897 * t80085 + 24.0 * t19739 * t20473 * t5334 - 6.0 * t19743 * t5344 * t6415 + 24.0 * t19654 * t20638 - 12.0 * t19810 * t20632 - 6.0 * t19815 * t6454 - 4.0 * t20643 * t5234 - 12.0 * t20645 * t5234;
    (t80151, t80164)
}
