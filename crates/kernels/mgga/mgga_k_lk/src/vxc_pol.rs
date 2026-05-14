//! MGGA_K_LK vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_lk.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_k_lk_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_kappa: f64,
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
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = rho0 + rho1;
        let t9 = 1.0 / t8;
        let t12 = 2.0 * rho0 * t9 <= zeta_threshold;
        let t13 = zeta_threshold - 1.0;
        let t16 = 2.0 * rho1 * t9 <= zeta_threshold;
        let t17 = -t13;
        let t18 = rho0 - rho1;
        let t20 = piecewise5(t12, t13, t16, t17, t18 * t9);
        let t21 = 1.0 + t20;
        let t22 = t21 <= zeta_threshold;
        let t23 = pow_1_3(zeta_threshold);
        let t24 = t23 * t23;
        let t25 = t24 * zeta_threshold;
        let t26 = pow_1_3(t21);
        let t27 = t26 * t26;
        let t29 = piecewise3(t22, t25, t27 * t21);
        let t30 = pow_1_3(t8);
        let t31 = t30 * t30;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t44 = sigma0 * t43;
        let t47 = t33 * t33;
        let t49 = 1.0 / t35 / t34;
        let t50 = t47 * t49;
        let t51 = lapl0 * lapl0;
        let t52 = t39 * rho0;
        let t54 = 1.0 / t40 / t52;
        let t57 = t50 * t51 * t54 / 5832.0;
        let t58 = t39 * t39;
        let t60 = 1.0 / t40 / t58;
        let t61 = sigma0 * t60;
        let t64 = t50 * t61 * lapl0 / 5184.0;
        let t65 = sigma0 * sigma0;
        let t66 = t58 * rho0;
        let t68 = 1.0 / t40 / t66;
        let t69 = t65 * t68;
        let t71 = t50 * t69 / 17496.0;
        let t72 = 1.0 / param_kappa;
        let t78 = 1.0 + (5.0 / 648.0 * t38 * t44 + t57 - t64 + t71 + 25.0 / 419904.0 * t50 * t69 * t72) * t72;
        let t80 = t38 * sigma0;
        let t81 = t57 - t64 + t71;
        let t83 = t43 * t81 * t72;
        let t86 = t34 * t34;
        let t87 = 1.0 / t86;
        let t88 = t65 * sigma0;
        let t89 = t87 * t88;
        let t90 = t58 * t58;
        let t91 = 1.0 / t90;
        let t92 = param_kappa * param_kappa;
        let t93 = 1.0 / t92;
        let t94 = t91 * t93;
        let t99 = 1.0 + (5.0 / 324.0 * t80 * t83 + 125.0 / 0.45349632e8 * t89 * t94) * t72;
        let t103 = 1.0 + param_kappa * (2.0 - 1.0 / t78 - 1.0 / t99);
        let t107 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t29 * t31 * t103);
        let t108 = rho1 <= dens_threshold;
        let t109 = -t18;
        let t111 = piecewise5(t16, t13, t12, t17, t109 * t9);
        let t112 = 1.0 + t111;
        let t113 = t112 <= zeta_threshold;
        let t114 = pow_1_3(t112);
        let t115 = t114 * t114;
        let t117 = piecewise3(t113, t25, t115 * t112);
        let t119 = rho1 * rho1;
        let t120 = pow_1_3(rho1);
        let t121 = t120 * t120;
        let t123 = 1.0 / t121 / t119;
        let t124 = sigma2 * t123;
        let t127 = lapl1 * lapl1;
        let t128 = t119 * rho1;
        let t130 = 1.0 / t120 / t128;
        let t133 = t50 * t127 * t130 / 5832.0;
        let t134 = t119 * t119;
        let t136 = 1.0 / t120 / t134;
        let t137 = sigma2 * t136;
        let t140 = t50 * t137 * lapl1 / 5184.0;
        let t141 = sigma2 * sigma2;
        let t142 = t134 * rho1;
        let t144 = 1.0 / t120 / t142;
        let t145 = t141 * t144;
        let t147 = t50 * t145 / 17496.0;
        let t153 = 1.0 + (5.0 / 648.0 * t38 * t124 + t133 - t140 + t147 + 25.0 / 419904.0 * t50 * t145 * t72) * t72;
        let t155 = t38 * sigma2;
        let t156 = t133 - t140 + t147;
        let t158 = t123 * t156 * t72;
        let t161 = t141 * sigma2;
        let t162 = t87 * t161;
        let t163 = t134 * t134;
        let t164 = 1.0 / t163;
        let t165 = t164 * t93;
        let t170 = 1.0 + (5.0 / 324.0 * t155 * t158 + 125.0 / 0.45349632e8 * t162 * t165) * t72;
        let t174 = 1.0 + param_kappa * (2.0 - 1.0 / t153 - 1.0 / t170);
        let t178 = piecewise3(t108, 0.0, 3.0 / 20.0 * t7 * t117 * t31 * t174);
        let tzk0 = t107 + t178;
        zk[ip] += tzk0;
        let t179 = t8 * t8;
        let t180 = 1.0 / t179;
        let t181 = t18 * t180;
        let t183 = piecewise5(t12, 0.0, t16, 0.0, t9 - t181);
        let t186 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t183);
        let t191 = 1.0 / t30;
        let t195 = t7 * t29 * t191 * t103 / 10.0;
        let t196 = t7 * t29;
        let t197 = t31 * param_kappa;
        let t198 = t78 * t78;
        let t199 = 1.0 / t198;
        let t201 = 1.0 / t41 / t52;
        let t202 = sigma0 * t201;
        let t207 = 5.0 / 8748.0 * t50 * t51 * t60;
        let t208 = sigma0 * t68;
        let t211 = 13.0 / 15552.0 * t50 * t208 * lapl0;
        let t212 = t58 * t39;
        let t214 = 1.0 / t40 / t212;
        let t215 = t65 * t214;
        let t217 = 2.0 / 6561.0 * t50 * t215;
        let t221 = -5.0 / 243.0 * t38 * t202 - t207 + t211 - t217 - 25.0 / 78732.0 * t50 * t215 * t72;
        let t224 = t99 * t99;
        let t225 = 1.0 / t224;
        let t227 = t201 * t81 * t72;
        let t230 = -t207 + t211 - t217;
        let t232 = t43 * t230 * t72;
        let t235 = t90 * rho0;
        let t236 = 1.0 / t235;
        let t237 = t236 * t93;
        let t240 = -10.0 / 243.0 * t80 * t227 + 5.0 / 324.0 * t80 * t232 - 125.0 / 5668704.0 * t89 * t237;
        let t243 = t199 * t221 * t72 + t225 * t240 * t72;
        let t244 = t197 * t243;
        let t248 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t186 * t31 * t103 + t195 + 3.0 / 20.0 * t196 * t244);
        let t249 = t109 * t180;
        let t251 = piecewise5(t16, 0.0, t12, 0.0, -t9 - t249);
        let t254 = piecewise3(t113, 0.0, 5.0 / 3.0 * t115 * t251);
        let t262 = t7 * t117 * t191 * t174 / 10.0;
        let t264 = piecewise3(t108, 0.0, 3.0 / 20.0 * t7 * t254 * t31 * t174 + t262);
        let tvrho0 = t107 + t178 + t8 * (t248 + t264);
        vrho[ip * 2] += tvrho0;
        let t268 = piecewise5(t12, 0.0, t16, 0.0, -t9 - t181);
        let t271 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t268);
        let t277 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t271 * t31 * t103 + t195);
        let t279 = piecewise5(t16, 0.0, t12, 0.0, t9 - t249);
        let t282 = piecewise3(t113, 0.0, 5.0 / 3.0 * t115 * t279);
        let t287 = t7 * t117;
        let t288 = t153 * t153;
        let t289 = 1.0 / t288;
        let t291 = 1.0 / t121 / t128;
        let t292 = sigma2 * t291;
        let t297 = 5.0 / 8748.0 * t50 * t127 * t136;
        let t298 = sigma2 * t144;
        let t301 = 13.0 / 15552.0 * t50 * t298 * lapl1;
        let t302 = t134 * t119;
        let t304 = 1.0 / t120 / t302;
        let t305 = t141 * t304;
        let t307 = 2.0 / 6561.0 * t50 * t305;
        let t311 = -5.0 / 243.0 * t38 * t292 - t297 + t301 - t307 - 25.0 / 78732.0 * t50 * t305 * t72;
        let t314 = t170 * t170;
        let t315 = 1.0 / t314;
        let t317 = t291 * t156 * t72;
        let t320 = -t297 + t301 - t307;
        let t322 = t123 * t320 * t72;
        let t325 = t163 * rho1;
        let t326 = 1.0 / t325;
        let t327 = t326 * t93;
        let t330 = -10.0 / 243.0 * t155 * t317 + 5.0 / 324.0 * t155 * t322 - 125.0 / 5668704.0 * t162 * t327;
        let t333 = t289 * t311 * t72 + t315 * t330 * t72;
        let t334 = t197 * t333;
        let t338 = piecewise3(t108, 0.0, 3.0 / 20.0 * t7 * t282 * t31 * t174 + t262 + 3.0 / 20.0 * t287 * t334);
        let tvrho1 = t107 + t178 + t8 * (t277 + t338);
        vrho[ip * 2 + 1] += tvrho1;
        let t344 = t50 * t60 * lapl0;
        let t345 = t344 / 5184.0;
        let t346 = t50 * t208;
        let t347 = t346 / 8748.0;
        let t351 = 5.0 / 648.0 * t38 * t43 - t345 + t347 + 25.0 / 209952.0 * t50 * t208 * t72;
        let t356 = -t345 + t347;
        let t358 = t43 * t356 * t72;
        let t361 = t87 * t65;
        let t364 = 5.0 / 324.0 * t38 * t83 + 5.0 / 324.0 * t80 * t358 + 125.0 / 0.15116544e8 * t361 * t94;
        let t367 = t199 * t351 * t72 + t225 * t364 * t72;
        let t368 = t197 * t367;
        let t371 = piecewise3(t2, 0.0, 3.0 / 20.0 * t196 * t368);
        let tvsigma0 = t8 * t371;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t375 = t50 * t136 * lapl1;
        let t376 = t375 / 5184.0;
        let t377 = t50 * t298;
        let t378 = t377 / 8748.0;
        let t382 = 5.0 / 648.0 * t38 * t123 - t376 + t378 + 25.0 / 209952.0 * t50 * t298 * t72;
        let t387 = -t376 + t378;
        let t389 = t123 * t387 * t72;
        let t392 = t87 * t141;
        let t395 = 5.0 / 324.0 * t38 * t158 + 5.0 / 324.0 * t155 * t389 + 125.0 / 0.15116544e8 * t392 * t165;
        let t398 = t289 * t382 * t72 + t315 * t395 * t72;
        let t399 = t197 * t398;
        let t402 = piecewise3(t108, 0.0, 3.0 / 20.0 * t287 * t399);
        let tvsigma2 = t8 * t402;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t408 = t50 * lapl0 * t54 / 2916.0 - t50 * t61 / 5184.0;
        let t412 = t225 * t33 * t37;
        let t413 = t408 * t93;
        let t417 = t199 * t408 * t72 + 5.0 / 324.0 * t412 * t44 * t413;
        let t418 = t197 * t417;
        let t421 = piecewise3(t2, 0.0, 3.0 / 20.0 * t196 * t418);
        let tvlapl0 = t8 * t421;
        vlapl[ip * 2] += tvlapl0;
        let t427 = t50 * lapl1 * t130 / 2916.0 - t50 * t137 / 5184.0;
        let t431 = t315 * t33 * t37;
        let t432 = t427 * t93;
        let t436 = t289 * t427 * t72 + 5.0 / 324.0 * t431 * t124 * t432;
        let t437 = t197 * t436;
        let t440 = piecewise3(t108, 0.0, 3.0 / 20.0 * t287 * t437);
        let tvlapl1 = t8 * t440;
        vlapl[ip * 2 + 1] += tvlapl1;
        let tvtau0 = 0.0;
        vtau[ip * 2] += tvtau0;
        let tvtau1 = 0.0;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
