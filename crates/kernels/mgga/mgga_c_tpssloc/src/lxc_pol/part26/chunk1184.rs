//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1184/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1184<F: Float>(t28: F, t9516: F, t1081: F, t2749: F, t23788: F, t46298: F, t25891: F, t9616: F, t2745: F, t25927: F, t46362: F, t46252: F, t11122: F, t1877: F, t1915: F, t1969: F, t22959: F, t23286: F, t23290: F, t23295: F, t23789: F, t23813: F, t25013: F, t2522: F, t25372: F, t3231: F, t6666: F, t6670: F, t6841: F, t6848: F, t81483: F, t81525: F, t82320: F) -> (F,) {
    let t83613 = t28 * t9516;
    let t83617 = t1081 * t2749;
    let t83624 = t23788 * t46298;
    let t83627 = t25891 * t9616;
    let t83630 = t1081 * t2745;
    let t83645 = t25927 * t46362;
    let t83651 = t23788 * t46252;
    let t83654 = 3.0 / 2.0 * t1877 * t23286 * t1081 - 9.0 * t81483 * t23789 + 3.0 / 2.0 * t2522 * t1915 * t83613 + 3.0 * t1877 * t23295 * t83617 - 3.0 / 2.0 * t1877 * t23290 * t23813 - 9.0 * t25013 * t83624 + 9.0 * t25013 * t83627 - 3.0 / 2.0 * t1877 * t6670 * t83630 - 3.0 / 2.0 * t1877 * t81525 * t6848 + 9.0 / 2.0 * t2522 * t23286 * t6841 + t1877 * t1915 * t11122 / 2.0 + 3.0 * t82320 * t1969 + 3.0 * t25372 * t83645 + 3.0 / 2.0 * t1877 * t6666 * t3231 - 9.0 / 2.0 * t22959 * t83651;
    (t83654,)
}
