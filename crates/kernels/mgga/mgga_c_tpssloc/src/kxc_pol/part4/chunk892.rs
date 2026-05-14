//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 892/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk892<F: Float>(t1369: F, t16336: F, t12189: F, t1811: F, t1358: F, t5231: F, t1815: F, t3862: F, t3726: F, t5227: F, t3802: F, t5234: F, t3788: F, t836: F, t1336: F, t5252: F) -> (F, F, F, F, F, F, F) {
    let t16338 = 7.0 / 576.0 * t16336 * t1369;
    let t16341 = t12189 * t1811;
    let t16346 = 7.0 / 2304.0 * t5231 * t1358;
    let t16350 = t1815 * t3862;
    let t16354 = 7.0 / 72.0 * t3726 * t5227;
    let t16394 = t5234 * t3802;
    let t16397 = t3788 * t836;
    let t16398 = t1336 * t16397;
    let t16400 = 7.0 / 1152.0 * t16398 * t5252;
    (t16338, t16341, t16346, t16350, t16354, t16394, t16400)
}
