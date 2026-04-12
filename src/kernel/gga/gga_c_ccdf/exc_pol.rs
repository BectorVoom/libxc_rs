//! GGA_C_CCDF exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_ccdf.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT6, M_PI};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_ccdf_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
    param_c4: f64,
    param_c5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t3 = 1.0 / t2;
        let t5 = param_c2 * t3 + 1.0;
        let t6 = 1.0 / t5;
        let t7 = param_c1 * t6;
        let t8 = M_CBRT2;
        let t9 = M_CBRT6;
        let t10 = t9 * t9;
        let t11 = t8 * t10;
        let t12 = M_PI * M_PI;
        let t13 = pow_1_3(t12);
        let t14 = 1.0 / t13;
        let t16 = sigma0 + 2.0 * sigma1 + sigma2;
        let t17 = f64::sqrt(t16);
        let t18 = t14 * t17;
        let t20 = 1.0 / t2 / t1;
        let t26 = f64::exp(-param_c4 * (t11 * t18 * t20 / 12.0 - param_c5));
        let t27 = 1.0 + t26;
        let t30 = 1.0 - param_c3 / t27;
        let tzk0 = t7 * t30;
        zk[ip] += tzk0;
    }
}
