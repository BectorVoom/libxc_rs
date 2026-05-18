//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 244/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk244<F: Float>(t1136: F, t1137: F, t1086: F, t1092: F, t449: F, t445: F, t440: F, t1111: F, t1103: F, t1108: F, t1115: F, t448: F) -> (F, F, F, F, F, F, F, F) {
    let t1138 = t1136 * t1137;
    let t1141 = F::new(0.92708333333333333333e-2) * t1086;
    let t1143 = -t1141 + F::new(0.92708333333333333333e-2) * t1092;
    let t1144 = t1143 * t449;
    let t1146 = t445 * t445;
    let t1147 = F::new(1.0) / t1146;
    let t1148 = t440 * t1147;
    let t1150 = F::new(0.301925e0) * t1086;
    let t1153 = F::new(0.82785e-1) * t1111;
    let t1155 = F::new(0.258925e1) * t1103 - t1150 + F::new(0.301925e0) * t1092 + F::new(0.16504875e0) * t1108 - t1153 + F::new(0.82785e-1) * t1115;
    let t1156 = F::new(1.0) / t448;
    (t1138, t1143, t1144, t1146, t1147, t1148, t1155, t1156)
}
