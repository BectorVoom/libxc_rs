//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 890/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk890<F: Float>(t1351: F, t1799: F, t12289: F, t242: F, t1336: F, t12283: F, t5259: F, t5293: F, t120: F, t5286: F, t5303: F, t1340: F, t16060: F, t3798: F, t5234: F, t1354: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16225 = t1799 * t1351;
    let t16232 = t12289 * t242;
    let t16233 = t1336 * t16232;
    let t16239 = 7.0 / 576.0 * t12283 * t5259;
    let t16241 = 7.0 / 2304.0 * t12283 * t5293;
    let t16242 = t120 * t5286;
    let t16269 = 7.0 / 576.0 * t12283 * t5303;
    let t16278 = t16060 * t1340;
    let t16288 = t5234 * t3798;
    let t16290 = 7.0 / 2304.0 * t16288 * t1354;
    (t16225, t16233, t16239, t16241, t16242, t16269, t16278, t16288, t16290)
}
