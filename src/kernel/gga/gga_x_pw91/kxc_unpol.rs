//! GGA_X_PW91 kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pw91.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pw91_kxc_unpol(
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
    param_a: f64,
    param_alpha: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_expo: f64,
    param_f: f64,
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
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t37 = f64::exp(-param_alpha * t20 * t25 * t34 / 24.0);
        let t40 = (param_d * t37 + param_c) * t20;
        let t41 = t40 * t25;
        let t44 = t20 * t20;
        let t45 = 1.0 / t23;
        let t46 = t44 * t45;
        let t47 = f64::sqrt(sigma[ip]);
        let t50 = 1.0 / t18 / rho[ip];
        let t51 = t47 * t27 * t50;
        let t54 = f64::powf(t46 * t51 / 12.0, param_expo);
        let t55 = param_f * t54;
        let t56 = t41 * t34 / 24.0 - t55;
        let t57 = t46 * t47;
        let t63 = f64::ln(param_b * t44 * t45 * t51 / 12.0 + f64::sqrt(pow_2(param_b * t44 * t45 * t51 / 12.0) + 1.0));
        let t64 = param_a * t63;
        let t65 = t27 * t50 * t64;
        let t68 = 1.0 + t57 * t65 / 12.0 + t55;
        let t69 = 1.0 / t68;
        let t71 = t56 * t69 + 1.0;
        let t75 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t71);
        let tzk0 = 2.0 * t75;
        zk[ip] += tzk0;
        let t77 = t17 / t31;
        let t81 = param_d * param_alpha;
        let t83 = 1.0 / t23 / t22;
        let t84 = t44 * t83;
        let t85 = t81 * t84;
        let t86 = sigma[ip] * sigma[ip];
        let t87 = t86 * t27;
        let t88 = t30 * t30;
        let t89 = t88 * t30;
        let t91 = 1.0 / t18 / t89;
        let t92 = t91 * t37;
        let t96 = t30 * rho[ip];
        let t98 = 1.0 / t31 / t96;
        let t102 = 1.0 / rho[ip];
        let t105 = 4.0 / 3.0 * t55 * param_expo * t102;
        let t106 = t85 * t87 * t92 / 108.0 - t41 * t29 * t98 / 9.0 + t105;
        let t108 = t68 * t68;
        let t109 = 1.0 / t108;
        let t110 = t56 * t109;
        let t114 = t27 / t18 / t30 * t64;
        let t117 = t20 * t25;
        let t118 = t117 * t29;
        let t120 = param_b * param_b;
        let t125 = 6.0 * t120 * t20 * t25 * t34 + 144.0;
        let t126 = f64::sqrt(t125);
        let t128 = param_b / t126;
        let t129 = t98 * param_a * t128;
        let t132 = -t57 * t114 / 9.0 - 2.0 / 3.0 * t118 * t129 - t105;
        let t134 = t106 * t69 - t110 * t132;
        let t139 = piecewise3(t2, 0.0, -t6 * t77 * t71 / 8.0 - 3.0 / 8.0 * t6 * t19 * t134);
        let tvrho0 = 2.0 * rho[ip] * t139 + 2.0 * t75;
        vrho[ip] += tvrho0;
        let t142 = t88 * rho[ip];
        let t144 = 1.0 / t18 / t142;
        let t145 = t27 * t144;
        let t146 = t37 * sigma[ip];
        let t150 = t25 * t28;
        let t154 = 1.0 / sigma[ip];
        let t157 = t55 * param_expo * t154 / 2.0;
        let t158 = -t85 * t145 * t146 / 288.0 + t40 * t150 * t33 / 24.0 - t157;
        let t161 = t46 / t47;
        let t164 = t117 * t28;
        let t166 = t33 * param_a * t128;
        let t169 = t161 * t65 / 24.0 + t164 * t166 / 4.0 + t157;
        let t171 = -t110 * t169 + t158 * t69;
        let t175 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t171);
        let tvsigma0 = 2.0 * rho[ip] * t175;
        vsigma[ip] += tvsigma0;
        let t180 = t17 / t31 / rho[ip];
        let t187 = t88 * t96;
        let t189 = 1.0 / t18 / t187;
        let t190 = t189 * t37;
        let t194 = param_alpha * param_alpha;
        let t195 = param_d * t194;
        let t196 = t22 * t22;
        let t197 = 1.0 / t196;
        let t198 = t195 * t197;
        let t199 = t86 * sigma[ip];
        let t200 = t88 * t88;
        let t201 = t200 * t30;
        let t202 = 1.0 / t201;
        let t208 = 1.0 / t31 / t88;
        let t212 = param_expo * param_expo;
        let t213 = 1.0 / t30;
        let t214 = t212 * t213;
        let t216 = 16.0 / 9.0 * t55 * t214;
        let t219 = 4.0 / 3.0 * t55 * param_expo * t213;
        let t220 = -t85 * t87 * t190 / 12.0 + t198 * t199 * t202 * t37 / 81.0 + 11.0 / 27.0 * t41 * t29 * t208 - t216 - t219;
        let t222 = t106 * t109;
        let t226 = 1.0 / t108 / t68;
        let t227 = t56 * t226;
        let t228 = t132 * t132;
        let t234 = t27 / t18 / t96 * t64;
        let t238 = t208 * param_a * t128;
        let t241 = t84 * t87;
        let t243 = t120 * param_b;
        let t245 = 1.0 / t126 / t125;
        let t246 = t243 * t245;
        let t247 = t189 * param_a * t246;
        let t250 = 7.0 / 27.0 * t57 * t234 + 10.0 / 3.0 * t118 * t238 - 32.0 / 3.0 * t241 * t247 + t216 + t219;
        let t252 = -t110 * t250 - 2.0 * t222 * t132 + t220 * t69 + 2.0 * t227 * t228;
        let t257 = piecewise3(t2, 0.0, t6 * t180 * t71 / 12.0 - t6 * t77 * t134 / 4.0 - 3.0 / 8.0 * t6 * t19 * t252);
        let tv2rho20 = 2.0 * rho[ip] * t257 + 4.0 * t139;
        v2rho2[ip] += tv2rho20;
        let t263 = t27 * t91;
        let t267 = t200 * rho[ip];
        let t268 = 1.0 / t267;
        let t276 = t212 * t102;
        let t279 = 2.0 / 3.0 * t55 * t276 * t154;
        let t280 = t85 * t263 * t146 / 36.0 - t198 * t268 * t86 * t37 / 216.0 - t40 * t150 * t98 / 9.0 + t279;
        let t282 = t158 * t109;
        let t285 = t169 * t132;
        let t294 = param_a * t243 * t245 * sigma[ip];
        let t297 = -t161 * t114 / 18.0 - t164 * t129 + 4.0 * t84 * t263 * t294 - t279;
        let t299 = -t110 * t297 - t282 * t132 - t222 * t169 + 2.0 * t227 * t285 + t280 * t69;
        let t304 = piecewise3(t2, 0.0, -t6 * t77 * t171 / 8.0 - 3.0 / 8.0 * t6 * t19 * t299);
        let tv2rhosigma0 = 2.0 * rho[ip] * t304 + 2.0 * t175;
        v2rhosigma[ip] += tv2rhosigma0;
        let t307 = 1.0 / t200;
        let t312 = t81 * t44;
        let t313 = t83 * t27;
        let t318 = 1.0 / t86;
        let t321 = t55 * t212 * t318 / 4.0;
        let t324 = t55 * param_expo * t318 / 2.0;
        let t325 = t198 * t307 * t37 * sigma[ip] / 576.0 - t312 * t313 * t144 * t37 / 144.0 - t321 + t324;
        let t329 = t169 * t169;
        let t334 = t46 / t47 / sigma[ip];
        let t338 = t117 * t154 * t28;
        let t341 = t84 * t27;
        let t343 = t144 * param_a * t246;
        let t346 = -t334 * t65 / 48.0 + t338 * t166 / 8.0 - 3.0 / 2.0 * t341 * t343 + t321 - t324;
        let t348 = -t110 * t346 - 2.0 * t282 * t169 + 2.0 * t227 * t329 + t325 * t69;
        let t352 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t348);
        let tv2sigma20 = 2.0 * rho[ip] * t352;
        v2sigma2[ip] += tv2sigma20;
        let t355 = t17 * t33;
        let t366 = 1.0 / t18 / t200;
        let t371 = t200 * t96;
        let t372 = 1.0 / t371;
        let t378 = param_d * t194 * param_alpha;
        let t379 = t86 * t86;
        let t380 = t197 * t379;
        let t381 = t378 * t380;
        let t382 = t200 * t142;
        let t384 = 1.0 / t31 / t382;
        let t386 = t150 * t37;
        let t391 = 1.0 / t31 / t142;
        let t395 = t212 * param_expo;
        let t396 = 1.0 / t96;
        let t397 = t395 * t396;
        let t399 = 64.0 / 27.0 * t55 * t397;
        let t400 = t212 * t396;
        let t402 = 16.0 / 3.0 * t55 * t400;
        let t405 = 8.0 / 3.0 * t55 * param_expo * t396;
        let t406 = 341.0 / 486.0 * t85 * t87 * t366 * t37 - 19.0 / 81.0 * t198 * t199 * t372 * t37 + t381 * t384 * t20 * t386 / 729.0 - 154.0 / 81.0 * t41 * t29 * t391 + t399 + t402 + t405;
        let t408 = t220 * t109;
        let t411 = t106 * t226;
        let t416 = t108 * t108;
        let t417 = 1.0 / t416;
        let t418 = t56 * t417;
        let t419 = t228 * t132;
        let t422 = t132 * t250;
        let t428 = t27 / t18 / t88 * t64;
        let t432 = t391 * param_a * t128;
        let t439 = t197 * t199;
        let t441 = t120 * t120;
        let t442 = t441 * param_b;
        let t444 = t125 * t125;
        let t446 = 1.0 / t126 / t444;
        let t447 = param_a * t442 * t446;
        let t450 = -70.0 / 81.0 * t57 * t428 - 476.0 / 27.0 * t118 * t432 + 1184.0 / 9.0 * t241 * t366 * param_a * t246 - 3072.0 * t439 * t372 * t447 - t399 - t402 - t405;
        let t452 = -t110 * t450 - 3.0 * t408 * t132 - 3.0 * t222 * t250 + 6.0 * t227 * t422 + 6.0 * t411 * t228 + t406 * t69 - 6.0 * t418 * t419;
        let t457 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t355 * t71 + t6 * t180 * t134 / 4.0 - 3.0 / 8.0 * t6 * t77 * t252 - 3.0 / 8.0 * t6 * t19 * t452);
        let tv3rho30 = 2.0 * rho[ip] * t457 + 6.0 * t257;
        v3rho3[ip] += tv3rho30;
        let t467 = t27 * t189;
        let t475 = t200 * t88;
        let t478 = t197 / t31 / t475;
        let t479 = t378 * t478;
        let t481 = t199 * t20 * t386;
        let t487 = t395 * t213;
        let t490 = 8.0 / 9.0 * t55 * t487 * t154;
        let t493 = 2.0 / 3.0 * t55 * t214 * t154;
        let t494 = -65.0 / 324.0 * t85 * t467 * t146 + 17.0 / 216.0 * t198 * t202 * t86 * t37 - t479 * t481 / 1944.0 + 11.0 / 27.0 * t40 * t150 * t208 - t490 - t493;
        let t496 = t280 * t109;
        let t499 = t158 * t226;
        let t508 = t169 * t228;
        let t511 = t297 * t132;
        let t514 = t169 * t250;
        let t525 = t197 * t202 * param_a;
        let t526 = t442 * t446;
        let t527 = t526 * t86;
        let t530 = 7.0 / 54.0 * t161 * t234 + 37.0 / 9.0 * t164 * t238 - 124.0 / 3.0 * t84 * t467 * t294 + 1152.0 * t525 * t527 + t490 + t493;
        let t532 = -t110 * t530 - 2.0 * t496 * t132 - t408 * t169 - 2.0 * t222 * t297 + 4.0 * t227 * t511 + 2.0 * t227 * t514 + 2.0 * t499 * t228 - t282 * t250 + 4.0 * t411 * t285 - 6.0 * t418 * t508 + t494 * t69;
        let t537 = piecewise3(t2, 0.0, t6 * t180 * t171 / 12.0 - t6 * t77 * t299 / 4.0 - 3.0 / 8.0 * t6 * t19 * t532);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t537 + 4.0 * t304;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t549 = t197 / t31 / t371;
        let t550 = t378 * t549;
        let t553 = t117 * t86 * t28 * t37;
        let t559 = t395 * t102;
        let t562 = t55 * t559 * t318 / 3.0;
        let t565 = 2.0 / 3.0 * t55 * t276 * t318;
        let t566 = -5.0 / 216.0 * t198 * t268 * t37 * sigma[ip] + t550 * t553 / 5184.0 + t312 * t313 * t92 / 27.0 + t562 - t565;
        let t568 = t325 * t109;
        let t578 = t329 * t132;
        let t581 = t169 * t297;
        let t585 = t346 * t132;
        let t593 = t91 * param_a * t246;
        let t596 = t197 * t268;
        let t598 = t526 * sigma[ip];
        let t601 = t334 * t114 / 36.0 - t338 * t129 / 6.0 + 10.0 * t341 * t593 - 432.0 * t596 * param_a * t598 - t562 + t565;
        let t603 = -t110 * t601 - t568 * t132 - 2.0 * t496 * t169 - t222 * t346 + 4.0 * t227 * t581 + 2.0 * t227 * t585 - 2.0 * t282 * t297 + 4.0 * t499 * t285 + 2.0 * t411 * t329 - 6.0 * t418 * t578 + t566 * t69;
        let t608 = piecewise3(t2, 0.0, -t6 * t77 * t348 / 8.0 - 3.0 / 8.0 * t6 * t19 * t603);
        let tv3rhosigma20 = 2.0 * rho[ip] * t608 + 2.0 * t352;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t613 = t197 / t31 / t201;
        let t614 = t378 * t613;
        let t615 = t28 * t37;
        let t617 = t117 * t615 * sigma[ip];
        let t620 = t197 * t307;
        let t624 = 1.0 / t199;
        let t627 = t55 * t395 * t624 / 8.0;
        let t630 = 3.0 / 4.0 * t55 * t212 * t624;
        let t632 = t55 * param_expo * t624;
        let t633 = -t614 * t617 / 13824.0 + t195 * t620 * t37 / 192.0 - t627 + t630 - t632;
        let t641 = t329 * t169;
        let t644 = t169 * t346;
        let t649 = t46 / t47 / t86;
        let t653 = t117 * t318 * t28;
        let t657 = t84 * t154 * t27;
        let t662 = t649 * t65 / 32.0 - 3.0 / 16.0 * t653 * t166 - 3.0 / 4.0 * t657 * t343 + 162.0 * t620 * t447 + t627 - t630 + t632;
        let t664 = -t110 * t662 - 3.0 * t568 * t169 + 6.0 * t227 * t644 - 3.0 * t282 * t346 + 6.0 * t499 * t329 - 6.0 * t418 * t641 + t633 * t69;
        let t668 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t664);
        let tv3sigma30 = 2.0 * rho[ip] * t668;
        v3sigma3[ip] += tv3sigma30;
    }
}
