//! GGA_C_CHACHIYO exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_chachiyo.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_chachiyo_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    param_h: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t3 = param_bp * t2;
        let t5 = pow_1_3(1.0 / M_PI);
        let t7 = M_CBRT4;
        let t8 = 1.0 / t5 * t7;
        let t9 = pow_1_3(rho[ip]);
        let t10 = t8 * t9;
        let t13 = param_cp * t1;
        let t14 = t5 * t5;
        let t16 = t7 * t7;
        let t17 = 1.0 / t14 * t16;
        let t18 = t9 * t9;
        let t19 = t17 * t18;
        let t22 = 1.0 + t3 * t10 / 3.0 + t13 * t19 / 3.0;
        let t23 = f64::ln(t22);
        let t24 = param_ap * t23;
        let t25 = param_bf * t2;
        let t28 = param_cf * t1;
        let t31 = 1.0 + t25 * t10 / 3.0 + t28 * t19 / 3.0;
        let t32 = f64::ln(t31);
        let t36 = pow_1_3(zeta_threshold);
        let t37 = t36 * t36;
        let t38 = piecewise3(1.0 <= zeta_threshold, t37, 1.0);
        let t39 = t38 * t38;
        let t42 = -2.0 * t39 * t38 + 2.0;
        let t44 = t24 + (param_af * t32 - t24) * t42;
        let t45 = M_CBRTPI;
        let t46 = t2 * t45;
        let t47 = rho[ip] * rho[ip];
        let t49 = 1.0 / t9 / t47;
        let t53 = 1.0 + t46 * t49 * sigma[ip] / 48.0;
        let t54 = 1.0 / t44;
        let t55 = param_h * t54;
        let t56 = f64::powf(t53, t55);
        let tzk0 = t44 * t56;
        zk[ip] += tzk0;
    }
}
