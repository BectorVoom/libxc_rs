//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 875/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk875<F: Float>(t11189: F, t409: F, t1117: F, t3265: F, t3315: F, t11135: F, t1102: F, t3270: F, t3279: F, t3287: F, t10292: F, t281: F, t415: F) -> (F, F, F, F, F, F) {
    let t11190 = t409 * t11189;
    let t11191 = t3265 * t1117;
    let t11192 = t11191 * t3315;
    let t11194 = F::cast_from(0.96491876992155210402e2_f64) * t11190 * t11192;
    let t11195 = F::cast_from(0.93011851851851851854e0_f64) * t11135;
    let t11197 = t3270 * t1102 * t3279;
    let t11200 = t3287 * t1102 * t3279;
    let t11203 = t281 * t10292 * t415;
    (t11191, t11194, t11195, t11197, t11200, t11203)
}
