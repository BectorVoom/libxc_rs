//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1443/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1443<F: Float>(t31772: F, t7458: F, t26135: F, t89: F, t2040: F, t33214: F, t7050: F, t25994: F, t7042: F, t31537: F, t7802: F, t31540: F) -> (F, F, F, F, F, F) {
    let t122606 = t7458 * t31772;
    let t122607 = t89 * t26135;
    let t122608 = t122607 * t2040;
    let t122609 = t33214 * t7050;
    let t122610 = t7042 * t25994;
    let t122623 = F::new(2.0) * t31537 * t7802;
    let t122625 = F::new(2.0) * t31540 * t7802;
    (t122606, t122608, t122609, t122610, t122623, t122625)
}
