//! GGA_K_MEYER vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_meyer.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_meyer_vxc_pol(
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
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t36 = 1.0 / t35;
        let t37 = t32 * t36;
        let t38 = rho0 * rho0;
        let t39 = pow_1_3(rho0);
        let t40 = t39 * t39;
        let t42 = 1.0 / t40 / t38;
        let t46 = 1.0 - t37 * sigma0 * t42 / 864.0;
        let t47 = t32 * t32;
        let t48 = 1.0 / t34;
        let t49 = t47 * t48;
        let t50 = f64::sqrt(sigma0);
        let t51 = t39 * rho0;
        let t52 = 1.0 / t51;
        let t55 = t49 * t50 * t52 / 72.0;
        let t56 = 1.0 + t55;
        let t57 = 1.0 - t55;
        let t58 = f64::abs(t57);
        let t59 = 1.0 / t58;
        let t61 = f64::ln(t56 * t59);
        let t63 = t46 * t61 * t32;
        let t64 = 1.0 / t50;
        let t65 = t34 * t64;
        let t68 = 3.0 * t63 * t65 * t51;
        let t69 = 1.0 / 2.0 - t68;
        let t70 = 1.0 / 2.0 + t68;
        let t71 = 1.0 / t70;
        let t74 = 20.0 * t69 * t71 + 1.0;
        let t78 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t74);
        let t79 = rho1 <= dens_threshold;
        let t80 = -t17;
        let t82 = piecewise5(t15, t12, t11, t16, t80 * t8);
        let t83 = 1.0 + t82;
        let t84 = t83 <= zeta_threshold;
        let t85 = pow_1_3(t83);
        let t86 = t85 * t85;
        let t88 = piecewise3(t84, t24, t86 * t83);
        let t89 = t88 * t30;
        let t90 = rho1 * rho1;
        let t91 = pow_1_3(rho1);
        let t92 = t91 * t91;
        let t94 = 1.0 / t92 / t90;
        let t98 = 1.0 - t37 * sigma2 * t94 / 864.0;
        let t99 = f64::sqrt(sigma2);
        let t100 = t91 * rho1;
        let t101 = 1.0 / t100;
        let t104 = t49 * t99 * t101 / 72.0;
        let t105 = 1.0 + t104;
        let t106 = 1.0 - t104;
        let t107 = f64::abs(t106);
        let t108 = 1.0 / t107;
        let t110 = f64::ln(t105 * t108);
        let t112 = t98 * t110 * t32;
        let t113 = 1.0 / t99;
        let t114 = t34 * t113;
        let t117 = 3.0 * t112 * t114 * t100;
        let t118 = 1.0 / 2.0 - t117;
        let t119 = 1.0 / 2.0 + t117;
        let t120 = 1.0 / t119;
        let t123 = 20.0 * t118 * t120 + 1.0;
        let t127 = piecewise3(t79, 0.0, 3.0 / 20.0 * t6 * t89 * t123);
        let tzk0 = t78 + t127;
        zk[ip] += tzk0;
        let t128 = t7 * t7;
        let t129 = 1.0 / t128;
        let t130 = t17 * t129;
        let t132 = piecewise5(t11, 0.0, t15, 0.0, t8 - t130);
        let t135 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t132);
        let t136 = t135 * t30;
        let t140 = 1.0 / t29;
        let t141 = t28 * t140;
        let t144 = t6 * t141 * t74 / 10.0;
        let t146 = 1.0 / t39 / t38;
        let t147 = t50 * t146;
        let t153 = t58 * t58;
        let t154 = 1.0 / t153;
        let t155 = t56 * t154;
        let t156 = t155 * t47;
        let t157 = t48 * t50;
        let t158 = f64::abs(t57) / t57;
        let t159 = t146 * t158;
        let t163 = -t49 * t147 * t59 / 54.0 - t156 * t157 * t159 / 54.0;
        let t164 = t46 * t163;
        let t165 = 1.0 / t56;
        let t166 = t165 * t58;
        let t167 = t164 * t166;
        let t168 = t32 * t34;
        let t170 = t168 * t64 * t51;
        let t176 = -t49 * t147 * t61 / 108.0 - 3.0 * t167 * t170 - 4.0 * t63 * t65 * t39;
        let t178 = t70 * t70;
        let t179 = 1.0 / t178;
        let t180 = t69 * t179;
        let t181 = -t176;
        let t184 = 20.0 * t176 * t71 - 20.0 * t180 * t181;
        let t189 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t136 * t74 + t144 + 3.0 / 20.0 * t6 * t31 * t184);
        let t190 = t80 * t129;
        let t192 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t190);
        let t195 = piecewise3(t84, 0.0, 5.0 / 3.0 * t86 * t192);
        let t196 = t195 * t30;
        let t200 = t88 * t140;
        let t203 = t6 * t200 * t123 / 10.0;
        let t205 = piecewise3(t79, 0.0, 3.0 / 20.0 * t6 * t196 * t123 + t203);
        let tvrho0 = t78 + t127 + t7 * (t189 + t205);
        vrho[ip * 2] += tvrho0;
        let t209 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t130);
        let t212 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t209);
        let t213 = t212 * t30;
        let t218 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t213 * t74 + t144);
        let t220 = piecewise5(t15, 0.0, t11, 0.0, t8 - t190);
        let t223 = piecewise3(t84, 0.0, 5.0 / 3.0 * t86 * t220);
        let t224 = t223 * t30;
        let t229 = 1.0 / t91 / t90;
        let t230 = t99 * t229;
        let t236 = t107 * t107;
        let t237 = 1.0 / t236;
        let t238 = t105 * t237;
        let t239 = t238 * t47;
        let t240 = t48 * t99;
        let t241 = f64::abs(t106) / t106;
        let t242 = t229 * t241;
        let t246 = -t49 * t230 * t108 / 54.0 - t239 * t240 * t242 / 54.0;
        let t247 = t98 * t246;
        let t248 = 1.0 / t105;
        let t249 = t248 * t107;
        let t250 = t247 * t249;
        let t252 = t168 * t113 * t100;
        let t258 = -t49 * t230 * t110 / 108.0 - 3.0 * t250 * t252 - 4.0 * t112 * t114 * t91;
        let t260 = t119 * t119;
        let t261 = 1.0 / t260;
        let t262 = t118 * t261;
        let t263 = -t258;
        let t266 = 20.0 * t258 * t120 - 20.0 * t262 * t263;
        let t271 = piecewise3(t79, 0.0, 3.0 / 20.0 * t6 * t224 * t123 + t203 + 3.0 / 20.0 * t6 * t89 * t266);
        let tvrho1 = t78 + t127 + t7 * (t218 + t271);
        vrho[ip * 2 + 1] += tvrho1;
        let t274 = t52 * t61;
        let t281 = t48 * t64;
        let t282 = t52 * t158;
        let t286 = t49 * t64 * t52 * t59 / 144.0 + t156 * t281 * t282 / 144.0;
        let t287 = t46 * t286;
        let t288 = t287 * t166;
        let t291 = t50 * sigma0;
        let t292 = 1.0 / t291;
        let t293 = t34 * t292;
        let t297 = t49 * t274 * t64 / 288.0 - 3.0 * t288 * t170 + 3.0 / 2.0 * t63 * t293 * t51;
        let t299 = -t297;
        let t302 = -20.0 * t180 * t299 + 20.0 * t297 * t71;
        let t306 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t302);
        let tvsigma0 = t7 * t306;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t307 = t101 * t110;
        let t314 = t48 * t113;
        let t315 = t101 * t241;
        let t319 = t49 * t113 * t101 * t108 / 144.0 + t239 * t314 * t315 / 144.0;
        let t320 = t98 * t319;
        let t321 = t320 * t249;
        let t324 = t99 * sigma2;
        let t325 = 1.0 / t324;
        let t326 = t34 * t325;
        let t330 = t49 * t307 * t113 / 288.0 - 3.0 * t321 * t252 + 3.0 / 2.0 * t112 * t326 * t100;
        let t332 = -t330;
        let t335 = 20.0 * t330 * t120 - 20.0 * t262 * t332;
        let t339 = piecewise3(t79, 0.0, 3.0 / 20.0 * t6 * t89 * t335);
        let tvsigma2 = t7 * t339;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
