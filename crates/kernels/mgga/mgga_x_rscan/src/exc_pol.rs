//! MGGA_X_RSCAN exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rscan.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_rscan_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_alphar: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
    param_taur: f64,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
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
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = t6 * t26;
        let t28 = pow_1_3(t7);
        let t29 = M_CBRT6;
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
        let t40 = sigma0 * t39;
        let t41 = t34 * t40;
        let t45 = 100.0 / 6561.0 / param_k1 - 73.0 / 648.0;
        let t46 = t29 * t29;
        let t47 = t45 * t46;
        let t48 = t31 * t30;
        let t49 = 1.0 / t48;
        let t50 = t47 * t49;
        let t51 = sigma0 * sigma0;
        let t52 = t35 * t35;
        let t53 = t52 * rho0;
        let t55 = 1.0 / t36 / t53;
        let t56 = t51 * t55;
        let t57 = t45 * t29;
        let t58 = t33 * sigma0;
        let t59 = t58 * t39;
        let t62 = f64::exp(-27.0 / 80.0 * t57 * t59);
        let t66 = f64::sqrt(146.0);
        let t67 = t66 * t29;
        let t70 = t20 * t20;
        let t71 = t70 * t70;
        let t72 = t71 * t20;
        let t73 = t7 * t7;
        let t74 = t73 * t73;
        let t75 = t74 * t7;
        let t76 = t72 * t75;
        let t77 = t37 * rho0;
        let t78 = 1.0 / t77;
        let t81 = tau0 * t78 - t40 / 8.0;
        let t82 = 0.0 < t81;
        let t83 = piecewise3(t82, t81, 0.0);
        let t84 = t83 * t83;
        let t85 = t84 * t83;
        let t86 = M_CBRT2;
        let t87 = t20 * t7;
        let t88 = pow_1_3(t87);
        let t89 = t88 * t88;
        let t92 = t46 * t32;
        let t95 = param_taur / 2.0;
        let t96 = 3.0 / 40.0 * t86 * t89 * t87 * t92 + t95;
        let t97 = t96 * t96;
        let t98 = t97 * t96;
        let t99 = 1.0 / t98;
        let t100 = t85 * t99;
        let t101 = t86 * t86;
        let t102 = t70 * t20;
        let t103 = t73 * t7;
        let t104 = t102 * t103;
        let t105 = t88 * t104;
        let t106 = t101 * t105;
        let t107 = 1.0 / t97;
        let t108 = t84 * t107;
        let t111 = t106 * t108 / 16.0 + param_alphar;
        let t112 = 1.0 / t111;
        let t113 = t100 * t112;
        let t115 = t76 * t113 / 32.0;
        let t116 = 1.0 - t115;
        let t118 = t116 * t116;
        let t120 = f64::exp(-t118 / 2.0);
        let t123 = 7.0 / 12960.0 * t67 * t59 + t66 * t116 * t120 / 100.0;
        let t124 = t123 * t123;
        let t125 = param_k1 + 5.0 / 972.0 * t41 + t50 * t56 * t62 / 576.0 + t124;
        let t130 = 1.0 + param_k1 * (1.0 - param_k1 / t125);
        let t131 = t115 <= 0.25e1;
        let t132 = 0.25e1 < t115;
        let t133 = piecewise3(t132, 0.25e1, t115);
        let t135 = t133 * t133;
        let t137 = t135 * t133;
        let t139 = t135 * t135;
        let t141 = t139 * t133;
        let t143 = t139 * t135;
        let t148 = piecewise3(t132, t115, 0.25e1);
        let t149 = 1.0 - t148;
        let t152 = f64::exp(param_c2 / t149);
        let t154 = piecewise3(t131, 1.0 - 0.667e0 * t133 - 0.4445555e0 * t135 - 0.663086601049e0 * t137 + 0.145129704449e1 * t139 - 0.887998041597e0 * t141 + 0.234528941479e0 * t143 - 0.23185843322e-1 * t139 * t137, -param_d * t152);
        let t155 = 1.0 - t154;
        let t158 = t130 * t155 + 0.1174e1 * t154;
        let t159 = t28 * t158;
        let t160 = f64::sqrt(3.0);
        let t161 = 1.0 / t31;
        let t162 = t46 * t161;
        let t163 = f64::sqrt(sigma0);
        let t164 = t36 * rho0;
        let t165 = 1.0 / t164;
        let t167 = t162 * t163 * t165;
        let t168 = f64::sqrt(t167);
        let t172 = f64::exp(-0.98958e1 * t160 / t168);
        let t173 = 1.0 - t172;
        let t174 = t159 * t173;
        let t177 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t174);
        let t178 = rho1 <= dens_threshold;
        let t179 = -t17;
        let t181 = piecewise5(t15, t12, t11, t16, t179 * t8);
        let t182 = 1.0 + t181;
        let t183 = t182 <= zeta_threshold;
        let t184 = pow_1_3(t182);
        let t186 = piecewise3(t183, t23, t184 * t182);
        let t187 = t6 * t186;
        let t188 = rho1 * rho1;
        let t189 = pow_1_3(rho1);
        let t190 = t189 * t189;
        let t191 = t190 * t188;
        let t192 = 1.0 / t191;
        let t193 = sigma2 * t192;
        let t194 = t34 * t193;
        let t196 = sigma2 * sigma2;
        let t197 = t188 * t188;
        let t198 = t197 * rho1;
        let t200 = 1.0 / t189 / t198;
        let t201 = t196 * t200;
        let t202 = t33 * sigma2;
        let t203 = t202 * t192;
        let t206 = f64::exp(-27.0 / 80.0 * t57 * t203);
        let t212 = t182 * t182;
        let t213 = t212 * t212;
        let t214 = t213 * t182;
        let t215 = t214 * t75;
        let t216 = t190 * rho1;
        let t217 = 1.0 / t216;
        let t220 = tau1 * t217 - t193 / 8.0;
        let t221 = 0.0 < t220;
        let t222 = piecewise3(t221, t220, 0.0);
        let t223 = t222 * t222;
        let t224 = t223 * t222;
        let t225 = t182 * t7;
        let t226 = pow_1_3(t225);
        let t227 = t226 * t226;
        let t232 = 3.0 / 40.0 * t86 * t227 * t225 * t92 + t95;
        let t233 = t232 * t232;
        let t234 = t233 * t232;
        let t235 = 1.0 / t234;
        let t236 = t224 * t235;
        let t237 = t212 * t182;
        let t238 = t237 * t103;
        let t239 = t226 * t238;
        let t240 = t101 * t239;
        let t241 = 1.0 / t233;
        let t242 = t223 * t241;
        let t245 = t240 * t242 / 16.0 + param_alphar;
        let t246 = 1.0 / t245;
        let t247 = t236 * t246;
        let t249 = t215 * t247 / 32.0;
        let t250 = 1.0 - t249;
        let t252 = t250 * t250;
        let t254 = f64::exp(-t252 / 2.0);
        let t257 = 7.0 / 12960.0 * t67 * t203 + t66 * t250 * t254 / 100.0;
        let t258 = t257 * t257;
        let t259 = param_k1 + 5.0 / 972.0 * t194 + t50 * t201 * t206 / 576.0 + t258;
        let t264 = 1.0 + param_k1 * (1.0 - param_k1 / t259);
        let t265 = t249 <= 0.25e1;
        let t266 = 0.25e1 < t249;
        let t267 = piecewise3(t266, 0.25e1, t249);
        let t269 = t267 * t267;
        let t271 = t269 * t267;
        let t273 = t269 * t269;
        let t275 = t273 * t267;
        let t277 = t273 * t269;
        let t282 = piecewise3(t266, t249, 0.25e1);
        let t283 = 1.0 - t282;
        let t286 = f64::exp(param_c2 / t283);
        let t288 = piecewise3(t265, 1.0 - 0.667e0 * t267 - 0.4445555e0 * t269 - 0.663086601049e0 * t271 + 0.145129704449e1 * t273 - 0.887998041597e0 * t275 + 0.234528941479e0 * t277 - 0.23185843322e-1 * t273 * t271, -param_d * t286);
        let t289 = 1.0 - t288;
        let t292 = t264 * t289 + 0.1174e1 * t288;
        let t293 = t28 * t292;
        let t294 = f64::sqrt(sigma2);
        let t295 = t189 * rho1;
        let t296 = 1.0 / t295;
        let t298 = t162 * t294 * t296;
        let t299 = f64::sqrt(t298);
        let t303 = f64::exp(-0.98958e1 * t160 / t299);
        let t304 = 1.0 - t303;
        let t305 = t293 * t304;
        let t308 = piecewise3(t178, 0.0, -3.0 / 8.0 * t187 * t305);
        let tzk0 = t177 + t308;
        zk[ip] += tzk0;
    }
}
