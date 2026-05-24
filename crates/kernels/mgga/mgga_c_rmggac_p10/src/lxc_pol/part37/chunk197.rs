//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 197/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk197<F: Float>(t1284: F, t76: F, t16: F, t239: F, t252: F, t954: F, t957: F, t960: F, t240: F, t20: F, t259: F, t253: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1285 = F::new(1.0) / t1284;
    let t1287 = F::new(156.0) * t76 * t1285;
    let t1294 = t239 * t16;
    let t1295 = F::new(1.0) / t1294;
    let t1296 = t252 * t252;
    let t1297 = t1295 * t1296;
    let t1302 = -F::cast_from(0.49388888888888888889e-2_f64) * t954 + F::cast_from(0.98777777777777777777e-2_f64) * t957 + F::new(0.13949e-1) * t960;
    let t1303 = t240 * t1302;
    let t1309 = t239 * t239;
    let t1310 = F::new(1.0) / t1309;
    let t1311 = t1310 * t1296;
    let t1314 = t20 * t259;
    let t1315 = t253 * t1314;
    (t1287, t1295, t1296, t1297, t1302, t1303, t1310, t1311, t1315)
}
