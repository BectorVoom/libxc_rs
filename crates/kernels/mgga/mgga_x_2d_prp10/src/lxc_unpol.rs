//! MGGA_X_2D_PRP10 lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_vxc/mgga_x_2d_prp10.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::lambert_w::{lambert_w};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_2d_prp10_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
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
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rho2lapl: &mut Array<f64>,
    v3rho2tau: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3rhosigmalapl: &mut Array<f64>,
    v3rhosigmatau: &mut Array<f64>,
    v3rholapl2: &mut Array<f64>,
    v3rholapltau: &mut Array<f64>,
    v3rhotau2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v3sigma2lapl: &mut Array<f64>,
    v3sigma2tau: &mut Array<f64>,
    v3sigmalapl2: &mut Array<f64>,
    v3sigmalapltau: &mut Array<f64>,
    v3sigmatau2: &mut Array<f64>,
    v3lapl3: &mut Array<f64>,
    v3lapl2tau: &mut Array<f64>,
    v3lapltau2: &mut Array<f64>,
    v3tau3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho3lapl: &mut Array<f64>,
    v4rho3tau: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rho2sigmalapl: &mut Array<f64>,
    v4rho2sigmatau: &mut Array<f64>,
    v4rho2lapl2: &mut Array<f64>,
    v4rho2lapltau: &mut Array<f64>,
    v4rho2tau2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4rhosigma2lapl: &mut Array<f64>,
    v4rhosigma2tau: &mut Array<f64>,
    v4rhosigmalapl2: &mut Array<f64>,
    v4rhosigmalapltau: &mut Array<f64>,
    v4rhosigmatau2: &mut Array<f64>,
    v4rholapl3: &mut Array<f64>,
    v4rholapl2tau: &mut Array<f64>,
    v4rholapltau2: &mut Array<f64>,
    v4rhotau3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    v4sigma3lapl: &mut Array<f64>,
    v4sigma3tau: &mut Array<f64>,
    v4sigma2lapl2: &mut Array<f64>,
    v4sigma2lapltau: &mut Array<f64>,
    v4sigma2tau2: &mut Array<f64>,
    v4sigmalapl3: &mut Array<f64>,
    v4sigmalapl2tau: &mut Array<f64>,
    v4sigmalapltau2: &mut Array<f64>,
    v4sigmatau3: &mut Array<f64>,
    v4lapl4: &mut Array<f64>,
    v4lapl3tau: &mut Array<f64>,
    v4lapl2tau2: &mut Array<f64>,
    v4lapltau3: &mut Array<f64>,
    v4tau4: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let t2 = rho[ip] * rho[ip];
        let t3 = 1.0 / t2;
        let t7 = 2.0 * tau[ip] * t3;
        let t9 = 1.0 / t2 / rho[ip];
        let t11 = sigma[ip] * t9 / 4.0;
        let t13 = 1.0 / M_PI;
        let t14 = (lapl[ip] * t3 / 2.0 - t7 + t11) * t13;
        let t15 = -0.9999999999e0 < t14;
        let t16 = piecewise3::<f64>(t15, t14, -0.9999999999e0);
        let t17 = f64::exp(-1.0);
        let t19 = lambert_w(t16 * t17);
        let t20 = t19 + 1.0;
        let t21 = t20 / 2.0;
        let t22 = xc_bessel_I0::<f64>(t21);
        let t24 = t7 - t11;
        let t25 = 0.1e-9 < t24;
        let t26 = piecewise3::<f64>(t25, t24, 0.1e-9);
        let t27 = f64::sqrt(t26);
        let t31 = M_SQRT2;
        let t32 = (M_PI * t22 - 4.0 / 3.0 * t13 * t27) * t31;
        let t33 = f64::sqrt(rho[ip]);
        let tvrho0 = -t32 * t33 / 2.0;
        vrho[ip] += tvrho0;
        let t36 = xc_bessel_I1::<f64>(t21);
        let t37 = M_PI * t36;
        let t40 = 4.0 * tau[ip] * t9;
        let t41 = t2 * t2;
        let t42 = 1.0 / t41;
        let t44 = 3.0 / 4.0 * sigma[ip] * t42;
        let t47 = piecewise3::<f64>(t15, (-lapl[ip] * t9 + t40 - t44) * t13, 0.0);
        let t49 = 1.0 / t20;
        let t50 = t19 * t49;
        let t51 = 1.0 / t16;
        let t52 = t50 * t51;
        let t56 = t13 / t27;
        let t58 = piecewise3::<f64>(t25, -t40 + t44, 0.0);
        let t62 = (t37 * t47 * t52 / 2.0 - 2.0 / 3.0 * t56 * t58) * t31;
        let t65 = 1.0 / t33;
        let tv2rho20 = -t62 * t33 / 2.0 - t32 * t65 / 4.0;
        v2rho2[ip] += tv2rho20;
        let t68 = t9 * t13;
        let t70 = piecewise3::<f64>(t15, t68 / 4.0, 0.0);
        let t71 = t37 * t70;
        let t75 = piecewise3::<f64>(t25, -t9 / 4.0, 0.0);
        let t79 = (t71 * t52 / 2.0 - 2.0 / 3.0 * t56 * t75) * t31;
        let tv2rhosigma0 = -t79 * t33 / 2.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t82 = t3 * t13;
        let t84 = piecewise3::<f64>(t15, t82 / 2.0, 0.0);
        let t85 = t84 * t19;
        let t86 = t37 * t85;
        let t87 = t49 * t51;
        let t88 = t31 * t33;
        let t89 = t87 * t88;
        let tv2rholapl0 = -t86 * t89 / 4.0;
        v2rholapl[ip] += tv2rholapl0;
        let t93 = piecewise3::<f64>(t15, -2.0 * t82, 0.0);
        let t94 = t37 * t93;
        let t98 = piecewise3::<f64>(t25, 2.0 * t3, 0.0);
        let t102 = (t94 * t52 / 2.0 - 2.0 / 3.0 * t56 * t98) * t31;
        let tv2rhotau0 = -t102 * t33 / 2.0;
        v2rhotau[ip] += tv2rhotau0;
        let t105 = 1.0 / t21;
        let t107 = -t105 * t36 + t22;
        let t108 = M_PI * t107;
        let t109 = t47 * t47;
        let t110 = t108 * t109;
        let t111 = t19 * t19;
        let t112 = t20 * t20;
        let t113 = 1.0 / t112;
        let t114 = t111 * t113;
        let t115 = t16 * t16;
        let t116 = 1.0 / t115;
        let t117 = t114 * t116;
        let t123 = 12.0 * tau[ip] * t42;
        let t125 = 1.0 / t41 / rho[ip];
        let t127 = 3.0 * sigma[ip] * t125;
        let t130 = piecewise3::<f64>(t15, (3.0 * lapl[ip] * t42 - t123 + t127) * t13, 0.0);
        let t131 = t37 * t130;
        let t134 = t37 * t109;
        let t135 = t19 * t113;
        let t136 = t135 * t116;
        let t140 = 1.0 / t112 / t20;
        let t141 = t111 * t140;
        let t142 = t141 * t116;
        let t145 = t50 * t116;
        let t150 = t13 / t27 / t26;
        let t151 = t58 * t58;
        let t155 = piecewise3::<f64>(t25, t123 - t127, 0.0);
        let t159 = (t110 * t117 / 4.0 + t131 * t52 / 2.0 + t134 * t136 / 2.0 - t134 * t142 / 2.0 - t134 * t145 / 2.0 + t150 * t151 / 3.0 - 2.0 / 3.0 * t56 * t155) * t31;
        let t165 = 1.0 / t33 / rho[ip];
        let tv3rho30 = -t159 * t33 / 2.0 - t62 * t65 / 2.0 + t32 * t165 / 8.0;
        v3rho3[ip] += tv3rho30;
        let t168 = t108 * t47;
        let t169 = t116 * t70;
        let t170 = t114 * t169;
        let t173 = t42 * t13;
        let t175 = piecewise3::<f64>(t15, -3.0 / 4.0 * t173, 0.0);
        let t176 = t37 * t175;
        let t179 = t47 * t19;
        let t180 = t113 * t116;
        let t181 = t179 * t180;
        let t184 = t116 * t47;
        let t185 = t141 * t184;
        let t188 = t50 * t184;
        let t195 = piecewise3::<f64>(t25, 3.0 / 4.0 * t42, 0.0);
        let t199 = (t168 * t170 / 4.0 + t176 * t52 / 2.0 + t71 * t181 / 2.0 - t71 * t185 / 2.0 - t71 * t188 / 2.0 + t150 * t75 * t58 / 3.0 - 2.0 / 3.0 * t56 * t195) * t31;
        let tv3rho2sigma0 = -t199 * t33 / 2.0 - t79 * t65 / 4.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t204 = t47 * t111;
        let t205 = t108 * t204;
        let t206 = t84 * t31;
        let t207 = t206 * t33;
        let t208 = t180 * t207;
        let t211 = piecewise3::<f64>(t15, -t68, 0.0);
        let t212 = t211 * t19;
        let t213 = t37 * t212;
        let t216 = t84 * t47;
        let t217 = t37 * t216;
        let t218 = t116 * t31;
        let t220 = t135 * t218 * t33;
        let t223 = t84 * t111;
        let t224 = t37 * t223;
        let t225 = t140 * t116;
        let t226 = t88 * t47;
        let t227 = t225 * t226;
        let t230 = t49 * t116;
        let t231 = t230 * t226;
        let t234 = t31 * t65;
        let t235 = t87 * t234;
        let tv3rho2lapl0 = -t205 * t208 / 8.0 - t213 * t89 / 4.0 - t217 * t220 / 4.0 + t224 * t227 / 4.0 + t86 * t231 / 4.0 - t86 * t235 / 8.0;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t239 = t114 * t116 * t93;
        let t243 = piecewise3::<f64>(t15, 4.0 * t68, 0.0);
        let t244 = t37 * t243;
        let t257 = piecewise3::<f64>(t25, -4.0 * t9, 0.0);
        let t261 = (t168 * t239 / 4.0 + t244 * t52 / 2.0 + t94 * t181 / 2.0 - t94 * t185 / 2.0 - t94 * t188 / 2.0 + t150 * t98 * t58 / 3.0 - 2.0 / 3.0 * t56 * t257) * t31;
        let tv3rho2tau0 = -t261 * t33 / 2.0 - t102 * t65 / 4.0;
        v3rho2tau[ip] += tv3rho2tau0;
        let t266 = t70 * t70;
        let t267 = t108 * t266;
        let t270 = piecewise3::<f64>(t15, 0.0, 0.0);
        let t271 = t37 * t270;
        let t273 = t271 * t52 / 2.0;
        let t274 = t37 * t266;
        let t281 = t75 * t75;
        let t284 = piecewise3::<f64>(t25, 0.0, 0.0);
        let t286 = 2.0 / 3.0 * t56 * t284;
        let t288 = (t267 * t117 / 4.0 + t273 + t274 * t136 / 2.0 - t274 * t142 / 2.0 - t274 * t145 / 2.0 + t150 * t281 / 3.0 - t286) * t31;
        let tv3rhosigma20 = -t288 * t33 / 2.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t291 = t70 * t111;
        let t292 = t108 * t291;
        let t295 = t270 * t19;
        let t296 = t37 * t295;
        let t298 = t296 * t89 / 4.0;
        let t299 = t84 * t70;
        let t300 = t37 * t299;
        let t303 = t88 * t70;
        let t304 = t225 * t303;
        let t307 = t230 * t303;
        let tv3rhosigmalapl0 = -t292 * t208 / 8.0 - t298 - t300 * t220 / 4.0 + t224 * t304 / 4.0 + t86 * t307 / 4.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let t310 = t108 * t70;
        let t313 = t70 * t19;
        let t314 = t313 * t180;
        let t317 = t141 * t169;
        let t320 = t50 * t169;
        let t323 = t98 * t75;
        let t327 = (t310 * t239 / 4.0 + t273 + t94 * t314 / 2.0 - t94 * t317 / 2.0 - t94 * t320 / 2.0 + t150 * t323 / 3.0 - t286) * t31;
        let tv3rhosigmatau0 = -t327 * t33 / 2.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let t330 = t84 * t84;
        let t331 = t330 * t111;
        let t332 = t108 * t331;
        let t333 = t180 * t88;
        let t336 = t330 * t19;
        let t337 = t37 * t336;
        let t340 = t37 * t331;
        let tv3rholapl20 = -t332 * t333 / 8.0 - t298 - t337 * t333 / 4.0 + t340 * t225 * t88 / 4.0 + t337 * t230 * t88 / 4.0;
        v3rholapl2[ip] += tv3rholapl20;
        let t347 = t108 * t84;
        let t350 = t85 * t180;
        let t353 = t116 * t84;
        let t354 = t141 * t353;
        let t357 = t50 * t353;
        let t361 = (t347 * t239 / 4.0 + t273 + t94 * t350 / 2.0 - t94 * t354 / 2.0 - t94 * t357 / 2.0) * t31;
        let tv3rholapltau0 = -t361 * t33 / 2.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let t364 = t93 * t93;
        let t365 = t108 * t364;
        let t368 = t37 * t364;
        let t375 = t98 * t98;
        let t379 = (t365 * t117 / 4.0 + t273 + t368 * t136 / 2.0 - t368 * t142 / 2.0 - t368 * t145 / 2.0 + t150 * t375 / 3.0 - t286) * t31;
        let tv3rhotau20 = -t379 * t33 / 2.0;
        v3rhotau2[ip] += tv3rhotau20;
        let t382 = t109 * t47;
        let t383 = t108 * t382;
        let t384 = t111 * t19;
        let t385 = t112 * t112;
        let t386 = 1.0 / t385;
        let t387 = t384 * t386;
        let t389 = 1.0 / t115 / t16;
        let t390 = t387 * t389;
        let t395 = t37 * t382;
        let t396 = t111 * t386;
        let t397 = t396 * t389;
        let t401 = 1.0 / t385 / t20;
        let t402 = t384 * t401;
        let t403 = t402 * t389;
        let t406 = t141 * t389;
        let t409 = t26 * t26;
        let t412 = t13 / t27 / t409;
        let t420 = t21 * t21;
        let t422 = 1.0 / t420 * t36;
        let t425 = t105 * t107;
        let t430 = M_PI * (t36 * t47 * t52 / 2.0 + t422 * t47 * t52 / 2.0 - t425 * t47 * t52 / 2.0);
        let t437 = 48.0 * tau[ip] * t125;
        let t441 = 15.0 * sigma[ip] / t41 / t2;
        let t444 = piecewise3::<f64>(t15, (-12.0 * lapl[ip] * t125 + t437 - t441) * t13, 0.0);
        let t448 = t116 * t130;
        let t452 = t114 * t389;
        let t457 = t135 * t389;
        let t462 = t50 * t389;
        let t466 = t19 * t140;
        let t467 = t466 * t389;
        let t471 = piecewise3::<f64>(t25, -t437 + t441, 0.0);
        let t474 = -3.0 / 4.0 * t383 * t390 - 3.0 / 2.0 * t131 * t185 - 2.0 * t395 * t397 + 3.0 / 2.0 * t395 * t403 + 3.0 / 4.0 * t383 * t406 - t412 * t151 * t58 / 2.0 + t150 * t58 * t155 + t430 * t109 * t117 / 4.0 + t37 * t444 * t52 / 2.0 + 3.0 / 4.0 * t168 * t114 * t448 - 3.0 / 4.0 * t383 * t452 - 3.0 / 2.0 * t131 * t188 - 3.0 / 2.0 * t395 * t457 + 3.0 / 2.0 * t395 * t406 + t395 * t462 + 3.0 / 2.0 * t131 * t181 + t395 * t467 / 2.0 - 2.0 / 3.0 * t56 * t471;
        let tv4rho40 = -t474 * t31 * t33 / 2.0 - 3.0 / 4.0 * t159 * t65 + 3.0 / 8.0 * t62 * t165 - 3.0 / 16.0 * t32 / t33 / t2;
        v4rho4[ip] += tv4rho40;
        let t487 = t109 * t19;
        let t488 = t140 * t389;
        let t489 = t487 * t488;
        let t492 = t109 * t111;
        let t493 = t386 * t389;
        let t494 = t492 * t493;
        let t497 = t389 * t109;
        let t498 = t402 * t497;
        let t501 = t389 * t70;
        let t502 = t141 * t501;
        let t505 = t387 * t501;
        let t514 = t114 * t501;
        let t517 = t113 * t389;
        let t518 = t487 * t517;
        let t521 = t141 * t497;
        let t524 = -t176 * t185 + t71 * t489 / 2.0 - 2.0 * t71 * t494 + 3.0 / 2.0 * t71 * t498 + 3.0 / 4.0 * t110 * t502 - 3.0 / 4.0 * t110 * t505 + 2.0 / 3.0 * t150 * t195 * t58 + t150 * t75 * t155 / 3.0 - 3.0 / 4.0 * t110 * t514 - 3.0 / 2.0 * t71 * t518 + 3.0 / 2.0 * t71 * t521;
        let t525 = t50 * t497;
        let t532 = piecewise3::<f64>(t15, 3.0 * t125 * t13, 0.0);
        let t536 = t430 * t47;
        let t539 = t108 * t130;
        let t542 = t116 * t175;
        let t543 = t114 * t542;
        let t548 = t130 * t19 * t180;
        let t551 = t141 * t448;
        let t554 = t50 * t448;
        let t559 = piecewise3::<f64>(t25, -3.0 * t125, 0.0);
        let t562 = t71 * t525 - t412 * t75 * t151 / 2.0 + t37 * t532 * t52 / 2.0 + t536 * t170 / 4.0 + t539 * t170 / 4.0 + t168 * t543 / 2.0 - t176 * t188 + t71 * t548 / 2.0 - t71 * t551 / 2.0 - t71 * t554 / 2.0 + t176 * t181 - 2.0 / 3.0 * t56 * t559;
        let tv4rho3sigma0 = -(t524 + t562) * t31 * t33 / 2.0 - t199 * t65 / 2.0 + t79 * t165 / 8.0;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t572 = piecewise3::<f64>(t15, 3.0 * t173, 0.0);
        let t583 = t108 * t492;
        let t584 = t517 * t207;
        let t588 = t180 * t206 * t65;
        let t592 = t37 * t84 * t109;
        let t593 = t389 * t31;
        let t594 = t593 * t33;
        let t595 = t135 * t594;
        let t599 = t135 * t218 * t65;
        let t602 = t88 * t109;
        let t606 = t234 * t47;
        let t610 = t49 * t389;
        let t621 = -t37 * t572 * t19 * t89 / 4.0 - t213 * t235 / 4.0 + t86 * t87 * t31 * t165 / 16.0 + 3.0 / 8.0 * t583 * t584 - t205 * t588 / 8.0 + 3.0 / 4.0 * t592 * t595 - t217 * t599 / 4.0 - 3.0 / 4.0 * t224 * t488 * t602 + t224 * t225 * t606 / 4.0 - t86 * t610 * t602 / 2.0 + t86 * t230 * t606 / 4.0 - t37 * t211 * t47 * t220 / 2.0;
        let t623 = t37 * t211 * t111;
        let t626 = t466 * t594;
        let t629 = t488 * t207;
        let t634 = t493 * t207;
        let t637 = t396 * t594;
        let t639 = t84 * t384;
        let t640 = t37 * t639;
        let t641 = t401 * t389;
        let t653 = t211 * t31 * t33;
        let t654 = t180 * t653;
        let t663 = t88 * t130;
        let t670 = t623 * t227 / 2.0 - t592 * t626 / 4.0 - 3.0 / 8.0 * t583 * t629 + 3.0 / 8.0 * t108 * t109 * t384 * t634 + t592 * t637 - 3.0 / 4.0 * t640 * t641 * t602 - t430 * t204 * t208 / 8.0 - t108 * t130 * t111 * t208 / 8.0 - t205 * t654 / 4.0 + t213 * t231 / 2.0 - t37 * t84 * t130 * t220 / 4.0 + t224 * t225 * t663 / 4.0 + t86 * t230 * t663 / 4.0;
        let tv4rho3lapl0 = t621 + t670;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let t671 = t389 * t93;
        let t672 = t141 * t671;
        let t675 = t387 * t671;
        let t691 = t114 * t671;
        let t698 = 3.0 / 4.0 * t110 * t672 - 3.0 / 4.0 * t110 * t675 - 2.0 * t94 * t494 + 3.0 / 2.0 * t94 * t498 - t244 * t185 + t94 * t489 / 2.0 + 2.0 / 3.0 * t150 * t257 * t58 + t150 * t98 * t155 / 3.0 - 3.0 / 4.0 * t110 * t691 - 3.0 / 2.0 * t94 * t518 + 3.0 / 2.0 * t94 * t521;
        let t704 = piecewise3::<f64>(t15, -12.0 * t173, 0.0);
        let t712 = t116 * t243;
        let t713 = t114 * t712;
        let t725 = piecewise3::<f64>(t25, 12.0 * t42, 0.0);
        let t728 = t94 * t525 - t412 * t98 * t151 / 2.0 + t37 * t704 * t52 / 2.0 + t536 * t239 / 4.0 + t539 * t239 / 4.0 + t168 * t713 / 2.0 - t244 * t188 + t94 * t548 / 2.0 - t94 * t551 / 2.0 - t94 * t554 / 2.0 + t244 * t181 - 2.0 / 3.0 * t56 * t725;
        let tv4rho3tau0 = -(t698 + t728) * t31 * t33 / 2.0 - t261 * t65 / 2.0 + t102 * t165 / 8.0;
        v4rho3tau[ip] += tv4rho3tau0;
        let t737 = t389 * t47;
        let t738 = t141 * t737;
        let t741 = t387 * t737;
        let t745 = t271 * t185 / 2.0;
        let t746 = t396 * t737;
        let t749 = t402 * t737;
        let t760 = t150 * t284 * t58 / 3.0;
        let t764 = 3.0 / 4.0 * t267 * t738 - 3.0 / 4.0 * t267 * t741 - t745 - 2.0 * t274 * t746 + 3.0 / 2.0 * t274 * t749 - t412 * t281 * t58 / 2.0 + 2.0 / 3.0 * t150 * t75 * t195 + t760 - t286 + t273 + t430 * t266 * t117 / 4.0;
        let t767 = t114 * t737;
        let t771 = t271 * t188 / 2.0;
        let t772 = t116 * t270;
        let t773 = t114 * t772;
        let t775 = t168 * t773 / 4.0;
        let t776 = t135 * t542;
        let t778 = t135 * t737;
        let t781 = t141 * t542;
        let t785 = t50 * t542;
        let t787 = t50 * t737;
        let t790 = t271 * t181 / 2.0;
        let t791 = t179 * t488;
        let t794 = t310 * t543 / 2.0 - 3.0 / 4.0 * t267 * t767 - t771 + t775 + t71 * t776 - 3.0 / 2.0 * t274 * t778 - t71 * t781 + 3.0 / 2.0 * t274 * t738 - t71 * t785 + t274 * t787 + t790 + t274 * t791 / 2.0;
        let tv4rho2sigma20 = -(t764 + t794) * t31 * t33 / 2.0 - t288 * t65 / 4.0;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t802 = t296 * t235 / 8.0;
        let t805 = t389 * t84;
        let t806 = t805 * t226;
        let t811 = t517 * t226;
        let t820 = t493 * t226;
        let t826 = t593 * t33 * t70 * t47;
        let t833 = t70 * t384;
        let t849 = t37 * t270 * t47 * t220 / 4.0;
        let t852 = t234 * t70;
        let t856 = -t802 - t298 + 3.0 / 8.0 * t108 * t291 * t113 * t806 + 3.0 / 4.0 * t37 * t299 * t19 * t811 - t37 * t299 * t47 * t626 / 4.0 + t37 * t299 * t111 * t820 - 3.0 / 4.0 * t37 * t639 * t401 * t826 - 3.0 / 8.0 * t108 * t291 * t140 * t806 + 3.0 / 8.0 * t108 * t833 * t386 * t806 - 3.0 / 4.0 * t37 * t223 * t140 * t826 - t37 * t85 * t49 * t826 / 2.0 - t849 - t300 * t599 / 8.0 + t224 * t225 * t852 / 8.0;
        let t862 = t270 * t111;
        let t863 = t37 * t862;
        let t865 = t863 * t227 / 4.0;
        let t876 = t296 * t231 / 4.0;
        let t878 = t270 * t31 * t33;
        let t879 = t180 * t878;
        let t881 = t205 * t879 / 8.0;
        let t892 = t88 * t175;
        let t901 = t86 * t230 * t852 / 8.0 - t292 * t588 / 16.0 + t865 - t430 * t291 * t208 / 8.0 - t108 * t175 * t111 * t208 / 8.0 - t292 * t654 / 8.0 + t876 - t881 - t37 * t211 * t70 * t220 / 4.0 - t37 * t84 * t175 * t220 / 4.0 + t623 * t304 / 4.0 + t224 * t225 * t892 / 4.0 + t213 * t307 / 4.0 + t86 * t230 * t892 / 4.0;
        let tv4rho2sigmalapl0 = t856 + t901;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let t905 = t93 * t47;
        let t906 = t488 * t905;
        let t910 = t493 * t905;
        let t913 = t93 * t70;
        let t914 = t37 * t913;
        let t918 = t37 * t93 * t384;
        let t919 = t70 * t47;
        let t925 = t517 * t905;
        let t931 = t37 * t93 * t111;
        let t936 = t37 * t93 * t19;
        let t943 = -t412 * t323 * t58 / 2.0 - t286 + 3.0 / 4.0 * t292 * t906 - 3.0 / 4.0 * t108 * t833 * t910 - 2.0 * t914 * t746 + 3.0 / 2.0 * t918 * t641 * t919 + t914 * t791 / 2.0 - 3.0 / 4.0 * t292 * t925 - 3.0 / 2.0 * t914 * t778 + 3.0 / 2.0 * t931 * t488 * t919 + t936 * t610 * t919 - t244 * t320 / 2.0 - t94 * t785 / 2.0 - t771;
        let t966 = t775 + t790 + t430 * t70 * t239 / 4.0 + t108 * t175 * t239 / 4.0 + t310 * t713 / 4.0 + t244 * t314 / 2.0 + t94 * t776 / 2.0 - t244 * t317 / 2.0 - t94 * t781 / 2.0 - t745 + t760 + t150 * t98 * t195 / 3.0 + t150 * t257 * t75 / 3.0 + t273;
        let tv4rho2sigmatau0 = -(t943 + t966) * t31 * t33 / 2.0 - t327 * t65 / 4.0;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let t973 = t488 * t226;
        let t976 = t330 * t384;
        let t977 = t108 * t976;
        let t981 = t37 * t976;
        let t992 = -3.0 / 8.0 * t332 * t973 + 3.0 / 8.0 * t977 * t820 + t340 * t820 - 3.0 / 4.0 * t981 * t641 * t226 - t37 * t330 * t47 * t626 / 4.0 - t849 + t865 - t298 - t430 * t331 * t333 / 8.0 + t876 - t802 - t881;
        let t993 = t108 * t223;
        let t998 = t180 * t234;
        let t1024 = -t993 * t654 / 4.0 + 3.0 / 8.0 * t332 * t811 - t332 * t998 / 16.0 - t86 * t654 / 2.0 + 3.0 / 4.0 * t337 * t811 - t337 * t998 / 8.0 + t224 * t225 * t653 / 2.0 - 3.0 / 4.0 * t340 * t973 + t340 * t225 * t234 / 8.0 + t86 * t230 * t653 / 2.0 - t337 * t610 * t226 / 2.0 + t337 * t230 * t234 / 8.0;
        let tv4rho2lapl20 = t992 + t1024;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let t1026 = t37 * t93 * t84;
        let t1034 = t108 * t639;
        let t1048 = -2.0 * t1026 * t746 + 3.0 / 2.0 * t918 * t641 * t216 + 3.0 / 4.0 * t993 * t906 - 3.0 / 4.0 * t1034 * t910 + t1026 * t791 / 2.0 - t745 - 3.0 / 4.0 * t993 * t925 - 3.0 / 2.0 * t1026 * t778 + 3.0 / 2.0 * t931 * t488 * t216 + t936 * t610 * t216 + t273;
        let t1064 = t116 * t211;
        let t1073 = -t771 + t775 + t430 * t84 * t239 / 4.0 + t108 * t211 * t239 / 4.0 + t347 * t713 / 4.0 + t244 * t350 / 2.0 + t94 * t212 * t180 / 2.0 - t244 * t354 / 2.0 - t94 * t141 * t1064 / 2.0 - t244 * t357 / 2.0 - t94 * t50 * t1064 / 2.0 + t790;
        let tv4rho2lapltau0 = -(t1048 + t1073) * t31 * t33 / 2.0 - t361 * t65 / 4.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let t1097 = -t412 * t375 * t58 / 2.0 + 2.0 / 3.0 * t150 * t98 * t257 + 3.0 / 4.0 * t365 * t738 - 3.0 / 4.0 * t365 * t741 - 2.0 * t368 * t746 + 3.0 / 2.0 * t368 * t749 - t745 + t760 - t286 + t273 + t430 * t364 * t117 / 4.0;
        let t1116 = -t771 + t775 + t108 * t93 * t713 / 2.0 - 3.0 / 4.0 * t365 * t767 + t94 * t135 * t712 - 3.0 / 2.0 * t368 * t778 - t94 * t141 * t712 + 3.0 / 2.0 * t368 * t738 - t94 * t50 * t712 + t368 * t787 + t790 + t368 * t791 / 2.0;
        let tv4rho2tau20 = -(t1097 + t1116) * t31 * t33 / 2.0 - t379 * t65 / 4.0;
        v4rho2tau2[ip] += tv4rho2tau20;
        let t1131 = M_PI * (t36 * t70 * t52 / 2.0 + t422 * t70 * t52 / 2.0 - t425 * t70 * t52 / 2.0);
        let t1139 = t150 * t75 * t284;
        let t1140 = t266 * t70;
        let t1141 = t37 * t1140;
        let t1146 = t108 * t1140;
        let t1151 = t271 * t317;
        let t1153 = t310 * t773;
        let t1157 = t271 * t320;
        let t1164 = t271 * t314;
        let t1168 = t1131 * t266 * t117 / 4.0 - t412 * t281 * t75 / 2.0 + t1139 - 2.0 * t1141 * t397 + 3.0 / 2.0 * t1141 * t403 + 3.0 / 4.0 * t1146 * t406 - 3.0 / 4.0 * t1146 * t390 - 3.0 / 2.0 * t1151 - t286 + t273 + 3.0 / 4.0 * t1153 - 3.0 / 4.0 * t1146 * t452 - 3.0 / 2.0 * t1157 - 3.0 / 2.0 * t1141 * t457 + 3.0 / 2.0 * t1141 * t406 + t1141 * t462 + 3.0 / 2.0 * t1164 + t1141 * t467 / 2.0;
        let tv4rhosigma30 = -t1168 * t31 * t33 / 2.0;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t1172 = t863 * t304;
        let t1175 = t37 * t84 * t266;
        let t1178 = t266 * t111;
        let t1179 = t108 * t1178;
        let t1187 = t88 * t266;
        let t1193 = t37 * t270 * t70 * t220;
        let t1209 = t108 * t862 * t208;
        let t1211 = t292 * t879;
        let t1213 = t296 * t307;
        let t1217 = t37 * t84 * t270 * t220;
        let t1220 = t224 * t225 * t878;
        let t1223 = t86 * t230 * t878;
        let tv4rhosigma2lapl0 = t1172 / 2.0 - t1175 * t626 / 4.0 - 3.0 / 8.0 * t1179 * t629 + 3.0 / 8.0 * t108 * t266 * t384 * t634 + t1175 * t637 - 3.0 / 4.0 * t640 * t641 * t1187 - t1193 / 2.0 + 3.0 / 8.0 * t1179 * t584 + 3.0 / 4.0 * t1175 * t595 - 3.0 / 4.0 * t224 * t488 * t1187 - t86 * t610 * t1187 / 2.0 - t298 - t1131 * t291 * t208 / 8.0 - t1209 / 8.0 - t1211 / 4.0 + t1213 / 2.0 - t1217 / 4.0 + t1220 / 4.0 + t1223 / 4.0;
        v4rhosigma2lapl[ip] += tv4rhosigma2lapl0;
        let t1226 = t150 * t98 * t284;
        let t1236 = t389 * t266;
        let t1240 = t266 * t19;
        let t1252 = t1226 / 3.0 + 2.0 / 3.0 * t1139 + 3.0 / 4.0 * t267 * t672 - 3.0 / 4.0 * t267 * t675 - 2.0 * t94 * t1178 * t493 + 3.0 / 2.0 * t94 * t402 * t1236 + t94 * t1240 * t488 / 2.0 - t1151 - t412 * t98 * t281 / 2.0 - 3.0 / 4.0 * t267 * t691 - 3.0 / 2.0 * t94 * t1240 * t517;
        let t1263 = t108 * t270 * t239;
        let t1264 = t1263 / 4.0;
        let t1266 = t94 * t295 * t180;
        let t1267 = t1266 / 2.0;
        let t1269 = t94 * t141 * t772;
        let t1270 = t1269 / 2.0;
        let t1272 = t94 * t50 * t772;
        let t1273 = t1272 / 2.0;
        let t1274 = 3.0 / 2.0 * t94 * t141 * t1236 + t94 * t50 * t1236 - t286 + t273 + t1153 / 2.0 - t1157 + t1131 * t70 * t239 / 4.0 + t1264 + t1267 - t1270 - t1273 + t1164;
        let tv4rhosigma2tau0 = -(t1252 + t1274) * t31 * t33 / 2.0;
        v4rhosigma2tau[ip] += tv4rhosigma2tau0;
        let t1279 = t488 * t303;
        let t1282 = t493 * t303;
        let t1304 = t517 * t303;
        let tv4rhosigmalapl20 = -3.0 / 8.0 * t332 * t1279 + 3.0 / 8.0 * t977 * t1282 + t340 * t1282 - 3.0 / 4.0 * t981 * t641 * t303 + t1172 / 4.0 - t37 * t330 * t70 * t626 / 4.0 - t1193 / 4.0 - t298 - t1131 * t331 * t333 / 8.0 - t1209 / 4.0 - t1211 / 8.0 + t1213 / 4.0 - t1217 / 2.0 + t1220 / 2.0 + t1223 / 2.0 + 3.0 / 8.0 * t332 * t1304 + 3.0 / 4.0 * t337 * t1304 - 3.0 / 4.0 * t340 * t1279 - t337 * t610 * t303 / 2.0;
        v4rhosigmalapl2[ip] += tv4rhosigmalapl20;
        let t1320 = t396 * t501;
        let t1326 = t313 * t488;
        let t1329 = t1151 / 2.0;
        let t1333 = t135 * t501;
        let t1341 = 3.0 / 4.0 * t993 * t488 * t913 - 3.0 / 4.0 * t1034 * t493 * t913 - 2.0 * t1026 * t1320 + 3.0 / 2.0 * t918 * t641 * t299 + t1026 * t1326 / 2.0 - t1329 - 3.0 / 4.0 * t993 * t517 * t913 - 3.0 / 2.0 * t1026 * t1333 + 3.0 / 2.0 * t931 * t488 * t299 + t936 * t610 * t299 + t273;
        let t1342 = t1153 / 4.0;
        let t1343 = t1157 / 2.0;
        let t1347 = t347 * t773;
        let t1348 = t1347 / 4.0;
        let t1349 = t271 * t350;
        let t1350 = t1349 / 2.0;
        let t1351 = t271 * t354;
        let t1352 = t1351 / 2.0;
        let t1353 = t271 * t357;
        let t1354 = t1353 / 2.0;
        let t1355 = t1164 / 2.0;
        let t1356 = t1342 - t1343 + t1264 + t1267 - t1270 - t1273 + t1131 * t84 * t239 / 4.0 + t1348 + t1350 - t1352 - t1354 + t1355;
        let tv4rhosigmalapltau0 = -(t1341 + t1356) * t31 * t33 / 2.0;
        v4rhosigmalapltau[ip] += tv4rhosigmalapltau0;
        let t1378 = -t412 * t375 * t75 / 2.0 + 2.0 / 3.0 * t1226 + t1131 * t364 * t117 / 4.0 + t1139 / 3.0 - 2.0 * t368 * t1320 + 3.0 / 2.0 * t368 * t402 * t501 + 3.0 / 4.0 * t365 * t502 - 3.0 / 4.0 * t365 * t505 - t1329 - t286 + t273;
        let t1379 = t1263 / 2.0;
        let t1390 = t1342 - t1343 + t1379 + t1266 - t1269 - t1272 - 3.0 / 4.0 * t365 * t514 - 3.0 / 2.0 * t368 * t1333 + 3.0 / 2.0 * t368 * t502 + t368 * t50 * t501 + t1355 + t368 * t1326 / 2.0;
        let tv4rhosigmatau20 = -(t1378 + t1390) * t31 * t33 / 2.0;
        v4rhosigmatau2[ip] += tv4rhosigmatau20;
        let t1395 = t330 * t84;
        let t1396 = t1395 * t384;
        let t1401 = t1395 * t111;
        let t1402 = t108 * t1401;
        let t1403 = t488 * t88;
        let t1407 = t493 * t88;
        let t1410 = t37 * t1401;
        let t1413 = t37 * t1395 * t19;
        let t1424 = M_PI * (t36 * t84 * t52 / 2.0 + t422 * t84 * t52 / 2.0 - t425 * t84 * t52 / 2.0);
        let t1432 = t517 * t88;
        let tv4rholapl30 = -3.0 / 4.0 * t37 * t1396 * t641 * t88 - 3.0 / 8.0 * t1402 * t1403 + 3.0 / 8.0 * t108 * t1396 * t1407 + t1410 * t1407 - t1413 * t1403 / 4.0 - t298 - t1424 * t331 * t333 / 8.0 - 3.0 / 8.0 * t1209 - 3.0 / 4.0 * t1217 + 3.0 / 4.0 * t1220 + 3.0 / 4.0 * t1223 + 3.0 / 8.0 * t1402 * t1432 + 3.0 / 4.0 * t1413 * t1432 - 3.0 / 4.0 * t1410 * t1403 - t1413 * t610 * t88 / 2.0;
        v4rholapl3[ip] += tv4rholapl30;
        let t1442 = t108 * t330;
        let t1450 = t389 * t330;
        let t1471 = 3.0 / 4.0 * t1442 * t672 - 3.0 / 4.0 * t1442 * t675 - 2.0 * t94 * t331 * t493 + 3.0 / 2.0 * t94 * t402 * t1450 + t94 * t336 * t488 / 2.0 - 3.0 / 4.0 * t1442 * t691 - 3.0 / 2.0 * t94 * t336 * t517 + 3.0 / 2.0 * t94 * t141 * t1450 + t94 * t50 * t1450 + t273 + t1264 + t1267 - t1270 - t1273 + t1347 / 2.0 + t1349 - t1351 - t1353 + t1424 * t84 * t239 / 4.0;
        let tv4rholapl2tau0 = -t1471 * t31 * t33 / 2.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let t1478 = t141 * t805;
        let t1503 = t1424 * t364 * t117 / 4.0 + 3.0 / 4.0 * t365 * t1478 - 3.0 / 4.0 * t365 * t387 * t805 - 2.0 * t368 * t396 * t805 + 3.0 / 2.0 * t368 * t402 * t805 - 3.0 / 4.0 * t365 * t114 * t805 - 3.0 / 2.0 * t368 * t135 * t805 + t273 + 3.0 / 2.0 * t368 * t1478 + t368 * t50 * t805 + t1379 + t1266 - t1269 - t1272 + t1348 + t1350 - t1352 - t1354 + t368 * t85 * t488 / 2.0;
        let tv4rholapltau20 = -t1503 * t31 * t33 / 2.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let t1507 = t364 * t93;
        let t1508 = t108 * t1507;
        let t1511 = t37 * t1507;
        let t1546 = -3.0 / 4.0 * t1508 * t452 - 3.0 / 2.0 * t1511 * t457 + 3.0 / 2.0 * t1511 * t406 + t1511 * t462 - t412 * t375 * t98 / 2.0 + M_PI * (t36 * t93 * t52 / 2.0 + t422 * t93 * t52 / 2.0 - t425 * t93 * t52 / 2.0) * t364 * t117 / 4.0 + t1226 + 3.0 / 4.0 * t1508 * t406 - 3.0 / 4.0 * t1508 * t390 - 2.0 * t1511 * t397 + 3.0 / 2.0 * t1511 * t403 - t286 + t273 + 3.0 / 4.0 * t1263 + 3.0 / 2.0 * t1266 - 3.0 / 2.0 * t1269 - 3.0 / 2.0 * t1272 + t1511 * t467 / 2.0;
        let tv4rhotau30 = -t1546 * t31 * t33 / 2.0;
        v4rhotau3[ip] += tv4rhotau30;
    }
}
