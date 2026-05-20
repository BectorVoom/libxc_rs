//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1212/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1212<F: Float>(t3129: F, t61735: F, t3038: F, t1041: F, t10868: F, t248: F, t5685: F, t18086: F, t3069: F, t10482: F, t5872: F, t5681: F) -> (F, F, F, F, F, F) {
    let t61736 = t61735 * t3129;
    let t61739 = t61735 * t3038;
    let t61782 = t1041 * t248 * t10868 * t5685;
    let t61950 = t18086 * t3069;
    let t62079 = t5872 * t10482;
    let t62137 = t1041 * t248 * t10868 * t5681;
    (t61736, t61739, t61782, t61950, t62079, t62137)
}
