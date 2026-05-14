//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 927/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk927<F: Float>(t1358: F, t7715: F, t1831: F, t22783: F, t5234: F, t6951: F, t1811: F, t22797: F, t22804: F, t7709: F, t1361: F, t1799: F, t22690: F, t22792: F, t1878: F, t22683: F) -> (F, F, F, F, F, F, F, F) {
    let t26251 = t7715 * t1358;
    let t26255 = t22783 * t1831;
    let t26257 = t5234 * t6951;
    let t26266 = t22797 * t1811;
    let t26268 = t22804 * t7709;
    let t26271 = t22690 * t1361 * t1799;
    let t26272 = t22792 * t26271;
    let t26284 = t1878 * t22683;
    (t26251, t26255, t26257, t26266, t26268, t26271, t26272, t26284)
}
