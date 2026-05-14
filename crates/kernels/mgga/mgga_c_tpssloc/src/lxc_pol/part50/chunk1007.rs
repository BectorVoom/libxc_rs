//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1007/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1007<F: Float>(t5: F, t33106: F, t8513: F, t1409: F, t31011: F, t7440: F, t8307: F, t1433: F, t79: F, t31004: F, t31010: F, t31017: F, t31022: F, t33103: F, t8309: F, t112: F, t7754: F, t8450: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t33107 = t8513 * t33106;
    let t33111 = t8513 * t31011 * t1409;
    let t33114 = t8307 * t7440;
    let t33115 = t8513 * t33114;
    let t33118 = t79 * t1433;
    let t33119 = t8513 * t33118;
    let t33123 = piecewise3(t8, 0.0, 5.0 / 144.0 * t33103 * t8309 - 5.0 / 24.0 * t31004 * t33107 - 5.0 / 36.0 * t31010 * t33111 + 5.0 / 72.0 * t31017 * t33115 + 5.0 / 72.0 * t31022 * t33119);
    let t33124 = t33123 * t112;
    let t33127 = t8450 * t7754;
    (t33107, t33111, t33114, t33115, t33118, t33119, t33123, t33124, t33127)
}
