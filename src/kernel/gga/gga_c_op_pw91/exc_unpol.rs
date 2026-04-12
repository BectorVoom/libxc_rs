//! GGA_C_OP_PW91 exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_pw91.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_op_pw91_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = 1.0 <= zeta_threshold;
        let t4 = t1 || rho[ip] / 2.0 <= dens_threshold;
        let t5 = zeta_threshold - 1.0;
        let t6 = -t5;
        let t7 = piecewise5(t1, t5, t1, t6, 0.0);
        let t8 = t7 * t7;
        let t9 = 1.0 - t8;
        let t10 = t9 * rho[ip];
        let t11 = 1.0 + t7;
        let t14 = t11 * rho[ip] / 2.0 <= dens_threshold;
        let t15 = M_CBRT3;
        let t16 = t15 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t20 = t16 / t18;
        let t21 = M_CBRT4;
        let t22 = t20 * t21;
        let t23 = M_CBRT2;
        let t24 = t11 <= zeta_threshold;
        let t25 = 1.0 - t7;
        let t26 = t25 <= zeta_threshold;
        let t27 = piecewise5(t24, t5, t26, t6, t7);
        let t28 = 1.0 + t27;
        let t29 = t28 * rho[ip];
        let t30 = pow_1_3(t29);
        let t31 = 1.0 / t30;
        let t32 = t23 * t31;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = t23 * t23;
        let t40 = sigma[ip] * t39;
        let t41 = rho[ip] * rho[ip];
        let t42 = pow_1_3(rho[ip]);
        let t43 = t42 * t42;
        let t45 = 1.0 / t43 / t41;
        let t46 = t40 * t45;
        let t47 = t38 * t46;
        let t49 = f64::exp(-25.0 / 6.0 * t47);
        let t52 = (0.2743e0 - 0.1508e0 * t49) * t33;
        let t53 = t52 * t37;
        let t56 = t33 * t33;
        let t58 = 1.0 / t35 / t34;
        let t59 = t56 * t58;
        let t60 = sigma[ip] * sigma[ip];
        let t61 = t60 * t23;
        let t62 = t41 * t41;
        let t63 = t62 * rho[ip];
        let t65 = 1.0 / t42 / t63;
        let t68 = 0.13888888888888888889e-4 * t59 * t61 * t65;
        let t69 = t53 * t46 / 24.0 - t68;
        let t71 = t56 / t35;
        let t72 = f64::sqrt(sigma[ip]);
        let t73 = t71 * t72;
        let t75 = 1.0 / t42 / rho[ip];
        let t81 = f64::ln(0.64963333333333333333e0 * t71 * t72 * t23 * t75 + f64::sqrt(pow_2(0.64963333333333333333e0 * t71 * t72 * t23 * t75) + 1.0));
        let t82 = t23 * t75 * t81;
        let t85 = 1.0 + 0.16370833333333333333e-1 * t73 * t82 + t68;
        let t86 = 1.0 / t85;
        let t88 = t69 * t86 + 1.0;
        let t89 = 1.0 / t88;
        let t93 = piecewise3(t14, 0.0, t22 * t32 * t89 / 9.0);
        let t97 = t25 * rho[ip] / 2.0 <= dens_threshold;
        let t98 = piecewise5(t26, t5, t24, t6, -t7);
        let t99 = 1.0 + t98;
        let t100 = t99 * rho[ip];
        let t101 = pow_1_3(t100);
        let t102 = 1.0 / t101;
        let t103 = t23 * t102;
        let t107 = piecewise3(t97, 0.0, t22 * t103 * t89 / 9.0);
        let t108 = t93 + t107;
        let t109 = t108 == 0.0;
        let t110 = piecewise3(t109, f64::EPSILON, t108);
        let t113 = 0.360663084e1 / t110 + 0.5764e0;
        let t114 = t110 * t110;
        let t115 = t114 * t114;
        let t116 = 1.0 / t115;
        let t118 = t114 * t110;
        let t119 = 1.0 / t118;
        let t121 = 1.0 / t114;
        let t123 = 0.315815266717518096e2 * t116 + 0.150327320916243744e2 * t119 + 0.1788764629788e1 * t121;
        let t124 = 1.0 / t123;
        let tzk0 = piecewise3(t4, 0.0, -0.25e0 * t10 * t113 * t124);
        zk[ip] += tzk0;
    }
}
