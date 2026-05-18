//! MGGA_X_JK fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_jk.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_jk_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t6 = 1.0 / t5;
        let t7 = t4 * t6;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t16 = pow_1_3::<f64>(t12);
        let t18 = piecewise3::<f64>(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3::<f64>(rho[ip]);
        let t20 = t18 * t19;
        let t21 = t4 * t4;
        let t22 = param_beta * t21;
        let t24 = pow_1_3::<f64>(1.0 / M_PI);
        let t25 = 1.0 / t24;
        let t26 = M_CBRT4;
        let t27 = t25 * t26;
        let t28 = t22 * t27;
        let t29 = M_CBRT2;
        let t30 = t29 * t29;
        let t31 = sigma[ip] * t30;
        let t32 = rho[ip] * rho[ip];
        let t33 = t19 * t19;
        let t34 = t33 * t32;
        let t35 = 1.0 / t34;
        let t36 = param_gamma * param_beta;
        let t37 = f64::sqrt(sigma[ip]);
        let t38 = t36 * t37;
        let t40 = 1.0 / t19 / rho[ip];
        let t41 = t29 * t40;
        let t44 = f64::ln(t37 * t29 * t40 + f64::sqrt(pow_2::<f64>(t37 * t29 * t40) + 1.0));
        let t45 = t41 * t44;
        let t47 = t38 * t45 + 1.0;
        let t48 = 1.0 / t47;
        let t49 = t35 * t48;
        let t50 = t31 * t35;
        let t51 = lapl[ip] * t30;
        let t52 = t33 * rho[ip];
        let t53 = 1.0 / t52;
        let t55 = -t51 * t53 + t50;
        let t56 = 1.0 / sigma[ip];
        let t57 = t55 * t56;
        let t58 = t29 * t34;
        let t60 = t57 * t58 + 1.0;
        let t61 = 1.0 / t60;
        let t66 = 1.0 + 2.0 / 9.0 * t28 * t31 * t49 * t61;
        let t70 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t66);
        let tzk0 = 2.0 * t70;
        zk[ip] += tzk0;
        let t72 = t18 / t33;
        let t76 = t32 * rho[ip];
        let t78 = 1.0 / t33 / t76;
        let t79 = t78 * t48;
        let t85 = t22 * t27 * sigma[ip];
        let t86 = t30 * t35;
        let t87 = t47 * t47;
        let t88 = 1.0 / t87;
        let t89 = t88 * t61;
        let t91 = 1.0 / t19 / t32;
        let t93 = t29 * t91 * t44;
        let t95 = t36 * sigma[ip];
        let t96 = t30 * t78;
        let t97 = t50 + 1.0;
        let t98 = f64::sqrt(t97);
        let t99 = 1.0 / t98;
        let t100 = t96 * t99;
        let t103 = -4.0 / 3.0 * t95 * t100 - 4.0 / 3.0 * t38 * t93;
        let t104 = t89 * t103;
        let t105 = t86 * t104;
        let t108 = t60 * t60;
        let t109 = 1.0 / t108;
        let t110 = t48 * t109;
        let t115 = -8.0 / 3.0 * t31 * t78 + 5.0 / 3.0 * t51 * t35;
        let t116 = t115 * t56;
        let t118 = t29 * t52;
        let t121 = t116 * t58 + 8.0 / 3.0 * t57 * t118;
        let t122 = t110 * t121;
        let t123 = t86 * t122;
        let t126 = -16.0 / 27.0 * t28 * t31 * t79 * t61 - 2.0 / 9.0 * t85 * t105 - 2.0 / 9.0 * t85 * t123;
        let t131 = piecewise3::<f64>(t3, 0.0, -t7 * t72 * t66 / 8.0 - 3.0 / 8.0 * t7 * t20 * t126);
        let tvrho0 = 2.0 * rho[ip] * t131 + 2.0 * t70;
        vrho[ip] += tvrho0;
        let t134 = t48 * t61;
        let t138 = t36 / t37;
        let t140 = t86 * t99;
        let t143 = t138 * t45 / 2.0 + t36 * t140 / 2.0;
        let t144 = t89 * t143;
        let t145 = t86 * t144;
        let t148 = sigma[ip] * sigma[ip];
        let t149 = 1.0 / t148;
        let t150 = t55 * t149;
        let t152 = -t150 * t58 + 2.0 * t56;
        let t153 = t110 * t152;
        let t154 = t86 * t153;
        let t157 = 2.0 / 9.0 * t28 * t86 * t134 - 2.0 / 9.0 * t85 * t145 - 2.0 / 9.0 * t85 * t154;
        let t161 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t157);
        let tvsigma0 = 2.0 * rho[ip] * t161;
        vsigma[ip] += tvsigma0;
        let t163 = t6 * t18;
        let t164 = t40 * param_beta;
        let t166 = t30 * t48;
        let t168 = t27 * t166 * t109;
        let t171 = piecewise3::<f64>(t3, 0.0, -t163 * t164 * t168 / 2.0);
        let tvlapl0 = 2.0 * rho[ip] * t171;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t174 = t18 * t53;
        let t181 = t32 * t32;
        let t183 = 1.0 / t33 / t181;
        let t189 = t96 * t104;
        let t192 = t96 * t122;
        let t196 = 1.0 / t87 / t47;
        let t197 = t196 * t61;
        let t198 = t103 * t103;
        let t199 = t197 * t198;
        let t200 = t86 * t199;
        let t203 = t86 * t88;
        let t204 = t109 * t103;
        let t205 = t204 * t121;
        let t210 = 1.0 / t19 / t76;
        let t212 = t29 * t210 * t44;
        let t215 = t30 * t183;
        let t216 = t215 * t99;
        let t219 = t36 * t148;
        let t222 = 1.0 / t19 / t181 / t76;
        let t225 = 1.0 / t98 / t97;
        let t226 = t29 * t222 * t225;
        let t229 = 28.0 / 9.0 * t38 * t212 + 20.0 / 3.0 * t95 * t216 - 32.0 / 9.0 * t219 * t226;
        let t230 = t89 * t229;
        let t231 = t86 * t230;
        let t235 = 1.0 / t108 / t60;
        let t236 = t48 * t235;
        let t237 = t121 * t121;
        let t238 = t236 * t237;
        let t239 = t86 * t238;
        let t246 = 88.0 / 9.0 * t31 * t183 - 40.0 / 9.0 * t51 * t78;
        let t247 = t246 * t56;
        let t251 = t29 * t33;
        let t254 = t247 * t58 + 16.0 / 3.0 * t116 * t118 + 40.0 / 9.0 * t57 * t251;
        let t255 = t110 * t254;
        let t256 = t86 * t255;
        let t259 = 176.0 / 81.0 * t28 * t31 * t183 * t48 * t61 + 32.0 / 27.0 * t85 * t189 + 32.0 / 27.0 * t85 * t192 + 4.0 / 9.0 * t85 * t200 + 4.0 / 9.0 * t85 * t203 * t205 - 2.0 / 9.0 * t85 * t231 + 4.0 / 9.0 * t85 * t239 - 2.0 / 9.0 * t85 * t256;
        let t264 = piecewise3::<f64>(t3, 0.0, t7 * t174 * t66 / 12.0 - t7 * t72 * t126 / 4.0 - 3.0 / 8.0 * t7 * t20 * t259);
        let tv2rho20 = 2.0 * rho[ip] * t264 + 4.0 * t131;
        v2rho2[ip] += tv2rho20;
        let t277 = t96 * t144;
        let t280 = t86 * t196;
        let t281 = t61 * t143;
        let t282 = t281 * t103;
        let t286 = t109 * t143;
        let t287 = t286 * t121;
        let t295 = t36 * t29;
        let t296 = t181 * t32;
        let t298 = 1.0 / t19 / t296;
        let t303 = -2.0 / 3.0 * t138 * t93 - 2.0 * t36 * t100 + 4.0 / 3.0 * t295 * t298 * t225 * sigma[ip];
        let t304 = t89 * t303;
        let t305 = t86 * t304;
        let t308 = t96 * t153;
        let t311 = t109 * t152;
        let t312 = t311 * t103;
        let t316 = t86 * t48;
        let t317 = t235 * t152;
        let t318 = t317 * t121;
        let t322 = t115 * t149;
        let t326 = -t322 * t58 - 8.0 / 3.0 * t150 * t118;
        let t327 = t110 * t326;
        let t328 = t86 * t327;
        let t331 = -16.0 / 27.0 * t28 * t96 * t134 - 2.0 / 9.0 * t28 * t105 - 2.0 / 9.0 * t28 * t123 + 16.0 / 27.0 * t85 * t277 + 4.0 / 9.0 * t85 * t280 * t282 + 2.0 / 9.0 * t85 * t203 * t287 - 2.0 / 9.0 * t85 * t305 + 16.0 / 27.0 * t85 * t308 + 2.0 / 9.0 * t85 * t203 * t312 + 4.0 / 9.0 * t85 * t316 * t318 - 2.0 / 9.0 * t85 * t328;
        let t336 = piecewise3::<f64>(t3, 0.0, -t7 * t72 * t157 / 8.0 - 3.0 / 8.0 * t7 * t20 * t331);
        let tv2rhosigma0 = 2.0 * rho[ip] * t336 + 2.0 * t161;
        v2rhosigma[ip] += tv2rhosigma0;
        let t339 = t91 * param_beta;
        let t344 = t163 * t164 * t25;
        let t345 = t26 * t30;
        let t346 = t88 * t109;
        let t348 = t345 * t346 * t103;
        let t352 = t345 * t236 * t121;
        let t355 = piecewise3::<f64>(t3, 0.0, 2.0 / 3.0 * t163 * t339 * t168 + t344 * t348 / 2.0 + t344 * t352);
        let tv2rholapl0 = 2.0 * rho[ip] * t355 + 2.0 * t171;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip] += tv2rhotau0;
        let t362 = t143 * t143;
        let t363 = t197 * t362;
        let t364 = t86 * t363;
        let t367 = t286 * t152;
        let t373 = t36 / t37 / sigma[ip];
        let t376 = t36 * t56;
        let t379 = t181 * rho[ip];
        let t382 = t29 / t19 / t379;
        let t383 = t382 * t225;
        let t386 = -t373 * t45 / 4.0 + t376 * t140 / 4.0 - t36 * t383 / 2.0;
        let t387 = t89 * t386;
        let t388 = t86 * t387;
        let t391 = t152 * t152;
        let t392 = t236 * t391;
        let t393 = t86 * t392;
        let t397 = t148 * sigma[ip];
        let t398 = 1.0 / t397;
        let t399 = t55 * t398;
        let t402 = 2.0 * t399 * t58 - 4.0 * t149;
        let t403 = t110 * t402;
        let t404 = t86 * t403;
        let t407 = -4.0 / 9.0 * t28 * t145 - 4.0 / 9.0 * t28 * t154 + 4.0 / 9.0 * t85 * t364 + 4.0 / 9.0 * t85 * t203 * t367 - 2.0 / 9.0 * t85 * t388 + 4.0 / 9.0 * t85 * t393 - 2.0 / 9.0 * t85 * t404;
        let t411 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t407);
        let tv2sigma20 = 2.0 * rho[ip] * t411;
        v2sigma2[ip] += tv2sigma20;
        let t414 = t345 * t346 * t143;
        let t418 = t345 * t236 * t152;
        let t421 = piecewise3::<f64>(t3, 0.0, t344 * t414 / 2.0 + t344 * t418);
        let tv2sigmalapl0 = 2.0 * rho[ip] * t421;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t423 = 1.0 / t19;
        let t426 = t163 * t423 * param_beta * t25;
        let t428 = t345 * t236 * t56;
        let t431 = piecewise3::<f64>(t3, 0.0, -2.0 * t426 * t428);
        let tv2lapl20 = 2.0 * rho[ip] * t431;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
    }
}
