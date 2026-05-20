//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2538/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2538<F: Float>(t10375: F, t1612: F, t1041: F, t1539: F, t248: F, t42749: F, t10661: F, t1556: F, t14363: F, t300: F, t14419: F, t923: F) -> (F, F, F, F, F) {
    let t48670 = t1612 * t10375;
    let t48674 = t1041 * t248 * t42749 * t1539;
    let t48763 = t10661 * t1556;
    let t48766 = t300 * t14363;
    let t48771 = t14419 * t923;
    (t48670, t48674, t48763, t48766, t48771)
}
