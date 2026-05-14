//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1319/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1319<F: Float>(t3475: F, t4899: F, t11545: F, t135: F, t11548: F, t1174: F, t43791: F, t461: F, t3439: F, t698: F, t3442: F, t11588: F, t3447: F, t3451: F, t1176: F, t697: F) -> (F, F, F, F, F, F) {
    let t44558 = t4899 * t3475;
    let t44562 = t135 * t11545;
    let t44564 = t1174 * t44562 * t11548;
    let t44566 = t461 * t43791;
    let t44571 = t698 * t3439;
    let t44573 = t1174 * t44571 * t3442;
    let t44579 = t11588 * t3475;
    let t44581 = t3447 * t44579 * t3451;
    let t44583 = t697 * t1176;
    (t44558, t44564, t44566, t44573, t44581, t44583)
}
