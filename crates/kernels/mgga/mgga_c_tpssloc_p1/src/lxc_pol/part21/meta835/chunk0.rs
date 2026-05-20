//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2962/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2962<F: Float>(t4649: F, t1009: F, t17875: F, t1011: F, t1019: F, t3030: F, t5848: F, t3032: F, t3129: F, t3038: F, t10891: F, t17655: F) -> (F, F, F, F, F, F, F) {
    let t61719 = t4649 * t4649;
    let t61729 = t17875 * t1009;
    let t61731 = t61729 * t1011 * t1019;
    let t61734 = t5848 * t3030;
    let t61735 = t61734 * t3032;
    let t61736 = t61735 * t3129;
    let t61739 = t61735 * t3038;
    let t61742 = t10891 * t17655;
    (t61719, t61729, t61731, t61734, t61736, t61739, t61742)
}
