//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1351/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1351<F: Float>(t10943: F, t135: F, t973: F, t3152: F, t698: F, t10870: F, t3117: F, t1020: F, t10858: F, t248: F, t3101: F, t10961: F, t3108: F) -> (F, F, F, F, F) {
    let t43103 = t973 * t135 * t10943;
    let t43110 = t973 * t698 * t3152;
    let t43114 = t3117 * t10870;
    let t43118 = t1020 * t248 * t3101 * t10858;
    let t43120 = t10961 * t3108;
    (t43103, t43110, t43114, t43118, t43120)
}
