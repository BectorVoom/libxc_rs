//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 281/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk281<F: Float>(t1088: F, t1090: F, t123: F, t1087: F, t423: F, t419: F, t409: F, t410: F, t1086: F, t407: F, t281: F, t415: F, t904: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1091 = t1088 * t1090;
    let t1092 = t123 * t1091;
    let t1094 = -t1087 + 0.17808333333333333333e-1 * t1092;
    let t1096 = 0.621814e-1 * t1094 * t423;
    let t1097 = t419 * t419;
    let t1098 = 1.0 / t1097;
    let t1099 = t409 * t1098;
    let t1100 = 1.0 / t410;
    let t1102 = -t1086 / 3.0 + t1092 / 3.0;
    let t1103 = t1100 * t1102;
    let t1105 = 0.29896666666666666667e0 * t1086;
    let t1107 = f64::sqrt(t407);
    let t1108 = t1107 * t1102;
    let t1111 = t281 * t904 * t415;
    (t1091, t1092, t1094, t1096, t1097, t1098, t1099, t1100, t1102, t1103, t1105, t1107, t1108, t1111)
}
