//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1329/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1329<F: Float>(t43763: F, t44620: F, t1174: F, t3551: F, t698: F, t11545: F, t43791: F, t3242: F, t415: F, t61: F, t42341: F, t44696: F, t42344: F, t483: F, t1210: F, t3561: F) -> (F, F, F, F, F, F, F, F) {
    let t44805 = t44620 * t43763;
    let t44811 = t1174 * t698 * t3551;
    let t44817 = t11545 * t43791;
    let t44827 = 1.0 / t415 / t3242;
    let t44828 = t61 * t44827;
    let t44833 = t44696 * t42341;
    let t44834 = t483 * t42344;
    let t44836 = t44833 * t1210 * t44834;
    let t44847 = t1174 * t698 * t3561;
    (t44805, t44811, t44817, t44828, t44833, t44834, t44836, t44847)
}
