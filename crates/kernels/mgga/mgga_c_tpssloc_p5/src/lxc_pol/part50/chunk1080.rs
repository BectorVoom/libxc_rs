//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1080/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1080<F: Float>(t7685: F, t8494: F, t7688: F, t8450: F, t1976: F, t7467: F, t652: F, t4028: F, t8327: F, t7458: F, t1774: F, t8326: F) -> (F, F, F, F, F, F, F) {
    let t32666 = t7685 * t8494;
    let t32668 = t8450 * t7688;
    let t32670 = t1976 * t7467;
    let t32671 = t652 * t32670;
    let t32673 = t4028 * t8327;
    let t32674 = F::new(2.0) * t32673;
    let t32675 = t7458 * t8327;
    let t32676 = F::new(2.0) * t32675;
    let t32677 = t1774 * t8326;
    (t32666, t32668, t32670, t32671, t32674, t32676, t32677)
}
