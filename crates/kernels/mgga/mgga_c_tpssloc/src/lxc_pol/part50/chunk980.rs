//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 980/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk980<F: Float>(t6534: F, t89: F, t88: F, t1458: F, t8439: F, t4028: F, t8323: F, t7458: F, t1873: F, t7670: F, t652: F, t7685: F, t8494: F, t7688: F, t8450: F, t1976: F, t7467: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31540 = t89 * t6534;
    let t31717 = t88 * t6534;
    let t32656 = t8439 * t1458;
    let t32659 = t4028 * t8323;
    let t32661 = t7458 * t8323;
    let t32663 = t7670 * t1873;
    let t32664 = t652 * t32663;
    let t32666 = t7685 * t8494;
    let t32668 = t8450 * t7688;
    let t32670 = t1976 * t7467;
    (t31540, t31717, t32656, t32659, t32661, t32663, t32664, t32666, t32668, t32670)
}
