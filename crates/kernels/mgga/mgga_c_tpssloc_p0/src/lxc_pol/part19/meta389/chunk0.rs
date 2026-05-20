//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1462/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1462<F: Float>(t3242: F, t415: F, t61: F, t42341: F, t44696: F, t42344: F, t483: F, t1210: F, t1174: F, t3561: F, t698: F, t11738: F, t11739: F, t248: F, t3570: F) -> (F, F, F, F, F, F) {
    let t44827 = F::new(1.0) / t415 / t3242;
    let t44828 = t61 * t44827;
    let t44833 = t44696 * t42341;
    let t44834 = t483 * t42344;
    let t44836 = t44833 * t1210 * t44834;
    let t44847 = t1174 * t698 * t3561;
    let t44851 = t11738 * t248 * t3570 * t11739;
    (t44828, t44833, t44834, t44836, t44847, t44851)
}
