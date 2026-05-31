//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 365/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk365<F: Float>(t1096: F, t1121: F, t1124: F, t1129: F, t1138: F, t1144: F, t1148: F, t1157: F, t300: F, t436: F, t440: F) -> (F, F, F) {
    let t1161 = t300 * (-F::cast_from(0.310907e-1_f64) * t1124 * t436 + F::cast_from(1.0_f64) * t1129 * t1138 + t1096 - t1121 - F::cast_from(0.19751673498613801407e-1_f64) * t1144 + F::cast_from(0.5848223622634646207e0_f64) * t1148 * t1157);
    let t1163 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t1144;
    let t1164 = t300 * t440;
    (t1161, t1163, t1164)
}
