//! GGA_X_PBETRANS vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbetrans.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pbetrans_vxc_pol(
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
        let t27 = t25 * t26;
        let t28 = M_PI * M_PI;
        let t29 = pow_1_3(t28);
        let t30 = t2 * t29;
        let t31 = M_CBRT6;
        let t32 = t31 * t31;
        let t34 = t32 / t29;
        let t35 = f64::sqrt(sigma0);
        let t36 = pow_1_3(rho0);
        let t38 = 1.0 / t36 / rho0;
        let t45 = f64::exp(-2.0 * t30 * (t34 * t35 * t38 / 12.0 - 3.0));
        let t46 = 1.0 + t45;
        let t48 = 0.413e0 / t46;
        let t49 = 0.1227e1 - t48;
        let t50 = t29 * t29;
        let t52 = t31 / t50;
        let t53 = rho0 * rho0;
        let t54 = t36 * t36;
        let t56 = 1.0 / t54 / t53;
        let t60 = 0.1227e1 - t48 + 0.91249999999999999998e-2 * t52 * sigma0 * t56;
        let t61 = 1.0 / t60;
        let t63 = -t49 * t61 + 1.0;
        let t65 = t49 * t63 + 1.0;
        let t69 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t65);
        let t70 = rho1 <= dens_threshold;
        let t71 = -t16;
        let t73 = piecewise5(t14, t11, t10, t15, t71 * t7);
        let t74 = 1.0 + t73;
        let t75 = t74 <= zeta_threshold;
        let t76 = pow_1_3(t74);
        let t78 = piecewise3(t75, t22, t76 * t74);
        let t79 = t78 * t26;
        let t80 = f64::sqrt(sigma2);
        let t81 = pow_1_3(rho1);
        let t83 = 1.0 / t81 / rho1;
        let t90 = f64::exp(-2.0 * t30 * (t34 * t80 * t83 / 12.0 - 3.0));
        let t91 = 1.0 + t90;
        let t93 = 0.413e0 / t91;
        let t94 = 0.1227e1 - t93;
        let t95 = rho1 * rho1;
        let t96 = t81 * t81;
        let t98 = 1.0 / t96 / t95;
        let t102 = 0.1227e1 - t93 + 0.91249999999999999998e-2 * t52 * sigma2 * t98;
        let t103 = 1.0 / t102;
        let t105 = -t94 * t103 + 1.0;
        let t107 = t94 * t105 + 1.0;
        let t111 = piecewise3(t70, 0.0, -3.0 / 8.0 * t5 * t79 * t107);
        let tzk0 = t69 + t111;
        zk[ip] += tzk0;
        let t112 = t6 * t6;
        let t113 = 1.0 / t112;
        let t114 = t16 * t113;
        let t116 = piecewise5(t10, 0.0, t14, 0.0, t7 - t114);
        let t119 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t116);
        let t120 = t119 * t26;
        let t124 = t26 * t26;
        let t125 = 1.0 / t124;
        let t126 = t25 * t125;
        let t129 = t5 * t126 * t65 / 8.0;
        let t130 = t46 * t46;
        let t131 = 1.0 / t130;
        let t132 = t131 * t2;
        let t133 = t132 * t32;
        let t135 = 1.0 / t36 / t53;
        let t136 = t35 * t135;
        let t137 = t45 * t63;
        let t141 = t45 * t61;
        let t145 = t60 * t60;
        let t146 = 1.0 / t145;
        let t147 = t49 * t146;
        let t151 = t53 * rho0;
        let t153 = 1.0 / t54 / t151;
        let t157 = 0.91777777777777777778e-1 * t133 * t136 * t45 - 0.24333333333333333333e-1 * t52 * sigma0 * t153;
        let t159 = -0.91777777777777777778e-1 * t133 * t136 * t141 + t147 * t157;
        let t161 = 0.91777777777777777778e-1 * t133 * t136 * t137 + t49 * t159;
        let t166 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t120 * t65 - t129 - 3.0 / 8.0 * t5 * t27 * t161);
        let t167 = t71 * t113;
        let t169 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t167);
        let t172 = piecewise3(t75, 0.0, 4.0 / 3.0 * t76 * t169);
        let t173 = t172 * t26;
        let t177 = t78 * t125;
        let t180 = t5 * t177 * t107 / 8.0;
        let t182 = piecewise3(t70, 0.0, -3.0 / 8.0 * t5 * t173 * t107 - t180);
        let tvrho0 = t69 + t111 + t6 * (t166 + t182);
        vrho[ip * 2] += tvrho0;
        let t186 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t114);
        let t189 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t186);
        let t190 = t189 * t26;
        let t195 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t190 * t65 - t129);
        let t197 = piecewise5(t14, 0.0, t10, 0.0, t7 - t167);
        let t200 = piecewise3(t75, 0.0, 4.0 / 3.0 * t76 * t197);
        let t201 = t200 * t26;
        let t205 = t91 * t91;
        let t206 = 1.0 / t205;
        let t207 = t206 * t2;
        let t208 = t207 * t32;
        let t210 = 1.0 / t81 / t95;
        let t211 = t80 * t210;
        let t212 = t90 * t105;
        let t216 = t90 * t103;
        let t220 = t102 * t102;
        let t221 = 1.0 / t220;
        let t222 = t94 * t221;
        let t226 = t95 * rho1;
        let t228 = 1.0 / t96 / t226;
        let t232 = 0.91777777777777777778e-1 * t208 * t211 * t90 - 0.24333333333333333333e-1 * t52 * sigma2 * t228;
        let t234 = -0.91777777777777777778e-1 * t208 * t211 * t216 + t222 * t232;
        let t236 = 0.91777777777777777778e-1 * t208 * t211 * t212 + t94 * t234;
        let t241 = piecewise3(t70, 0.0, -3.0 / 8.0 * t5 * t201 * t107 - t180 - 3.0 / 8.0 * t5 * t79 * t236);
        let tvrho1 = t69 + t111 + t6 * (t195 + t241);
        vrho[ip * 2 + 1] += tvrho1;
        let t244 = 1.0 / t35;
        let t245 = t244 * t38;
        let t257 = -0.34416666666666666667e-1 * t133 * t245 * t45 + 0.91249999999999999998e-2 * t52 * t56;
        let t259 = 0.34416666666666666667e-1 * t133 * t245 * t141 + t147 * t257;
        let t261 = -0.34416666666666666667e-1 * t133 * t245 * t137 + t49 * t259;
        let t265 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t261);
        let tvsigma0 = t6 * t265;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t266 = 1.0 / t80;
        let t267 = t266 * t83;
        let t279 = -0.34416666666666666667e-1 * t208 * t267 * t90 + 0.91249999999999999998e-2 * t52 * t98;
        let t281 = 0.34416666666666666667e-1 * t208 * t267 * t216 + t222 * t279;
        let t283 = -0.34416666666666666667e-1 * t208 * t267 * t212 + t94 * t281;
        let t287 = piecewise3(t70, 0.0, -3.0 / 8.0 * t5 * t79 * t283);
        let tvsigma2 = t6 * t287;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
