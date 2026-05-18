//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1454/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1454<F: Float>(t26179: F, t8533: F, t31772: F, t7458: F, t26135: F, t89: F, t2040: F, t33214: F, t7050: F, t25994: F, t7042: F, t122597: F, t122598: F, t122599: F, t122600: F, t122602: F, t122603: F, t122604: F, t24999: F, t96361: F) -> F {
    let t122605 = t26179 * t8533;
    let t122606 = t7458 * t31772;
    let t122607 = t89 * t26135;
    let t122608 = t122607 * t2040;
    let t122609 = t33214 * t7050;
    let t122610 = t7042 * t25994;
    let t122613 = -t2040 * t96361 - t24999 * t7050 - t122597 - t122598 - t122599 - t122600 - t122602 - t122603 - t122604 - t122605 - t122606 - t122608 - t122609 - t122610;
    t122613
}
