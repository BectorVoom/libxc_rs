//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1276/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1276<F: Float>(t33231: F, t4034: F, t1873: F, t26870: F, t652: F, t31744: F, t4028: F, t26114: F, t8533: F, t26179: F, t31772: F, t7458: F, t26135: F, t89: F, t2040: F, t33214: F, t7050: F) -> (F, F, F, F, F, F, F, F) {
    let t122600 = t4034 * t33231;
    let t122602 = t652 * t26870 * t1873;
    let t122603 = t4028 * t31744;
    let t122604 = t26114 * t8533;
    let t122605 = t26179 * t8533;
    let t122606 = t7458 * t31772;
    let t122607 = t89 * t26135;
    let t122608 = t122607 * t2040;
    let t122609 = t33214 * t7050;
    (t122600, t122602, t122603, t122604, t122605, t122606, t122608, t122609)
}
