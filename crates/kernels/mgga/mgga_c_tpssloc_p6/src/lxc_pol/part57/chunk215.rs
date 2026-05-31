//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 215/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk215<F: Float>(t1127: F, t427: F, t1086: F, t1111: F, t435: F, t445: F, t440: F, t448: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1128 = F::cast_from(1.0_f64) / t1127;
    let t1129 = t427 * t1128;
    let t1131 = F::cast_from(0.516475e0_f64) * t1086;
    let t1134 = F::cast_from(0.104195e0_f64) * t1111;
    let t1137 = F::cast_from(1.0_f64) / t435;
    let t1141 = F::cast_from(0.92708333333333333333e-2_f64) * t1086;
    let t1146 = t445 * t445;
    let t1147 = F::cast_from(1.0_f64) / t1146;
    let t1148 = t440 * t1147;
    let t1150 = F::cast_from(0.301925e0_f64) * t1086;
    let t1153 = F::cast_from(0.82785e-1_f64) * t1111;
    let t1156 = F::cast_from(1.0_f64) / t448;
    (t1128, t1129, t1131, t1134, t1137, t1141, t1146, t1147, t1148, t1150, t1153, t1156)
}
