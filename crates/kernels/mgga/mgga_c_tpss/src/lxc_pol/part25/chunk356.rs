//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 356/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk356<F: Float>(t1186: F, t489: F, t1183: F, t187: F, t497: F, t72: F, t732: F, t177: F) -> (F, F, F, F, F) {
    let t1187 = t489 * t1186;
    let t1189 = F::cast_from(0.19751673498613801407e-1_f64) * t1183 * t187;
    let t1190 = t497 * t72;
    let t1192 = F::cast_from(0.18311447306006545054e-3_f64) * t1190 * t732;
    let t1193 = t497 * t177;
    (t1187, t1189, t1190, t1192, t1193)
}
