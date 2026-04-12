//! GGA_X_AK13 lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ak13.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ak13_lxc_unpol(
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
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    param_B1: f64,
    param_B2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = t20 * t20;
        let t23 = M_PI * M_PI;
        let t24 = pow_1_3(t23);
        let t25 = 1.0 / t24;
        let t26 = param_B1 * t21 * t25;
        let t27 = f64::sqrt(sigma[ip]);
        let t28 = M_CBRT2;
        let t29 = t27 * t28;
        let t31 = 1.0 / t18 / rho[ip];
        let t32 = t21 * t25;
        let t36 = 1.0 + t32 * t29 * t31 / 12.0;
        let t37 = f64::ln(t36);
        let t38 = t31 * t37;
        let t43 = param_B2 * t21 * t25;
        let t44 = 1.0 + t37;
        let t45 = f64::ln(t44);
        let t46 = t31 * t45;
        let t50 = 1.0 + t26 * t29 * t38 / 12.0 + t43 * t29 * t46 / 12.0;
        let t54 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t50);
        let tzk0 = 2.0 * t54;
        zk[ip] += tzk0;
        let t55 = t18 * t18;
        let t57 = t17 / t55;
        let t61 = rho[ip] * rho[ip];
        let t63 = 1.0 / t18 / t61;
        let t64 = t63 * t37;
        let t69 = t24 * t24;
        let t70 = 1.0 / t69;
        let t71 = param_B1 * t20 * t70;
        let t72 = t28 * t28;
        let t73 = sigma[ip] * t72;
        let t74 = t61 * rho[ip];
        let t76 = 1.0 / t55 / t74;
        let t77 = 1.0 / t36;
        let t78 = t76 * t77;
        let t82 = t63 * t45;
        let t86 = param_B2 * t20;
        let t88 = t86 * t70 * sigma[ip];
        let t89 = t72 * t76;
        let t90 = 1.0 / t44;
        let t91 = t77 * t90;
        let t92 = t89 * t91;
        let t95 = -t26 * t29 * t64 / 9.0 - t71 * t73 * t78 / 18.0 - t43 * t29 * t82 / 9.0 - t88 * t92 / 18.0;
        let t100 = piecewise3(t2, 0.0, -t6 * t57 * t50 / 8.0 - 3.0 / 8.0 * t6 * t19 * t95);
        let tvrho0 = 2.0 * rho[ip] * t100 + 2.0 * t54;
        vrho[ip] += tvrho0;
        let t103 = 1.0 / t27;
        let t104 = t103 * t28;
        let t109 = 1.0 / t55 / t61;
        let t110 = t72 * t109;
        let t117 = t86 * t70;
        let t118 = t110 * t91;
        let t121 = t26 * t104 * t38 / 24.0 + t71 * t110 * t77 / 48.0 + t43 * t104 * t46 / 24.0 + t117 * t118 / 48.0;
        let t125 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t121);
        let tvsigma0 = 2.0 * rho[ip] * t125;
        vsigma[ip] += tvsigma0;
        let t130 = t17 / t55 / rho[ip];
        let t138 = 1.0 / t18 / t74;
        let t139 = t138 * t37;
        let t143 = t61 * t61;
        let t145 = 1.0 / t55 / t143;
        let t146 = t145 * t77;
        let t150 = 1.0 / t23;
        let t151 = param_B1 * t150;
        let t152 = t27 * sigma[ip];
        let t153 = t143 * t61;
        let t154 = 1.0 / t153;
        let t156 = t36 * t36;
        let t157 = 1.0 / t156;
        let t161 = t138 * t45;
        let t165 = t72 * t145;
        let t166 = t165 * t91;
        let t169 = param_B2 * t150;
        let t170 = t169 * t152;
        let t171 = t154 * t157;
        let t172 = t171 * t90;
        let t175 = t44 * t44;
        let t176 = 1.0 / t175;
        let t177 = t171 * t176;
        let t180 = 7.0 / 27.0 * t26 * t29 * t139 + 5.0 / 18.0 * t71 * t73 * t146 - 2.0 / 27.0 * t151 * t152 * t154 * t157 + 7.0 / 27.0 * t43 * t29 * t161 + 5.0 / 18.0 * t88 * t166 - 2.0 / 27.0 * t170 * t172 - 2.0 / 27.0 * t170 * t177;
        let t185 = piecewise3(t2, 0.0, t6 * t130 * t50 / 12.0 - t6 * t57 * t95 / 4.0 - 3.0 / 8.0 * t6 * t19 * t180);
        let tv2rho20 = 2.0 * rho[ip] * t185 + 4.0 * t100;
        v2rho2[ip] += tv2rho20;
        let t197 = t143 * rho[ip];
        let t198 = 1.0 / t197;
        let t199 = t198 * t157;
        let t208 = t169 * t198;
        let t209 = t157 * t90;
        let t210 = t209 * t27;
        let t213 = t157 * t176;
        let t214 = t213 * t27;
        let t217 = -t26 * t104 * t64 / 18.0 - t71 * t89 * t77 / 12.0 + t151 * t199 * t27 / 36.0 - t43 * t104 * t82 / 18.0 - t117 * t92 / 12.0 + t208 * t210 / 36.0 + t208 * t214 / 36.0;
        let t222 = piecewise3(t2, 0.0, -t6 * t57 * t121 / 8.0 - 3.0 / 8.0 * t6 * t19 * t217);
        let tv2rhosigma0 = 2.0 * rho[ip] * t222 + 2.0 * t125;
        v2rhosigma[ip] += tv2rhosigma0;
        let t225 = 1.0 / t152;
        let t226 = t225 * t28;
        let t230 = 1.0 / sigma[ip];
        let t231 = t230 * t72;
        let t232 = t109 * t77;
        let t236 = 1.0 / t143;
        let t237 = t236 * t157;
        let t245 = t86 * t70 * t230;
        let t248 = t169 * t236;
        let t255 = -t26 * t226 * t38 / 48.0 + t71 * t231 * t232 / 96.0 - t151 * t237 * t103 / 96.0 - t43 * t226 * t46 / 48.0 + t245 * t118 / 96.0 - t248 * t209 * t103 / 96.0 - t248 * t213 * t103 / 96.0;
        let t259 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t255);
        let tv2sigma20 = 2.0 * rho[ip] * t259;
        v2sigma2[ip] += tv2sigma20;
        let t262 = t17 * t109;
        let t273 = 1.0 / t18 / t143;
        let t274 = t273 * t37;
        let t279 = 1.0 / t55 / t197;
        let t284 = t143 * t74;
        let t285 = 1.0 / t284;
        let t290 = sigma[ip] * sigma[ip];
        let t291 = t143 * t143;
        let t293 = 1.0 / t18 / t291;
        let t294 = t290 * t293;
        let t297 = 1.0 / t156 / t36;
        let t299 = t25 * t28;
        let t300 = t297 * t21 * t299;
        let t303 = t273 * t45;
        let t307 = t72 * t279;
        let t308 = t307 * t91;
        let t311 = t285 * t157;
        let t318 = t169 * t294;
        let t320 = t32 * t28;
        let t321 = t297 * t90 * t320;
        let t325 = t297 * t176 * t320;
        let t329 = 1.0 / t175 / t44;
        let t331 = t297 * t329 * t320;
        let t334 = -70.0 / 81.0 * t26 * t29 * t274 - 119.0 / 81.0 * t71 * t73 * t279 * t77 + 22.0 / 27.0 * t151 * t152 * t285 * t157 - 4.0 / 243.0 * t151 * t294 * t300 - 70.0 / 81.0 * t43 * t29 * t303 - 119.0 / 81.0 * t88 * t308 + 22.0 / 27.0 * t170 * t311 * t90 + 22.0 / 27.0 * t170 * t311 * t176 - 4.0 / 243.0 * t318 * t321 - 2.0 / 81.0 * t318 * t325 - 4.0 / 243.0 * t318 * t331;
        let t339 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t262 * t50 + t6 * t130 * t95 / 4.0 - 3.0 / 8.0 * t6 * t57 * t180 - 3.0 / 8.0 * t6 * t19 * t334);
        let tv3rho30 = 2.0 * rho[ip] * t339 + 6.0 * t185;
        v3rho3[ip] += tv3rho30;
        let t359 = 1.0 / t18 / t284;
        let t360 = t359 * t297;
        let t363 = sigma[ip] * t21 * t299;
        let t371 = t169 * t154;
        let t376 = t169 * t360;
        let t378 = t90 * sigma[ip] * t320;
        let t382 = t176 * sigma[ip] * t320;
        let t386 = t329 * sigma[ip] * t320;
        let t389 = 7.0 / 54.0 * t26 * t104 * t139 + 37.0 / 108.0 * t71 * t165 * t77 - t151 * t171 * t27 / 4.0 + t151 * t360 * t363 / 162.0 + 7.0 / 54.0 * t43 * t104 * t161 + 37.0 / 108.0 * t117 * t166 - t371 * t210 / 4.0 - t371 * t214 / 4.0 + t376 * t378 / 162.0 + t376 * t382 / 108.0 + t376 * t386 / 162.0;
        let t394 = piecewise3(t2, 0.0, t6 * t130 * t121 / 12.0 - t6 * t57 * t217 / 4.0 - 3.0 / 8.0 * t6 * t19 * t389);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t394 + 4.0 * t222;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t411 = 1.0 / t18 / t153;
        let t420 = t169 * t103;
        let t421 = t199 * t90;
        let t424 = t199 * t176;
        let t427 = t411 * t297;
        let t428 = t169 * t427;
        let t430 = t90 * t21 * t299;
        let t434 = t176 * t21 * t299;
        let t438 = t329 * t21 * t299;
        let t441 = t26 * t226 * t64 / 36.0 - t71 * t231 * t78 / 72.0 + t151 * t103 * t198 * t157 / 18.0 - t151 * t411 * t300 / 432.0 + t43 * t226 * t82 / 36.0 - t245 * t92 / 72.0 + t420 * t421 / 18.0 + t420 * t424 / 18.0 - t428 * t430 / 432.0 - t428 * t434 / 288.0 - t428 * t438 / 432.0;
        let t446 = piecewise3(t2, 0.0, -t6 * t57 * t255 / 8.0 - 3.0 / 8.0 * t6 * t19 * t441);
        let tv3rhosigma20 = 2.0 * rho[ip] * t446 + 2.0 * t259;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t449 = t27 * t290;
        let t450 = 1.0 / t449;
        let t451 = t450 * t28;
        let t455 = 1.0 / t290;
        let t456 = t455 * t72;
        let t461 = 1.0 / t18 / t197;
        let t462 = t461 * t297;
        let t463 = t151 * t462;
        let t465 = t230 * t21 * t299;
        let t472 = t86 * t70 * t455;
        let t475 = t169 * t462;
        let t477 = t90 * t230 * t320;
        let t481 = t176 * t230 * t320;
        let t485 = t329 * t230 * t320;
        let t488 = t26 * t451 * t38 / 32.0 - t71 * t456 * t232 / 64.0 + t463 * t465 / 1152.0 + t43 * t451 * t46 / 32.0 - t472 * t118 / 64.0 + t475 * t477 / 1152.0 + t475 * t481 / 768.0 + t475 * t485 / 1152.0;
        let t492 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t488);
        let tv3sigma30 = 2.0 * rho[ip] * t492;
        v3sigma3[ip] += tv3sigma30;
        let t508 = 1.0 / t291;
        let t513 = t508 * t157;
        let t520 = t291 * rho[ip];
        let t523 = t290 / t18 / t520;
        let t530 = t449 / t55 / t291 / t61;
        let t532 = t156 * t156;
        let t533 = 1.0 / t532;
        let t535 = t70 * t72;
        let t544 = 1.0 / t55 / t153;
        let t557 = t169 * t523;
        let t564 = t169 * t530;
        let t566 = t20 * t70;
        let t567 = t566 * t72;
        let t579 = t175 * t175;
        let t580 = 1.0 / t579;
        let t585 = -1862.0 / 243.0 * t151 * t152 * t508 * t157 - 1862.0 / 243.0 * t170 * t513 * t90 - 1862.0 / 243.0 * t170 * t513 * t176 + 232.0 / 729.0 * t151 * t523 * t300 - 8.0 / 243.0 * t151 * t530 * t533 * t20 * t535 + 910.0 / 243.0 * t43 * t29 * t461 * t45 + 721.0 / 81.0 * t88 * t72 * t544 * t91 + 910.0 / 243.0 * t26 * t29 * t461 * t37 + 721.0 / 81.0 * t71 * t73 * t544 * t77 + 232.0 / 729.0 * t557 * t321 + 116.0 / 243.0 * t557 * t325 + 232.0 / 729.0 * t557 * t331 - 8.0 / 243.0 * t564 * t533 * t90 * t567 - 44.0 / 729.0 * t564 * t533 * t176 * t567 - 16.0 / 243.0 * t564 * t533 * t329 * t567 - 8.0 / 243.0 * t564 * t533 * t580 * t567;
        let t590 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 * t76 * t50 - 5.0 / 9.0 * t6 * t262 * t95 + t6 * t130 * t180 / 2.0 - t6 * t57 * t334 / 2.0 - 3.0 / 8.0 * t6 * t19 * t585);
        let tv4rho40 = 2.0 * rho[ip] * t590 + 8.0 * t339;
        v4rho4[ip] += tv4rho40;
        let t606 = t293 * t297;
        let t612 = 1.0 / t55 / t520 * t533;
        let t627 = t169 * t285;
        let t634 = t169 * t606;
        let t641 = t169 * t612;
        let t658 = -35.0 / 81.0 * t26 * t104 * t274 - 49.0 / 486.0 * t151 * t606 * t363 + t151 * t612 * t152 * t20 * t535 / 81.0 - 35.0 / 81.0 * t43 * t104 * t303 + 317.0 / 162.0 * t151 * t311 * t27 - 91.0 / 54.0 * t71 * t307 * t77 + 317.0 / 162.0 * t627 * t210 + 317.0 / 162.0 * t627 * t214 - 91.0 / 54.0 * t117 * t308 - 49.0 / 486.0 * t634 * t378 - 49.0 / 324.0 * t634 * t382 - 49.0 / 486.0 * t634 * t386 + t641 * t90 * t152 * t567 / 81.0 + 11.0 / 486.0 * t641 * t176 * t152 * t567 + 2.0 / 81.0 * t641 * t329 * t152 * t567 + t641 * t580 * t152 * t567 / 81.0;
        let t663 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t262 * t121 + t6 * t130 * t217 / 4.0 - 3.0 / 8.0 * t6 * t57 * t389 - 3.0 / 8.0 * t6 * t19 * t658);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t663 + 6.0 * t394;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t675 = 1.0 / t55 / t291 * t533;
        let t676 = t169 * t675;
        let t678 = t535 * t27;
        let t727 = -t676 * t329 * t20 * t678 / 108.0 - t676 * t580 * t20 * t678 / 216.0 - t151 * t675 * t566 * t72 * t27 / 216.0 - 7.0 / 108.0 * t43 * t226 * t161 + 7.0 / 216.0 * t245 * t166 + 35.0 / 1296.0 * t376 * t430 + 35.0 / 864.0 * t376 * t434 + 35.0 / 1296.0 * t376 * t438 - 7.0 / 108.0 * t26 * t226 * t139 - 8.0 / 27.0 * t151 * t103 * t154 * t157 - 8.0 / 27.0 * t420 * t172 - 8.0 / 27.0 * t420 * t177 + 7.0 / 216.0 * t71 * t231 * t146 + 35.0 / 1296.0 * t151 * t359 * t300 - t676 * t90 * t20 * t678 / 216.0 - 11.0 / 1296.0 * t676 * t176 * t20 * t678;
        let t732 = piecewise3(t2, 0.0, t6 * t130 * t255 / 12.0 - t6 * t57 * t441 / 4.0 - 3.0 / 8.0 * t6 * t19 * t727);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t732 + 4.0 * t446;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t753 = 1.0 / t55 / t284 * t533;
        let t764 = t169 * t225;
        let t771 = t169 * t753;
        let t792 = -t26 * t451 * t64 / 24.0 + t71 * t456 * t78 / 48.0 - t151 * t225 * t198 * t157 / 48.0 - t151 * t427 * t465 / 216.0 + t151 * t753 * t103 * t20 * t535 / 576.0 - t43 * t451 * t82 / 24.0 + t472 * t92 / 48.0 - t764 * t421 / 48.0 - t764 * t424 / 48.0 - t428 * t477 / 216.0 + t771 * t90 * t103 * t567 / 576.0 + 11.0 / 3456.0 * t771 * t176 * t103 * t567 - t428 * t481 / 144.0 + t771 * t329 * t103 * t567 / 288.0 - t428 * t485 / 216.0 + t771 * t580 * t103 * t567 / 576.0;
        let t797 = piecewise3(t2, 0.0, -t6 * t57 * t488 / 8.0 - 3.0 / 8.0 * t6 * t19 * t792);
        let tv4rhosigma30 = 2.0 * rho[ip] * t797 + 2.0 * t492;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t800 = t290 * sigma[ip];
        let t803 = 1.0 / t27 / t800 * t28;
        let t807 = 1.0 / t800;
        let t816 = t544 * t533;
        let t833 = t169 * t450;
        let t840 = t169 * t816;
        let t869 = -5.0 / 64.0 * t26 * t803 * t38 + 5.0 / 128.0 * t71 * t807 * t72 * t232 + t151 * t450 * t236 * t157 / 128.0 - t151 * t816 * t225 * t20 * t535 / 1536.0 - t463 * t455 * t21 * t299 / 1152.0 - 5.0 / 64.0 * t43 * t803 * t46 + 5.0 / 128.0 * t86 * t70 * t807 * t118 + t833 * t237 * t90 / 128.0 + t833 * t237 * t176 / 128.0 - t840 * t90 * t225 * t567 / 1536.0 - 11.0 / 9216.0 * t840 * t176 * t225 * t567 - t475 * t90 * t455 * t320 / 1152.0 - t840 * t329 * t225 * t567 / 768.0 - t475 * t176 * t455 * t320 / 768.0 - t840 * t580 * t225 * t567 / 1536.0 - t475 * t329 * t455 * t320 / 1152.0;
        let t873 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t869);
        let tv4sigma40 = 2.0 * rho[ip] * t873;
        v4sigma4[ip] += tv4sigma40;
    }
}
