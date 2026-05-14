//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1124/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1124<F: Float>(t253: F, t254: F, t10109: F, t1911: F, t234: F, t6604: F, t193: F, t1962: F, t10143: F, t25: F, t2775: F, t387: F, t221: F, t4509: F, t1926: F, t2770: F) -> (F, F, F, F, F, F, F, F) {
    let t25168 = t253 * t254;
    let t25169 = t10109 * t1911;
    let t25248 = t6604 * t234;
    let t25372 = t193 * t1962;
    let t25373 = t10143 * t25;
    let t25423 = t387 * t2775;
    let t25428 = t221 * t4509;
    let t25429 = t1926 * t25428;
    let t25430 = t387 * t2770;
    (t25168, t25169, t25248, t25372, t25373, t25423, t25429, t25430)
}
