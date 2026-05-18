//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 984/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk984<F: Float>(t12283: F, t5303: F, t1340: F, t16060: F, t3798: F, t5234: F, t1354: F, t12211: F, t5223: F, t3804: F, t820: F, t1351: F, t1824: F) -> (F, F, F, F, F, F, F) {
    let t16269 = F::new(7.0) / F::new(576.0) * t12283 * t5303;
    let t16278 = t16060 * t1340;
    let t16288 = t5234 * t3798;
    let t16290 = F::new(7.0) / F::new(2304.0) * t16288 * t1354;
    let t16294 = F::new(7.0) / F::new(24.0) * t12211 * t5223;
    let t16305 = t3804 * t820;
    let t16306 = t1824 * t1351;
    (t16269, t16278, t16288, t16290, t16294, t16305, t16306)
}
