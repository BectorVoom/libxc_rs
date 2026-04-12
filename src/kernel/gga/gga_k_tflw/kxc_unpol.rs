//! GGA_K_TFLW kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_tflw.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_tflw_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    param_gamma: f64,
    param_lambda: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t24 = param_lambda * sigma[ip];
        let t25 = M_CBRT2;
        let t26 = t25 * t25;
        let t28 = rho[ip] * rho[ip];
        let t31 = M_CBRT6;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t36 = 1.0 / t35;
        let t40 = param_gamma + 5.0 / 72.0 * t24 * t26 / t22 / t28 * t31 * t36;
        let t44 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t20 * t22 * t40);
        let tzk0 = 2.0 * t44;
        zk[ip] += tzk0;
        let t50 = t28 * rho[ip];
        let t53 = t7 * t20 / t50;
        let t56 = t24 * t26 * t31 * t36;
        let t60 = piecewise3(t2, 0.0, t7 * t20 / t21 * t40 / 10.0 - t53 * t56 / 36.0);
        let tvrho0 = 2.0 * rho[ip] * t60 + 2.0 * t44;
        vrho[ip] += tvrho0;
        let t68 = param_lambda * t26 * t31 * t36;
        let t71 = piecewise3(t2, 0.0, t7 * t20 / t28 * t68 / 96.0);
        let tvsigma0 = 2.0 * rho[ip] * t71;
        vsigma[ip] += tvsigma0;
        let t80 = t28 * t28;
        let t83 = t7 * t20 / t80;
        let t87 = piecewise3(t2, 0.0, -t7 * t20 / t21 / rho[ip] * t40 / 30.0 + 7.0 / 108.0 * t83 * t56);
        let tv2rho20 = 2.0 * rho[ip] * t87 + 4.0 * t60;
        v2rho2[ip] += tv2rho20;
        let t92 = piecewise3(t2, 0.0, -t53 * t68 / 48.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t92 + 2.0 * t71;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let t105 = t7 * t20 / t80 / rho[ip];
        let t109 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t20 / t21 / t28 * t40 - 41.0 / 162.0 * t105 * t56);
        let tv3rho30 = 2.0 * rho[ip] * t109 + 6.0 * t87;
        v3rho3[ip] += tv3rho30;
        let t115 = piecewise3(t2, 0.0, t83 * t68 / 16.0);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t115 + 4.0 * t92;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rhosigma20 = 0.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3sigma30 = 0.0;
        v3sigma3[ip] += tv3sigma30;
    }
}
