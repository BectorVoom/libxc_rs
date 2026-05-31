//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 248/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk248<F: Float>(t1302: F, t240: F, t239: F, t1296: F, t20: F, t259: F, t253: F, t40: F, t41: F, t21: F, t22: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1303 = t240 * t1302;
    let t1309 = t239 * t239;
    let t1310 = F::cast_from(1.0_f64) / t1309;
    let t1311 = t1310 * t1296;
    let t1314 = t20 * t259;
    let t1315 = t253 * t1314;
    let t1318 = t40 * t40;
    let t1320 = F::cast_from(1.0_f64) / t41 / t1318;
    let t1321 = t21 * t1320;
    let t1322 = t22 * t22;
    let t1323 = t1321 * t1322;
    (t1303, t1309, t1310, t1311, t1314, t1315, t1318, t1320, t1321, t1322, t1323)
}
