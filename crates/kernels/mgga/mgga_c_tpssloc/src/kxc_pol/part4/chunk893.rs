//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 893/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk893<F: Float>(t225: F, t5319: F, t5217: F, t1390: F, t5356: F, t1395: F, t1858: F, t5381: F, t576: F, t112: F, t5363: F, t111: F, t1851: F, t580: F, t6470: F, t5392: F, t9427: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16439 = t5319 * t225;
    let t16460 = t5217 * t225;
    let t16497 = t5356 * t1390;
    let t16513 = 2.0 * t1395 * t1858;
    let t16515 = 2.0 * t576 * t5381;
    let t16521 = t5363 * t112;
    let t16524 = t1851 * t111;
    let t16548 = t6470 * t580;
    let t16549 = t9427 * t5392;
    (t16439, t16460, t16497, t16513, t16515, t16521, t16524, t16548, t16549)
}
