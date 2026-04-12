//! GGA_C_WL exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wl.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_wl_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
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
        let t1 = rho0 - rho1;
        let t2 = t1 * t1;
        let t3 = rho0 + rho1;
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = -t2 * t5 + 1.0;
        let t8 = f64::sqrt(t7);
        let t10 = sigma0 + 2.0 * sigma1 + sigma2;
        let t11 = f64::sqrt(t10);
        let t12 = pow_1_3(t3);
        let t14 = 1.0 / t12 / t3;
        let t17 = -0.7486e0 + 0.6001e-1 * t11 * t14;
        let t18 = t8 * t17;
        let t19 = f64::sqrt(sigma0);
        let t20 = pow_1_3(rho0);
        let t22 = 1.0 / t20 / rho0;
        let t25 = f64::sqrt(sigma2);
        let t26 = pow_1_3(rho1);
        let t28 = 1.0 / t26 / rho1;
        let t31 = M_CBRT3;
        let t33 = pow_1_3(1.0 / M_PI);
        let t34 = t31 * t33;
        let t35 = M_CBRT4;
        let t36 = t35 * t35;
        let t37 = 1.0 / t12;
        let t41 = 0.360073e1 + 0.9e0 * t19 * t22 + 0.9e0 * t25 * t28 + t34 * t36 * t37 / 4.0;
        let t42 = 1.0 / t41;
        let tzk0 = t18 * t42;
        zk[ip] += tzk0;
    }
}
