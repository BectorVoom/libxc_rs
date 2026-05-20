//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 965/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk965<F: Float>(t23097: F, t25111: F, t6612: F, t25115: F, t6605: F, t1484: F, t22690: F, t23122: F, t6619: F, t4162: F, t8342: F, t8344: F) -> (F, F, F, F) {
    let t118566 = t23097 * t6612 * t25111;
    let t118569 = t6605 * t6612 * t25115;
    let t118573 = t23122 * t22690 * t6619 * t1484;
    let t118576 = t4162 * t8342 * t8344;
    (t118566, t118569, t118573, t118576)
}
