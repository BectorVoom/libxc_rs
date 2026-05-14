//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1145/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1145<F: Float>(t1009: F, t21480: F, t1057: F, t1615: F, t883: F, t5866: F, t17906: F, t4644: F, t17607: F, t4571: F, t1011: F, t1019: F, t1040: F, t21482: F, t10876: F, t21396: F, t248: F, t3101: F) -> (F, F, F, F, F, F, F, F) {
    let t69923 = t21480 * t1009;
    let t69924 = t69923 * t1057;
    let t70100 = t1615 * t883;
    let t70122 = t5866 * t1615;
    let t70132 = t4644 * t17906;
    let t70138 = t17607 * t4571;
    let t70148 = t69923 * t1011 * t1019;
    let t70153 = t21482 * t1040;
    let t70162 = t10876 * t248 * t3101 * t21396;
    (t69924, t70100, t70122, t70132, t70138, t70148, t70153, t70162)
}
