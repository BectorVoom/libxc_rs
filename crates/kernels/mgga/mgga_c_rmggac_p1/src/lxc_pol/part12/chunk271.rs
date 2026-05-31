//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 271/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk271<F: Float>(t1288: F, t131: F, t302: F, t356: F, t16: F, t239: F, t252: F, t954: F, t957: F, t960: F) -> (F, F, F, F, F, F) {
    let t1289 = t1288 * t131;
    let t1291 = t302 * t356;
    let t1294 = t239 * t16;
    let t1295 = F::cast_from(1.0_f64) / t1294;
    let t1296 = t252 * t252;
    let t1297 = t1295 * t1296;
    let t1302 = -F::cast_from(0.49388888888888888889e-2_f64) * t954 + F::cast_from(0.98777777777777777777e-2_f64) * t957 + F::cast_from(0.13949e-1_f64) * t960;
    (t1289, t1291, t1295, t1296, t1297, t1302)
}
