//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 997/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk997<F: Float>(t11569: F, t11571: F, t3448: F, t3469: F, t3451: F, t2250: F, t3450: F, t3449: F, t3247: F, t460: F, t2244: F, t1176: F, t134: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11572 = t11569 * t11571;
    let t11575 = t3448 * t3469;
    let t11576 = t11575 * t3451;
    let t11579 = t3450 * t2250;
    let t11580 = t3449 * t11579;
    let t11583 = t460 * t3247;
    let t11584 = t11583 * t2244;
    let t11585 = t3449 * t11584;
    let t11588 = t134 * t1176;
    (t11572, t11575, t11576, t11579, t11580, t11583, t11584, t11585, t11588)
}
