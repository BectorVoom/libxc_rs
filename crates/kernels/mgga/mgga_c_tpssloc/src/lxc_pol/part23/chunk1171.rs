//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1171/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1171<F: Float>(t1171: F, t22104: F, t3447: F, t4904: F, t64779: F, t15402: F, t21749: F, t22398: F, t225: F, t1243: F, t72361: F, t22334: F, t22337: F, t22328: F, t20396: F, t67: F, t758: F) -> (F, F, F, F, F, F, F, F, F) {
    let t73523 = t22104 * t1171;
    let t73535 = t3447 * t64779 * t4904;
    let t73541 = t3447 * t15402 * t21749;
    let t73613 = t22398 * t225;
    let t73630 = t72361 * t1243;
    let t73856 = t22334 * t225;
    let t73891 = t22337 * t225;
    let t73900 = t22328 * t225;
    let t73967 = t20396 * t67 * t758;
    (t73523, t73535, t73541, t73613, t73630, t73856, t73891, t73900, t73967)
}
