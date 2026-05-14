//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 729/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk729<F: Float>(t2157: F, t3640: F, t112: F, t2169: F, t33: F, t3953: F, t1437: F, t79: F, t72: F, t1410: F, t605: F, t1433: F, t71: F, t1874: F, t4028: F, t1458: F, t89: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7398 = t2157 * t3640;
    let t7423 = t2169 * t112;
    let t7428 = t3953 * t33;
    let t7431 = t79 * t1437;
    let t7432 = t72 * t7431;
    let t7435 = t605 * t1410;
    let t7445 = t71 * t1433;
    let t7457 = 2.0 * t4028 * t1874;
    let t7458 = t89 * t1458;
    (t7398, t7423, t7428, t7431, t7432, t7435, t7445, t7457, t7458)
}
