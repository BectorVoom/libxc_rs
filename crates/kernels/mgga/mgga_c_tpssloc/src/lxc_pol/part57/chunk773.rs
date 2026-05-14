//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 773/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk773<F: Float>(t2040: F, t33211: F, t7467: F, t89: F, t7796: F, t8526: F, t1845: F, t2018: F, t26558: F, t26161: F, t4028: F, t8533: F, t1459: F, t1849: F, t31532: F, t33085: F, t33199: F, t33204: F, t33208: F, t6517: F, t652: F, t7042: F, t7472: F, t7802: F, t8604: F) -> (F, F, F, F) {
    let t33213 = 2.0 * t33211 * t2040;
    let t33214 = t89 * t7467;
    let t33216 = 2.0 * t33214 * t2040;
    let t33218 = 2.0 * t8526 * t7796;
    let t33221 = t2018 * t1845;
    let t33222 = t26558 * t33221;
    let t33224 = 2.0 * t26161 * t33222;
    let t33227 = 2.0 * t4028 * t8533;
    let t33228 = -2.0 * t1459 * t31532 + t1849 * t8604 - 2.0 * t2040 * t33085 - 2.0 * t33204 * t652 - 2.0 * t6517 * t7802 - 2.0 * t7042 * t7472 - t33199 - t33208 - t33213 - t33216 - t33218 + t33224 - t33227;
    (t33214, t33221, t33222, t33228)
}
