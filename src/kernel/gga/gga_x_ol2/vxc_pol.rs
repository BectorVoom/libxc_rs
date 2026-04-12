//! GGA_X_OL2 vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ol2.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ol2_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_aa: f64,
    param_bb: f64,
    param_cc: f64,
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
        let t28 = param_bb * sigma0;
        let t29 = rho0 * rho0;
        let t30 = pow_1_3(rho0);
        let t31 = t30 * t30;
        let t33 = 1.0 / t31 / t29;
        let t36 = f64::sqrt(sigma0);
        let t37 = param_cc * t36;
        let t39 = 1.0 / t30 / rho0;
        let t40 = M_CBRT2;
        let t43 = 4.0 * t36 * t39 + t40;
        let t44 = 1.0 / t43;
        let t45 = t39 * t44;
        let t47 = param_aa + 0.13888888888888888889e-1 * t28 * t33 + t37 * t45;
        let t51 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t47);
        let t52 = rho1 <= dens_threshold;
        let t53 = -t16;
        let t55 = piecewise5(t14, t11, t10, t15, t53 * t7);
        let t56 = 1.0 + t55;
        let t57 = t56 <= zeta_threshold;
        let t58 = pow_1_3(t56);
        let t60 = piecewise3(t57, t22, t58 * t56);
        let t61 = t60 * t26;
        let t62 = param_bb * sigma2;
        let t63 = rho1 * rho1;
        let t64 = pow_1_3(rho1);
        let t65 = t64 * t64;
        let t67 = 1.0 / t65 / t63;
        let t70 = f64::sqrt(sigma2);
        let t71 = param_cc * t70;
        let t73 = 1.0 / t64 / rho1;
        let t76 = 4.0 * t70 * t73 + t40;
        let t77 = 1.0 / t76;
        let t78 = t73 * t77;
        let t80 = param_aa + 0.13888888888888888889e-1 * t62 * t67 + t71 * t78;
        let t84 = piecewise3(t52, 0.0, -3.0 / 8.0 * t5 * t61 * t80);
        let tzk0 = t51 + t84;
        zk[ip] += tzk0;
        let t85 = t6 * t6;
        let t86 = 1.0 / t85;
        let t87 = t16 * t86;
        let t89 = piecewise5(t10, 0.0, t14, 0.0, t7 - t87);
        let t92 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t89);
        let t93 = t92 * t26;
        let t97 = t26 * t26;
        let t98 = 1.0 / t97;
        let t99 = t25 * t98;
        let t102 = t5 * t99 * t47 / 8.0;
        let t103 = t29 * rho0;
        let t105 = 1.0 / t31 / t103;
        let t110 = 1.0 / t30 / t29 * t44;
        let t113 = param_cc * sigma0;
        let t114 = t43 * t43;
        let t115 = 1.0 / t114;
        let t116 = t105 * t115;
        let t119 = -0.37037037037037037037e-1 * t28 * t105 - 4.0 / 3.0 * t37 * t110 + 16.0 / 3.0 * t113 * t116;
        let t124 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t93 * t47 - t102 - 3.0 / 8.0 * t5 * t27 * t119);
        let t125 = t53 * t86;
        let t127 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t125);
        let t130 = piecewise3(t57, 0.0, 4.0 / 3.0 * t58 * t127);
        let t131 = t130 * t26;
        let t135 = t60 * t98;
        let t138 = t5 * t135 * t80 / 8.0;
        let t140 = piecewise3(t52, 0.0, -3.0 / 8.0 * t5 * t131 * t80 - t138);
        let tvrho0 = t51 + t84 + t6 * (t124 + t140);
        vrho[ip * 2] += tvrho0;
        let t144 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t87);
        let t147 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t144);
        let t148 = t147 * t26;
        let t153 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t148 * t47 - t102);
        let t155 = piecewise5(t14, 0.0, t10, 0.0, t7 - t125);
        let t158 = piecewise3(t57, 0.0, 4.0 / 3.0 * t58 * t155);
        let t159 = t158 * t26;
        let t163 = t63 * rho1;
        let t165 = 1.0 / t65 / t163;
        let t170 = 1.0 / t64 / t63 * t77;
        let t173 = param_cc * sigma2;
        let t174 = t76 * t76;
        let t175 = 1.0 / t174;
        let t176 = t165 * t175;
        let t179 = -0.37037037037037037037e-1 * t62 * t165 - 4.0 / 3.0 * t71 * t170 + 16.0 / 3.0 * t173 * t176;
        let t184 = piecewise3(t52, 0.0, -3.0 / 8.0 * t5 * t159 * t80 - t138 - 3.0 / 8.0 * t5 * t61 * t179);
        let tvrho1 = t51 + t84 + t6 * (t153 + t184);
        vrho[ip * 2 + 1] += tvrho1;
        let t189 = 1.0 / t36;
        let t190 = param_cc * t189;
        let t196 = 0.13888888888888888889e-1 * param_bb * t33 + t190 * t45 / 2.0 - 2.0 * param_cc * t33 * t115;
        let t200 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t196);
        let tvsigma0 = t6 * t200;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t203 = 1.0 / t70;
        let t204 = param_cc * t203;
        let t210 = 0.13888888888888888889e-1 * param_bb * t67 + t204 * t78 / 2.0 - 2.0 * param_cc * t67 * t175;
        let t214 = piecewise3(t52, 0.0, -3.0 / 8.0 * t5 * t61 * t210);
        let tvsigma2 = t6 * t214;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
