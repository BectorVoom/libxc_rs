//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1026/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1026<F: Float>(t12283: F, t5303: F, t1340: F, t16060: F, t3798: F, t5234: F, t1354: F, t12211: F, t5223: F, t3804: F, t820: F, t1351: F, t1824: F) -> (F, F, F, F, F, F, F) {
    let t16269 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t12283 * t5303;
    let t16278 = t16060 * t1340;
    let t16288 = t5234 * t3798;
    let t16290 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t16288 * t1354;
    let t16294 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t12211 * t5223;
    let t16305 = t3804 * t820;
    let t16306 = t1824 * t1351;
    (t16269, t16278, t16288, t16290, t16294, t16305, t16306)
}
