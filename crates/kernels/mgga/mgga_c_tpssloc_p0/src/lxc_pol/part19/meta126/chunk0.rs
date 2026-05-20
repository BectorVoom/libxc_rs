//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 677/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk677<F: Float>(t3481: F, t491: F, t1190: F, t1235: F, t1191: F, t225: F, t1202: F, t1226: F) -> (F, F, F, F) {
    let t3482 = t3481 * t491;
    let t3484 = t1190 * t1235;
    let t3487 = t1191 * t225;
    let t3490 = t1202 * t1226;
    (t3482, t3484, t3487, t3490)
}
