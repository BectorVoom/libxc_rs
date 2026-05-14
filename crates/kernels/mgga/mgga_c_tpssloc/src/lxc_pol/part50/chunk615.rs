//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 615/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk615<F: Float>(t1883: F, t6547: F, t131: F, t209: F, t229: F, t1878: F) -> (F, F, F) {
    let t6548 = t6547 * t1883;
    let t6549 = 0.19190897446562641759e-1 * t6548;
    let t6551 = t229 * t131 * t209;
    let t6552 = t1878 * t6551;
    (t6549, t6551, t6552)
}
