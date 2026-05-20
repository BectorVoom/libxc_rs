//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1131/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1131<F: Float>(t6646: F, t7524: F, t1888: F, t1519: F, t1894: F, t214: F, t1880: F, t1530: F, t25: F, t1484: F, t28: F, t1458: F, t88: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7525 = t6646 * t7524;
    let t7526 = t1888 * t7525;
    let t7528 = t1894 * t1519;
    let t7529 = t214 * t7528;
    let t7530 = t1880 * t7529;
    let t7545 = t25 * t1530;
    let t7649 = t28 * t1484;
    let t7656 = t28 * t1530;
    let t7676 = t88 * t1458;
    (t7525, t7526, t7528, t7529, t7530, t7545, t7649, t7656, t7676)
}
