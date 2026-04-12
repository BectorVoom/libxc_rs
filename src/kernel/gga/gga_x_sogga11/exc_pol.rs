//! GGA_X_SOGGA11 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sogga11.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_sogga11_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_kappa: f64,
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
        let t28 = param_a_0;
        let t29 = param_a_1;
        let t30 = M_CBRT6;
        let t31 = param_mu * t30;
        let t32 = M_PI * M_PI;
        let t33 = pow_1_3(t32);
        let t34 = t33 * t33;
        let t35 = 1.0 / t34;
        let t36 = t31 * t35;
        let t37 = 1.0 / param_kappa;
        let t38 = t37 * sigma0;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t46 = t36 * t38 * t43 / 24.0;
        let t47 = 1.0 + t46;
        let t49 = 1.0 - 1.0 / t47;
        let t51 = param_a_2;
        let t52 = t49 * t49;
        let t54 = param_a_3;
        let t55 = t52 * t49;
        let t57 = param_a_4;
        let t58 = t52 * t52;
        let t60 = param_a_5;
        let t63 = param_b_0;
        let t64 = param_b_1;
        let t65 = f64::exp(-t46);
        let t66 = 1.0 - t65;
        let t68 = param_b_2;
        let t69 = t66 * t66;
        let t71 = param_b_3;
        let t72 = t69 * t66;
        let t74 = param_b_4;
        let t75 = t69 * t69;
        let t77 = param_b_5;
        let t80 = t60 * t58 * t49 + t77 * t75 * t66 + t29 * t49 + t51 * t52 + t54 * t55 + t57 * t58 + t64 * t66 + t68 * t69 + t71 * t72 + t74 * t75 + t28 + t63;
        let t84 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t80);
        let t85 = rho1 <= dens_threshold;
        let t86 = -t16;
        let t88 = piecewise5(t14, t11, t10, t15, t86 * t7);
        let t89 = 1.0 + t88;
        let t90 = t89 <= zeta_threshold;
        let t91 = pow_1_3(t89);
        let t93 = piecewise3(t90, t22, t91 * t89);
        let t94 = t93 * t26;
        let t95 = t37 * sigma2;
        let t96 = rho1 * rho1;
        let t97 = pow_1_3(rho1);
        let t98 = t97 * t97;
        let t100 = 1.0 / t98 / t96;
        let t103 = t36 * t95 * t100 / 24.0;
        let t104 = 1.0 + t103;
        let t106 = 1.0 - 1.0 / t104;
        let t108 = t106 * t106;
        let t110 = t108 * t106;
        let t112 = t108 * t108;
        let t116 = f64::exp(-t103);
        let t117 = 1.0 - t116;
        let t119 = t117 * t117;
        let t121 = t119 * t117;
        let t123 = t119 * t119;
        let t127 = t60 * t112 * t106 + t77 * t123 * t117 + t29 * t106 + t51 * t108 + t54 * t110 + t57 * t112 + t64 * t117 + t68 * t119 + t71 * t121 + t74 * t123 + t28 + t63;
        let t131 = piecewise3(t85, 0.0, -3.0 / 8.0 * t5 * t94 * t127);
        let tzk0 = t84 + t131;
        zk[ip] += tzk0;
    }
}
