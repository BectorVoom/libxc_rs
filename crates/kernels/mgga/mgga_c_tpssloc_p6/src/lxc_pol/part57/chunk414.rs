//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 414/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk414<F: Float>(t1543: F, t892: F, t1553: F, t699: F, t1561: F, t923: F, t1573: F, t942: F, t300: F, t1592: F, t2970: F, t973: F) -> (F, F, F, F, F, F) {
    let t4354 = t1543 * t892;
    let t4384 = t699 * t1553;
    let t4411 = t1561 * t923;
    let t4449 = t1573 * t942;
    let t4483 = t300 * t1573;
    let t4506 = t2970 * t1592;
    let t4507 = t973 * t4506;
    (t4354, t4384, t4411, t4449, t4483, t4507)
}
