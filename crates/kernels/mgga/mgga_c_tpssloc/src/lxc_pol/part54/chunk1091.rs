//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1091/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1091<F: Float>(t2040: F, t33211: F, t7467: F, t89: F, t7796: F, t8526: F, t1845: F, t2018: F, t26558: F, t26161: F, t4028: F, t8533: F, t7458: F, t1873: F, t7890: F, t652: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t33213 = 2.0 * t33211 * t2040;
    let t33214 = t89 * t7467;
    let t33216 = 2.0 * t33214 * t2040;
    let t33218 = 2.0 * t8526 * t7796;
    let t33221 = t2018 * t1845;
    let t33222 = t26558 * t33221;
    let t33224 = 2.0 * t26161 * t33222;
    let t33227 = 2.0 * t4028 * t8533;
    let t33230 = 2.0 * t7458 * t8533;
    let t33231 = t7890 * t1873;
    let t33233 = 2.0 * t652 * t33231;
    (t33213, t33214, t33216, t33218, t33221, t33222, t33224, t33227, t33230, t33231, t33233)
}
