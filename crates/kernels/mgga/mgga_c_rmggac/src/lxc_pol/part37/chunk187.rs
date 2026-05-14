//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 187/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk187<F: Float>(t140: F, t453: F, t73: F, t75: F, t80: F, t1007: F, t78: F, t76: F, t16: F, t239: F, t252: F, t954: F, t957: F, t960: F, t240: F, t20: F, t259: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1231 = t453 * t140;
    let t1279 = t75 * t73;
    let t1281 = 132.0 * t1279 * t80;
    let t1284 = t78 * t1007;
    let t1285 = 1.0 / t1284;
    let t1287 = 156.0 * t76 * t1285;
    let t1294 = t239 * t16;
    let t1295 = 1.0 / t1294;
    let t1296 = t252 * t252;
    let t1297 = t1295 * t1296;
    let t1302 = -0.49388888888888888889e-2 * t954 + 0.98777777777777777777e-2 * t957 + 0.13949e-1 * t960;
    let t1303 = t240 * t1302;
    let t1309 = t239 * t239;
    let t1310 = 1.0 / t1309;
    let t1311 = t1310 * t1296;
    let t1314 = t20 * t259;
    (t1231, t1281, t1287, t1295, t1296, t1297, t1302, t1303, t1310, t1311, t1314)
}
