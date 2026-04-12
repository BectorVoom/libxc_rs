//! GGA_X_WC vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_wc.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_wc_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
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
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t39 = sigma0 * t38;
        let t40 = t33 * t39;
        let t43 = f64::exp(-t40 / 24.0);
        let t47 = t28 * t28;
        let t49 = 1.0 / t30 / t29;
        let t50 = t47 * t49;
        let t51 = sigma0 * sigma0;
        let t52 = t34 * t34;
        let t53 = t52 * rho0;
        let t55 = 1.0 / t35 / t53;
        let t59 = 1.0 + 0.13780328706878157639e-4 * t50 * t51 * t55;
        let t60 = f64::ln(t59);
        let t61 = 0.804e0 + 5.0 / 972.0 * t40 + 0.4002424276710846245e-2 * t33 * t39 * t43 + t60;
        let t64 = 0.1804e1 - 0.646416e0 / t61;
        let t68 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t25 * t26 * t64);
        let t69 = rho1 <= dens_threshold;
        let t70 = -t16;
        let t72 = piecewise5(t14, t11, t10, t15, t70 * t7);
        let t73 = 1.0 + t72;
        let t74 = t73 <= zeta_threshold;
        let t75 = pow_1_3(t73);
        let t77 = piecewise3(t74, t22, t75 * t73);
        let t79 = rho1 * rho1;
        let t80 = pow_1_3(rho1);
        let t81 = t80 * t80;
        let t83 = 1.0 / t81 / t79;
        let t84 = sigma2 * t83;
        let t85 = t33 * t84;
        let t88 = f64::exp(-t85 / 24.0);
        let t92 = sigma2 * sigma2;
        let t93 = t79 * t79;
        let t94 = t93 * rho1;
        let t96 = 1.0 / t80 / t94;
        let t100 = 1.0 + 0.13780328706878157639e-4 * t50 * t92 * t96;
        let t101 = f64::ln(t100);
        let t102 = 0.804e0 + 5.0 / 972.0 * t85 + 0.4002424276710846245e-2 * t33 * t84 * t88 + t101;
        let t105 = 0.1804e1 - 0.646416e0 / t102;
        let t109 = piecewise3(t69, 0.0, -3.0 / 8.0 * t5 * t77 * t26 * t105);
        let tzk0 = t68 + t109;
        zk[ip] += tzk0;
        let t110 = t6 * t6;
        let t111 = 1.0 / t110;
        let t112 = t16 * t111;
        let t114 = piecewise5(t10, 0.0, t14, 0.0, t7 - t112);
        let t117 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t114);
        let t122 = t26 * t26;
        let t123 = 1.0 / t122;
        let t127 = t5 * t25 * t123 * t64 / 8.0;
        let t128 = t2 * t25;
        let t129 = t61 * t61;
        let t130 = 1.0 / t129;
        let t131 = t26 * t130;
        let t132 = t34 * rho0;
        let t134 = 1.0 / t36 / t132;
        let t135 = sigma0 * t134;
        let t141 = t52 * t34;
        let t143 = 1.0 / t35 / t141;
        let t144 = t51 * t143;
        let t148 = 1.0 / t59;
        let t152 = -10.0 / 729.0 * t33 * t135 - 0.10673131404562256653e-1 * t33 * t135 * t43 + 0.44471380852342736056e-3 * t50 * t144 * t43 - 0.73495086436683507408e-4 * t50 * t144 * t148;
        let t153 = t131 * t152;
        let t157 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t117 * t26 * t64 - t127 - 0.16551095363746320496e0 * t128 * t153);
        let t158 = t70 * t111;
        let t160 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t158);
        let t163 = piecewise3(t74, 0.0, 4.0 / 3.0 * t75 * t160);
        let t171 = t5 * t77 * t123 * t105 / 8.0;
        let t173 = piecewise3(t69, 0.0, -3.0 / 8.0 * t5 * t163 * t26 * t105 - t171);
        let tvrho0 = t68 + t109 + t6 * (t157 + t173);
        vrho[ip * 2] += tvrho0;
        let t177 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t112);
        let t180 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t177);
        let t186 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t180 * t26 * t64 - t127);
        let t188 = piecewise5(t14, 0.0, t10, 0.0, t7 - t158);
        let t191 = piecewise3(t74, 0.0, 4.0 / 3.0 * t75 * t188);
        let t196 = t2 * t77;
        let t197 = t102 * t102;
        let t198 = 1.0 / t197;
        let t199 = t26 * t198;
        let t200 = t79 * rho1;
        let t202 = 1.0 / t81 / t200;
        let t203 = sigma2 * t202;
        let t209 = t93 * t79;
        let t211 = 1.0 / t80 / t209;
        let t212 = t92 * t211;
        let t216 = 1.0 / t100;
        let t220 = -10.0 / 729.0 * t33 * t203 - 0.10673131404562256653e-1 * t33 * t203 * t88 + 0.44471380852342736056e-3 * t50 * t212 * t88 - 0.73495086436683507408e-4 * t50 * t212 * t216;
        let t221 = t199 * t220;
        let t225 = piecewise3(t69, 0.0, -3.0 / 8.0 * t5 * t191 * t26 * t105 - t171 - 0.16551095363746320496e0 * t196 * t221);
        let tvrho1 = t68 + t109 + t6 * (t186 + t225);
        vrho[ip * 2 + 1] += tvrho1;
        let t233 = sigma0 * t55;
        let t240 = 5.0 / 972.0 * t33 * t38 + 0.4002424276710846245e-2 * t33 * t38 * t43 - 0.16676767819628526021e-3 * t50 * t233 * t43 + 0.27560657413756315278e-4 * t50 * t233 * t148;
        let t241 = t131 * t240;
        let t244 = piecewise3(t1, 0.0, -0.16551095363746320496e0 * t128 * t241);
        let tvsigma0 = t6 * t244;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t250 = sigma2 * t96;
        let t257 = 5.0 / 972.0 * t33 * t83 + 0.4002424276710846245e-2 * t33 * t83 * t88 - 0.16676767819628526021e-3 * t50 * t250 * t88 + 0.27560657413756315278e-4 * t50 * t250 * t216;
        let t258 = t199 * t257;
        let t261 = piecewise3(t69, 0.0, -0.16551095363746320496e0 * t196 * t258);
        let tvsigma2 = t6 * t261;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
