//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 987/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk987<F: Float>(t1336: F, t16397: F, t5252: F, t225: F, t5319: F, t5217: F, t1390: F, t5356: F, t1395: F, t1858: F, t5381: F, t576: F) -> (F, F, F, F, F, F) {
    let t16398 = t1336 * t16397;
    let t16400 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t16398 * t5252;
    let t16439 = t5319 * t225;
    let t16460 = t5217 * t225;
    let t16497 = t5356 * t1390;
    let t16513 = F::cast_from(2.0_f64) * t1395 * t1858;
    let t16515 = F::cast_from(2.0_f64) * t576 * t5381;
    (t16400, t16439, t16460, t16497, t16513, t16515)
}
