//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1139/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1139<F: Float>(t23097: F, t2628: F, t2632: F, t47320: F, t46519: F, t6605: F, t133: F, t1891: F, t6601: F, t80953: F, t46511: F, t815: F) -> (F, F, F, F) {
    let t81728 = t23097 * t2628 * t47320 * t2632;
    let t81731 = t6605 * t2628 * t46519;
    let t81735 = t80953 * t1891 * t133 * t6601;
    let t81738 = t6605 * t815 * t46511;
    (t81728, t81731, t81735, t81738)
}
