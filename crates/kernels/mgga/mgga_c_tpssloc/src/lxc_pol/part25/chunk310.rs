//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 310/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk310<F: Float>(t1088: F, t1090: F, t123: F, t1087: F, t423: F, t419: F, t409: F, t410: F, t1086: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1091 = t1088 * t1090;
    let t1092 = t123 * t1091;
    let t1094 = -t1087 + F::cast_from(0.17808333333333333333e-1_f64) * t1092;
    let t1096 = F::new(0.621814e-1) * t1094 * t423;
    let t1097 = t419 * t419;
    let t1098 = F::new(1.0) / t1097;
    let t1099 = t409 * t1098;
    let t1100 = F::new(1.0) / t410;
    let t1102 = -t1086 / F::new(3.0) + t1092 / F::new(3.0);
    (t1091, t1092, t1094, t1096, t1097, t1098, t1099, t1100, t1102)
}
