//! GGA_X_SFAT exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_sfat_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t17 / t4 * t3;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t3 * t3;
        let t22 = 1.0 / M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t26 = t25 * t24;
        let t27 = t24 * t20;
        let t28 = t25 * t27;
        let t29 = M_CBRT2;
        let t30 = t29 * t29;
        let t31 = t30 * sigma[ip];
        let t32 = rho[ip] * rho[ip];
        let t33 = t19 * t19;
        let t35 = 1.0 / t33 / t32;
        let t36 = f64::sqrt(sigma[ip]);
        let t37 = t29 * t36;
        let t39 = 1.0 / t19 / rho[ip];
        let t41 = f64::ln(t39 * t37 + f64::sqrt(pow_2(t39 * t37) + 1.0));
        let t42 = t41 * t39;
        let t45 = 1.0 + 0.252e-1 * t42 * t37;
        let t46 = 1.0 / t45;
        let t51 = 1.0 + 0.93333333333333333332e-3 * t46 * t35 * t31 * t28;
        let t54 = 1.0 / t51 * t26 * t20 * M_PI;
        let t55 = f64::sqrt(t54);
        let t57 = 1.0 / t55 * param_hyb_omega_0;
        let t58 = rho[ip] * t11;
        let t59 = pow_1_3(t58);
        let t60 = 1.0 / t59;
        let t61 = t60 * t29;
        let t63 = t61 * t57 / 2.0;
        let t64 = 0.192e1 <= t63;
        let t65 = 0.192e1 < t63;
        let t66 = piecewise3(t65, t63, 0.192e1);
        let t67 = t66 * t66;
        let t68 = t67 * t67;
        let t69 = t68 * t68;
        let t70 = t69 * t69;
        let t71 = t70 * t70;
        let t73 = 1.0 / t71 / t67;
        let t76 = 1.0 / t71 / t68;
        let t78 = 1.0 / t68;
        let t80 = t68 * t67;
        let t81 = 1.0 / t80;
        let t83 = 1.0 / t69;
        let t85 = t69 * t67;
        let t86 = 1.0 / t85;
        let t88 = t69 * t68;
        let t89 = 1.0 / t88;
        let t91 = t69 * t80;
        let t92 = 1.0 / t91;
        let t94 = 1.0 / t70;
        let t97 = 1.0 / t70 / t67;
        let t100 = 1.0 / t70 / t68;
        let t103 = 1.0 / t70 / t80;
        let t106 = 1.0 / t70 / t69;
        let t109 = 1.0 / t70 / t85;
        let t112 = 1.0 / t70 / t88;
        let t115 = 1.0 / t70 / t91;
        let t117 = 1.0 / t71;
        let t121 = t73 / 5985.0 - t76 / 7030.0 - t78 / 30.0 + t81 / 70.0 - t83 / 135.0 + t86 / 231.0 - t89 / 364.0 + t92 / 540.0 - t94 / 765.0 + t97 / 1045.0 - t100 / 1386.0 + t103 / 1794.0 - t106 / 2275.0 + t109 / 2835.0 - t112 / 3480.0 + t115 / 4216.0 - t117 / 5049.0 + 1.0 / t67 / 9.0;
        let t122 = piecewise3(t65, 0.192e1, t63);
        let t123 = f64::atan2(1.0, t122);
        let t124 = t122 * t122;
        let t125 = t124 + 3.0;
        let t126 = 1.0 / t124;
        let t127 = 1.0 + t126;
        let t128 = f64::ln(t127);
        let t130 = -t125 * t128 + 1.0;
        let t133 = t123 + t130 * t122 / 4.0;
        let t137 = piecewise3(t64, t121, 1.0 - 8.0 / 3.0 * t133 * t122);
        let t138 = t137 * t19;
        let t142 = piecewise3(t2, 0.0, -3.0 / 8.0 * t51 * t138 * t18);
        let tzk0 = 2.0 * t142;
        zk[ip] += tzk0;
    }
}
