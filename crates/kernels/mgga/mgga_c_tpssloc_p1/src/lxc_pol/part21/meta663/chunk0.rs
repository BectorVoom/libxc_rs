//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2464/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2464<F: Float>(t11147: F, t460: F, t11588: F, t3469: F, t1184: F, t15418: F, t4899: F, t3475: F, t11545: F, t135: F, t3439: F, t698: F) -> (F, F, F, F, F, F, F) {
    let t44505 = t460 * t11147;
    let t44510 = t11588 * t3469;
    let t44525 = t15418 * t1184;
    let t44529 = t4899 * t3469;
    let t44558 = t4899 * t3475;
    let t44562 = t135 * t11545;
    let t44571 = t698 * t3439;
    (t44505, t44510, t44525, t44529, t44558, t44562, t44571)
}
