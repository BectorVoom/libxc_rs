//! GGA_C_AM05 kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_am05.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use crate::math::piecewise::{piecewise3};
use crate::math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_am05_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    param_alpha: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t10 = t4 * t6 / t7;
        let t12 = 1.0 + 0.53425e-1 * t10;
        let t13 = f64::sqrt(t10);
        let t16 = pow_3_2(t10);
        let t18 = t1 * t1;
        let t19 = t3 * t3;
        let t20 = t18 * t19;
        let t21 = t7 * t7;
        let t24 = t20 * t5 / t21;
        let t26 = 0.379785e1 * t13 + 0.8969e0 * t10 + 0.204775e0 * t16 + 0.123235e0 * t24;
        let t29 = 1.0 + 0.16081979498692535067e2 / t26;
        let t30 = f64::ln(t29);
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.278125e-1 * t10;
        let t50 = 0.51785e1 * t13 + 0.905775e0 * t10 + 0.1100325e0 * t16 + 0.1241775e0 * t24;
        let t53 = 1.0 + 0.29608749977793437516e2 / t50;
        let t54 = f64::ln(t53);
        let t58 = -0.621814e-1 * t12 * t30 + 0.19751673498613801407e-1 * t43 * t45 * t54;
        let t59 = piecewise3(t33, zeta_threshold, 1.0);
        let t60 = t58 * t59;
        let t61 = M_CBRT6;
        let t62 = param_alpha * t61;
        let t63 = M_PI * M_PI;
        let t64 = pow_1_3(t63);
        let t65 = t64 * t64;
        let t66 = 1.0 / t65;
        let t68 = t39 * t39;
        let t69 = sigma[ip] * t68;
        let t70 = rho[ip] * rho[ip];
        let t72 = 1.0 / t21 / t70;
        let t76 = 1.0 + t62 * t66 * t69 * t72 / 24.0;
        let t77 = 1.0 / t76;
        let t80 = t77 + param_gamma * (1.0 - t77);
        let tzk0 = t60 * t80;
        zk[ip] += tzk0;
        let t82 = 1.0 / t7 / rho[ip];
        let t83 = t6 * t82;
        let t87 = t26 * t26;
        let t88 = 1.0 / t87;
        let t89 = t12 * t88;
        let t91 = 1.0 / t13 * t1;
        let t92 = t3 * t6;
        let t93 = t92 * t82;
        let t94 = t91 * t93;
        let t96 = t4 * t83;
        let t98 = f64::sqrt(t10);
        let t99 = t98 * t1;
        let t100 = t99 * t93;
        let t105 = t20 * t5 / t21 / rho[ip];
        let t107 = -0.632975e0 * t94 - 0.29896666666666666667e0 * t96 - 0.1023875e0 * t100 - 0.82156666666666666667e-1 * t105;
        let t108 = 1.0 / t29;
        let t109 = t107 * t108;
        let t112 = t43 * t1;
        let t117 = t43 * t45;
        let t118 = t50 * t50;
        let t119 = 1.0 / t118;
        let t124 = -0.86308333333333333334e0 * t94 - 0.301925e0 * t96 - 0.5501625e-1 * t100 - 0.82785e-1 * t105;
        let t126 = 1.0 / t53;
        let t127 = t119 * t124 * t126;
        let t130 = 0.11073470983333333333e-2 * t4 * t83 * t30 + 1.0 * t89 * t109 - 0.18311447306006545054e-3 * t112 * t92 * t82 * t54 - 0.5848223622634646207e0 * t117 * t127;
        let t131 = rho[ip] * t130;
        let t132 = t59 * t80;
        let t134 = rho[ip] * t58;
        let t135 = t76 * t76;
        let t136 = 1.0 / t135;
        let t138 = t136 * param_alpha * t61;
        let t139 = t66 * sigma[ip];
        let t140 = t70 * rho[ip];
        let t142 = 1.0 / t21 / t140;
        let t143 = t68 * t142;
        let t144 = t139 * t143;
        let t146 = param_gamma * t136;
        let t147 = t146 * t62;
        let t150 = t138 * t144 / 9.0 - t147 * t144 / 9.0;
        let t151 = t59 * t150;
        let tvrho0 = t131 * t132 + t134 * t151 + tzk0;
        vrho[ip] += tvrho0;
        let t153 = t66 * t68;
        let t156 = t146 * param_alpha;
        let t157 = t61 * t66;
        let t162 = t156 * t157 * t68 * t72 / 24.0 - t138 * t153 * t72 / 24.0;
        let t163 = t59 * t162;
        let tvsigma0 = t134 * t163;
        vsigma[ip] += tvsigma0;
        let t164 = t130 * t59;
        let t170 = 1.0 / t7 / t70;
        let t171 = t6 * t170;
        let t175 = t4 * t6;
        let t176 = t82 * t88;
        let t180 = t87 * t26;
        let t181 = 1.0 / t180;
        let t182 = t12 * t181;
        let t183 = t107 * t107;
        let t184 = t183 * t108;
        let t189 = 1.0 / t13 / t10 * t18;
        let t190 = t19 * t5;
        let t191 = t190 * t72;
        let t192 = t189 * t191;
        let t194 = t92 * t170;
        let t195 = t91 * t194;
        let t197 = t4 * t171;
        let t199 = 1.0/f64::sqrt(t10);
        let t200 = t199 * t18;
        let t201 = t200 * t191;
        let t203 = t99 * t194;
        let t206 = t20 * t5 * t72;
        let t208 = -0.42198333333333333333e0 * t192 + 0.84396666666666666666e0 * t195 + 0.39862222222222222223e0 * t197 + 0.68258333333333333333e-1 * t201 + 0.13651666666666666667e0 * t203 + 0.13692777777777777778e0 * t206;
        let t209 = t208 * t108;
        let t212 = t87 * t87;
        let t213 = 1.0 / t212;
        let t214 = t12 * t213;
        let t215 = t29 * t29;
        let t216 = 1.0 / t215;
        let t217 = t183 * t216;
        let t224 = t43 * t4;
        let t228 = t118 * t50;
        let t229 = 1.0 / t228;
        let t230 = t124 * t124;
        let t232 = t229 * t230 * t126;
        let t241 = -0.57538888888888888889e0 * t192 + 0.11507777777777777778e1 * t195 + 0.40256666666666666667e0 * t197 + 0.366775e-1 * t201 + 0.73355e-1 * t203 + 0.137975e0 * t206;
        let t243 = t119 * t241 * t126;
        let t246 = t118 * t118;
        let t247 = 1.0 / t246;
        let t248 = t247 * t230;
        let t249 = t53 * t53;
        let t250 = 1.0 / t249;
        let t251 = t248 * t250;
        let t254 = -0.14764627977777777777e-2 * t4 * t171 * t30 - 0.35616666666666666666e-1 * t175 * t176 * t109 - 2.0 * t182 * t184 + 1.0 * t89 * t209 + 0.16081979498692535067e2 * t214 * t217 + 0.24415263074675393405e-3 * t112 * t92 * t170 * t54 + 0.10843581300301739842e-1 * t224 * t83 * t127 + 0.11696447245269292414e1 * t117 * t232 - 0.5848223622634646207e0 * t117 * t243 - 0.17315859105681463759e2 * t117 * t251;
        let t255 = rho[ip] * t254;
        let t260 = 1.0 / t135 / t76;
        let t261 = param_alpha * param_alpha;
        let t263 = t61 * t61;
        let t264 = t260 * t261 * t263;
        let t266 = 1.0 / t64 / t63;
        let t267 = sigma[ip] * sigma[ip];
        let t268 = t266 * t267;
        let t269 = t70 * t70;
        let t272 = 1.0 / t7 / t269 / t140;
        let t273 = t39 * t272;
        let t274 = t268 * t273;
        let t278 = 1.0 / t21 / t269;
        let t279 = t68 * t278;
        let t280 = t139 * t279;
        let t283 = param_gamma * t260;
        let t285 = t283 * t261 * t263;
        let t290 = 4.0 / 81.0 * t264 * t274 - 11.0 / 27.0 * t138 * t280 - 4.0 / 81.0 * t285 * t274 + 11.0 / 27.0 * t147 * t280;
        let t291 = t59 * t290;
        let tv2rho20 = 2.0 * t131 * t151 + t255 * t132 + t134 * t291 + 2.0 * t60 * t150 + 2.0 * t164 * t80;
        v2rho2[ip] += tv2rho20;
        let t295 = t266 * t39;
        let t296 = t269 * t70;
        let t298 = 1.0 / t7 / t296;
        let t300 = t295 * t298 * sigma[ip];
        let t311 = -t264 * t300 / 54.0 + t138 * t153 * t142 / 9.0 + t285 * t300 / 54.0 - t156 * t157 * t143 / 9.0;
        let t312 = t59 * t311;
        let tv2rhosigma0 = t131 * t163 + t134 * t312 + t60 * t162;
        v2rhosigma[ip] += tv2rhosigma0;
        let t314 = t269 * rho[ip];
        let t316 = 1.0 / t7 / t314;
        let t319 = t283 * t261;
        let t320 = t263 * t266;
        let t325 = -t319 * t320 * t39 * t316 / 144.0 + t264 * t295 * t316 / 144.0;
        let t326 = t59 * t325;
        let tv2sigma20 = t134 * t326;
        v2sigma2[ip] += tv2sigma20;
        let t327 = t254 * t59;
        let t347 = 1.0 / t212 / t26;
        let t348 = t12 * t347;
        let t349 = t183 * t107;
        let t350 = t349 * t216;
        let t356 = 1.0 / t13 / t24 * t2 / 4.0;
        let t357 = 1.0 / t269;
        let t358 = t356 * t357;
        let t360 = t190 * t142;
        let t361 = t189 * t360;
        let t364 = 1.0 / t7 / t140;
        let t365 = t92 * t364;
        let t366 = t91 * t365;
        let t368 = t6 * t364;
        let t369 = t4 * t368;
        let t371 = 1.0/pow_3_2(t10);
        let t372 = t371 * t2;
        let t373 = t372 * t357;
        let t375 = t200 * t360;
        let t377 = t99 * t365;
        let t380 = t20 * t5 * t142;
        let t382 = -0.25319e1 * t358 + 0.16879333333333333333e1 * t361 - 0.19692555555555555555e1 * t366 - 0.93011851851851851854e0 * t369 + 0.13651666666666666667e0 * t373 - 0.27303333333333333333e0 * t375 - 0.3185388888888888889e0 * t377 - 0.36514074074074074075e0 * t380;
        let t383 = t382 * t108;
        let t387 = 1.0 / t212 / t87;
        let t388 = t12 * t387;
        let t390 = 1.0 / t215 / t29;
        let t391 = t349 * t390;
        let t402 = t349 * t108;
        let t408 = -0.32530743900905219526e-1 * t224 * t83 * t232 - 0.21687162600603479684e-1 * t224 * t171 * t127 + 0.16265371950452609763e-1 * t224 * t83 * t243 + 0.48159733137676571078e0 * t224 * t83 * t251 - 0.96491876992155210402e2 * t348 * t350 + 1.0 * t89 * t383 + 0.51726012919273400301e3 * t388 * t391 + 0.10685e0 * t175 * t82 * t181 * t184 - 0.56968947174242584612e-3 * t112 * t92 * t364 * t54 + 6.0 * t214 * t402 - 6.0 * t182 * t109 * t208;
        let t414 = 1.0 / t246 / t50;
        let t415 = t230 * t124;
        let t417 = t414 * t415 * t250;
        let t428 = -0.34523333333333333333e1 * t358 + 0.23015555555555555556e1 * t361 - 0.26851481481481481482e1 * t366 - 0.93932222222222222223e0 * t369 + 0.73355e-1 * t373 - 0.14671e0 * t375 - 0.17116166666666666667e0 * t377 - 0.36793333333333333333e0 * t380;
        let t430 = t119 * t428 * t126;
        let t434 = 1.0 / t246 / t118;
        let t437 = 1.0 / t249 / t53;
        let t438 = t434 * t415 * t437;
        let t444 = t170 * t88;
        let t451 = t82 * t213;
        let t456 = t247 * t415 * t126;
        let t460 = t126 * t241;
        let t465 = t250 * t124;
        let t469 = 0.48245938496077605201e2 * t214 * t208 * t216 * t107 + 0.10389515463408878255e3 * t117 * t417 - 0.5848223622634646207e0 * t117 * t430 - 0.10254018858216406658e4 * t117 * t438 + 0.34450798614814814813e-2 * t4 * t368 * t30 + 0.71233333333333333332e-1 * t175 * t444 * t109 - 0.53424999999999999999e-1 * t175 * t176 * t209 - 0.85917975471764868594e0 * t175 * t451 * t217 - 0.35089341735807877242e1 * t117 * t456 + 0.35089341735807877242e1 * t117 * t229 * t124 * t460 - 0.51947577317044391277e2 * t117 * t247 * t241 * t465;
        let t470 = t408 + t469;
        let t471 = rho[ip] * t470;
        let t477 = t135 * t135;
        let t478 = 1.0 / t477;
        let t479 = t261 * param_alpha;
        let t480 = t478 * t479;
        let t481 = t63 * t63;
        let t482 = 1.0 / t481;
        let t483 = t267 * sigma[ip];
        let t484 = t482 * t483;
        let t485 = t269 * t269;
        let t486 = t485 * t140;
        let t487 = 1.0 / t486;
        let t488 = t484 * t487;
        let t492 = 1.0 / t7 / t485;
        let t494 = t268 * t39 * t492;
        let t498 = 1.0 / t21 / t314;
        let t499 = t68 * t498;
        let t500 = t139 * t499;
        let t503 = param_gamma * t478;
        let t504 = t503 * t479;
        let t511 = 16.0 / 81.0 * t480 * t488 - 44.0 / 81.0 * t264 * t494 + 154.0 / 81.0 * t138 * t500 - 16.0 / 81.0 * t504 * t488 + 44.0 / 81.0 * t285 * t494 - 154.0 / 81.0 * t147 * t500;
        let t512 = t59 * t511;
        let tv3rho30 = 3.0 * t131 * t291 + t471 * t132 + t134 * t512 + 6.0 * t164 * t150 + 3.0 * t255 * t151 + 3.0 * t60 * t290 + 3.0 * t327 * t80;
        v3rho3[ip] += tv3rho30;
        let t521 = t485 * t70;
        let t523 = t482 / t521;
        let t524 = t523 * t267;
        let t528 = t295 * t272 * sigma[ip];
        let t541 = -2.0 / 27.0 * t480 * t524 + t264 * t528 / 6.0 - 11.0 / 27.0 * t138 * t153 * t278 + 2.0 / 27.0 * t504 * t524 - t285 * t528 / 6.0 + 11.0 / 27.0 * t156 * t157 * t279;
        let t542 = t59 * t541;
        let tv3rho2sigma0 = 2.0 * t131 * t312 + t134 * t542 + 2.0 * t164 * t162 + t255 * t163 + 2.0 * t60 * t311;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t546 = t485 * rho[ip];
        let t547 = 1.0 / t546;
        let t548 = t482 * t547;
        let t549 = t548 * sigma[ip];
        let t561 = t480 * t549 / 36.0 - t264 * t295 * t298 / 27.0 - t504 * t549 / 36.0 + t319 * t320 * t39 * t298 / 27.0;
        let t562 = t59 * t561;
        let tv3rhosigma20 = t131 * t326 + t134 * t562 + t60 * t325;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t564 = 1.0 / t485;
        let t567 = t479 * t482;
        let t571 = -t480 * t482 * t564 / 96.0 + t503 * t567 * t564 / 96.0;
        let t572 = t59 * t571;
        let tv3sigma30 = t134 * t572;
        v3sigma3[ip] += tv3sigma30;
    }
}
