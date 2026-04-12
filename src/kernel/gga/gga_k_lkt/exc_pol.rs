//! GGA_K_LKT exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lkt.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_lkt_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
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
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_CBRT6;
        let t33 = t32 * t32;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t37 = t33 / t35;
        let t38 = f64::sqrt(sigma0);
        let t39 = pow_1_3(rho0);
        let t41 = 1.0 / t39 / rho0;
        let t44 = t37 * t38 * t41 / 12.0;
        let t45 = t44 < 200.0;
        let t46 = piecewise3(t45, t44, 200.0);
        let t47 = param_a * t46;
        let t48 = f64::cosh(t47);
        let t49 = 1.0 / t48;
        let t50 = t35 * t35;
        let t52 = t32 / t50;
        let t53 = rho0 * rho0;
        let t54 = t39 * t39;
        let t56 = 1.0 / t54 / t53;
        let t60 = t49 + 5.0 / 72.0 * t52 * sigma0 * t56;
        let t64 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t60);
        let t65 = rho1 <= dens_threshold;
        let t66 = -t17;
        let t68 = piecewise5(t15, t12, t11, t16, t66 * t8);
        let t69 = 1.0 + t68;
        let t70 = t69 <= zeta_threshold;
        let t71 = pow_1_3(t69);
        let t72 = t71 * t71;
        let t74 = piecewise3(t70, t24, t72 * t69);
        let t75 = t74 * t30;
        let t76 = f64::sqrt(sigma2);
        let t77 = pow_1_3(rho1);
        let t79 = 1.0 / t77 / rho1;
        let t82 = t37 * t76 * t79 / 12.0;
        let t83 = t82 < 200.0;
        let t84 = piecewise3(t83, t82, 200.0);
        let t85 = param_a * t84;
        let t86 = f64::cosh(t85);
        let t87 = 1.0 / t86;
        let t88 = rho1 * rho1;
        let t89 = t77 * t77;
        let t91 = 1.0 / t89 / t88;
        let t95 = t87 + 5.0 / 72.0 * t52 * sigma2 * t91;
        let t99 = piecewise3(t65, 0.0, 3.0 / 20.0 * t6 * t75 * t95);
        let tzk0 = t64 + t99;
        zk[ip] += tzk0;
    }
}
