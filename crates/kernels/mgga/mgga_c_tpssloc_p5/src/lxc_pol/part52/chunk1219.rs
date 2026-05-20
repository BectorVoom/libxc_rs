//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1219/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1219<F: Float>(t6517: F, t7468: F, t1437: F, t8307: F, t7440: F, t8513: F, t1433: F, t79: F, t7754: F, t8450: F, t31047: F, t7687: F) -> (F, F, F, F, F, F, F) {
    let t33101 = t6517 * t7468;
    let t33106 = t8307 * t1437;
    let t33114 = t8307 * t7440;
    let t33115 = t8513 * t33114;
    let t33118 = t79 * t1433;
    let t33127 = t8450 * t7754;
    let t33129 = t31047 * t7687;
    (t33101, t33106, t33114, t33115, t33118, t33127, t33129)
}
