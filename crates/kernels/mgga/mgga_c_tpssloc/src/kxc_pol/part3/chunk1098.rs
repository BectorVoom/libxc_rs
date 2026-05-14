//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1098/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1098<F: Float>(t16205: F, t550: F, t1343: F, t820: F, t12365: F, t1827: F, t12300: F, t1799: F, t3734: F, t12351: F, t12418: F, t1351: F, t3807: F, t12289: F, t242: F, t1336: F) -> (F, F, F, F, F, F, F) {
    let t16206 = t16205 * t550;
    let t16208 = t1343 * t820 * t16206;
    let t16211 = t12365 * t1827;
    let t16214 = 7.0 / 2304.0 * t12300 * t1827;
    let t16215 = t1799 * t3734;
    let t16217 = t12351 * t820 * t16215;
    let t16224 = t12418 * t820;
    let t16225 = t1799 * t1351;
    let t16226 = t16225 * t3807;
    let t16227 = t16224 * t16226;
    let t16232 = t12289 * t242;
    let t16233 = t1336 * t16232;
    (t16206, t16208, t16211, t16214, t16217, t16227, t16233)
}
