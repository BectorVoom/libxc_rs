//! GGA_X_B86 fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_b86.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_b86_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_beta: f64,
    param_gamma: f64,
    param_omega: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = param_beta * sigma[ip];
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = rho[ip] * rho[ip];
        let t24 = t18 * t18;
        let t26 = 1.0 / t24 / t23;
        let t27 = t22 * t26;
        let t30 = param_gamma * sigma[ip] * t27 + 1.0;
        let t31 = f64::powf(t30, param_omega);
        let t32 = 1.0 / t31;
        let t35 = t20 * t27 * t32 + 1.0;
        let t39 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t35);
        let tzk0 = 2.0 * t39;
        zk[ip] += tzk0;
        let t41 = t17 / t24;
        let t45 = t23 * rho[ip];
        let t47 = 1.0 / t24 / t45;
        let t52 = sigma[ip] * sigma[ip];
        let t53 = param_beta * t52;
        let t54 = t23 * t23;
        let t55 = t54 * t23;
        let t57 = 1.0 / t18 / t55;
        let t60 = t32 * param_omega;
        let t61 = 1.0 / t30;
        let t63 = t60 * param_gamma * t61;
        let t66 = -8.0 / 3.0 * t20 * t22 * t47 * t32 + 16.0 / 3.0 * t53 * t21 * t57 * t63;
        let t71 = piecewise3(t2, 0.0, -t6 * t41 * t35 / 8.0 - 3.0 / 8.0 * t6 * t19 * t66);
        let tvrho0 = 2.0 * rho[ip] * t71 + 2.0 * t39;
        vrho[ip] += tvrho0;
        let t74 = param_beta * t22;
        let t77 = t54 * rho[ip];
        let t79 = 1.0 / t18 / t77;
        let t84 = -2.0 * t20 * t21 * t79 * t63 + t74 * t26 * t32;
        let t88 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t84);
        let tvsigma0 = 2.0 * rho[ip] * t88;
        vsigma[ip] += tvsigma0;
        let t93 = t17 / t24 / rho[ip];
        let t101 = 1.0 / t24 / t54;
        let t106 = t54 * t45;
        let t108 = 1.0 / t18 / t106;
        let t113 = t52 * sigma[ip];
        let t114 = param_beta * t113;
        let t115 = t54 * t54;
        let t116 = t115 * t23;
        let t117 = 1.0 / t116;
        let t118 = t114 * t117;
        let t119 = param_omega * param_omega;
        let t120 = t32 * t119;
        let t121 = param_gamma * param_gamma;
        let t122 = t30 * t30;
        let t123 = 1.0 / t122;
        let t124 = t121 * t123;
        let t125 = t120 * t124;
        let t128 = t60 * t124;
        let t131 = 88.0 / 9.0 * t20 * t22 * t101 * t32 - 48.0 * t53 * t21 * t108 * t63 + 256.0 / 9.0 * t118 * t125 + 256.0 / 9.0 * t118 * t128;
        let t136 = piecewise3(t2, 0.0, t6 * t93 * t35 / 12.0 - t6 * t41 * t66 / 4.0 - 3.0 / 8.0 * t6 * t19 * t131);
        let tv2rho20 = 2.0 * rho[ip] * t136 + 4.0 * t71;
        v2rho2[ip] += tv2rho20;
        let t145 = param_beta * t21;
        let t150 = param_omega * param_gamma * sigma[ip] * t61;
        let t153 = t115 * rho[ip];
        let t154 = 1.0 / t153;
        let t155 = t53 * t154;
        let t160 = -8.0 / 3.0 * t74 * t47 * t32 + 16.0 * t145 * t57 * t32 * t150 - 32.0 / 3.0 * t155 * t125 - 32.0 / 3.0 * t155 * t128;
        let t165 = piecewise3(t2, 0.0, -t6 * t41 * t84 / 8.0 - 3.0 / 8.0 * t6 * t19 * t160);
        let tv2rhosigma0 = 2.0 * rho[ip] * t165 + 2.0 * t88;
        v2rhosigma[ip] += tv2rhosigma0;
        let t170 = 1.0 / t115;
        let t171 = t20 * t170;
        let t175 = -4.0 * t145 * t79 * t63 + 4.0 * t171 * t125 + 4.0 * t171 * t128;
        let t179 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t175);
        let tv2sigma20 = 2.0 * rho[ip] * t179;
        v2sigma2[ip] += tv2sigma20;
    }
}
