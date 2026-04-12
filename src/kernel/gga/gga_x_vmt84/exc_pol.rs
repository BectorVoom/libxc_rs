//! GGA_X_VMT84 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_vmt84.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_vmt84_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_alpha: f64,
    param_mu: f64,
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
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_mu * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t38 = t37 * t35;
        let t39 = 1.0 / t38;
        let t41 = param_alpha * t28;
        let t42 = t33 * sigma0;
        let t43 = t42 * t39;
        let t46 = f64::exp(-t41 * t43 / 24.0);
        let t49 = 1.0 + t29 * t43 / 24.0;
        let t50 = 1.0 / t49;
        let t51 = t46 * t50;
        let t55 = t28 * t28;
        let t56 = param_alpha * t55;
        let t58 = 1.0 / t31 / t30;
        let t59 = sigma0 * sigma0;
        let t60 = t58 * t59;
        let t61 = t35 * t35;
        let t62 = t61 * rho0;
        let t64 = 1.0 / t36 / t62;
        let t68 = f64::exp(-t56 * t60 * t64 / 576.0);
        let t70 = (1.0 - t68) * t55;
        let t71 = 1.0 / sigma0;
        let t72 = t32 * t71;
        let t76 = t34 * sigma0 * t39 * t51 / 24.0 + 4.0 * t70 * t72 * t38 + t68;
        let t80 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t76);
        let t81 = rho1 <= dens_threshold;
        let t82 = -t16;
        let t84 = piecewise5(t14, t11, t10, t15, t82 * t7);
        let t85 = 1.0 + t84;
        let t86 = t85 <= zeta_threshold;
        let t87 = pow_1_3(t85);
        let t89 = piecewise3(t86, t22, t87 * t85);
        let t90 = t89 * t26;
        let t91 = rho1 * rho1;
        let t92 = pow_1_3(rho1);
        let t93 = t92 * t92;
        let t94 = t93 * t91;
        let t95 = 1.0 / t94;
        let t97 = t33 * sigma2;
        let t98 = t97 * t95;
        let t101 = f64::exp(-t41 * t98 / 24.0);
        let t104 = 1.0 + t29 * t98 / 24.0;
        let t105 = 1.0 / t104;
        let t106 = t101 * t105;
        let t110 = sigma2 * sigma2;
        let t111 = t58 * t110;
        let t112 = t91 * t91;
        let t113 = t112 * rho1;
        let t115 = 1.0 / t92 / t113;
        let t119 = f64::exp(-t56 * t111 * t115 / 576.0);
        let t121 = (1.0 - t119) * t55;
        let t122 = 1.0 / sigma2;
        let t123 = t32 * t122;
        let t127 = t34 * sigma2 * t95 * t106 / 24.0 + 4.0 * t121 * t123 * t94 + t119;
        let t131 = piecewise3(t81, 0.0, -3.0 / 8.0 * t5 * t90 * t127);
        let tzk0 = t80 + t131;
        zk[ip] += tzk0;
    }
}
