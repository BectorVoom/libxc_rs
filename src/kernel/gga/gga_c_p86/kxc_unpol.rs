//! GGA_C_P86 kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_p86.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use crate::math::piecewise::{piecewise3};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_p86_kxc_unpol(
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
    param_aa: f64,
    param_bb: f64,
    param_ftilde: f64,
    param_malpha: f64,
    param_mbeta: f64,
    param_mdelta: f64,
    param_mgamma: f64,
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
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = 1.0 <= t11;
        let t13 = f64::sqrt(t10);
        let t16 = 1.0 + 0.52645e0 * t13 + 0.8335e-1 * t10;
        let t19 = f64::ln(t11);
        let t22 = t4 * t9 * t19;
        let t26 = piecewise3(t12, -0.1423e0 / t16, 0.311e-1 * t19 - 0.48e-1 + 0.5e-3 * t22 - 0.29e-2 * t10);
        let t29 = 1.0 + 0.69905e0 * t13 + 0.65275e-1 * t10;
        let t36 = piecewise3(t12, -0.843e-1 / t29, 0.1555e-1 * t19 - 0.269e-1 + 0.175e-3 * t22 - 0.12e-2 * t10);
        let t38 = 1.0 <= zeta_threshold;
        let t39 = pow_1_3(zeta_threshold);
        let t41 = piecewise3(t38, t39 * zeta_threshold, 1.0);
        let t43 = 2.0 * t41 - 2.0;
        let t45 = M_CBRT2;
        let t48 = 1.0 / (2.0 * t45 - 2.0);
        let t49 = (t36 - t26) * t43 * t48;
        let t50 = rho[ip] * rho[ip];
        let t52 = 1.0 / t7 / t50;
        let t53 = sigma[ip] * t52;
        let t54 = param_aa + param_bb;
        let t55 = param_ftilde * t54;
        let t56 = param_malpha * t1;
        let t57 = t3 * t6;
        let t58 = t57 * t8;
        let t61 = t1 * t1;
        let t62 = param_mbeta * t61;
        let t63 = t3 * t3;
        let t64 = t63 * t5;
        let t65 = t7 * t7;
        let t66 = 1.0 / t65;
        let t67 = t64 * t66;
        let t70 = param_bb + t56 * t58 / 4.0 + t62 * t67 / 4.0;
        let t71 = param_mgamma * t1;
        let t74 = param_mdelta * t61;
        let t77 = 1.0 / rho[ip];
        let t80 = 1.0 + t71 * t58 / 4.0 + t74 * t67 / 4.0 + 0.23873241463784300365e4 * param_mbeta * t77;
        let t81 = 1.0 / t80;
        let t83 = t70 * t81 + param_aa;
        let t84 = 1.0 / t83;
        let t85 = f64::sqrt(sigma[ip]);
        let t86 = t84 * t85;
        let t87 = f64::powf(rho[ip], 1.0 / 6.0);
        let t89 = 1.0 / t87 / rho[ip];
        let t92 = f64::exp(-t55 * t86 * t89);
        let t94 = t39 * t39;
        let t96 = piecewise3(t38, t94 * zeta_threshold, 1.0);
        let t97 = f64::sqrt(t96);
        let t98 = 1.0 / t97;
        let t99 = t92 * t83 * t98;
        let t100 = t53 * t99;
        let tzk0 = t26 + t49 + t100;
        zk[ip] += tzk0;
        let t101 = t16 * t16;
        let t102 = 1.0 / t101;
        let t104 = 1.0 / t13 * t1;
        let t106 = 1.0 / t7 / rho[ip];
        let t107 = t57 * t106;
        let t108 = t104 * t107;
        let t110 = t6 * t106;
        let t111 = t4 * t110;
        let t113 = -0.87741666666666666667e-1 * t108 - 0.27783333333333333333e-1 * t111;
        let t118 = t4 * t110 * t19;
        let t122 = piecewise3(t12, 0.1423e0 * t102 * t113, -0.10366666666666666667e-1 * t77 - 0.16666666666666666667e-3 * t118 + 0.8e-3 * t111);
        let t123 = t29 * t29;
        let t124 = 1.0 / t123;
        let t127 = -0.11650833333333333333e0 * t108 - 0.21758333333333333333e-1 * t111;
        let t134 = piecewise3(t12, 0.843e-1 * t124 * t127, -0.51833333333333333333e-2 * t77 - 0.58333333333333333333e-4 * t118 + 0.34166666666666666667e-3 * t111);
        let t137 = (t134 - t122) * t43 * t48;
        let t138 = t50 * rho[ip];
        let t140 = 1.0 / t7 / t138;
        let t141 = sigma[ip] * t140;
        let t142 = t141 * t99;
        let t144 = t83 * t83;
        let t145 = 1.0 / t144;
        let t146 = t55 * t145;
        let t147 = t85 * t89;
        let t152 = t64 / t65 / rho[ip];
        let t155 = -t56 * t107 / 12.0 - t62 * t152 / 6.0;
        let t157 = t80 * t80;
        let t158 = 1.0 / t157;
        let t159 = t70 * t158;
        let t164 = 1.0 / t50;
        let t167 = -t71 * t107 / 12.0 - t74 * t152 / 6.0 - 0.23873241463784300365e4 * param_mbeta * t164;
        let t169 = t155 * t81 - t159 * t167;
        let t173 = 1.0 / t87 / t50;
        let t177 = t146 * t147 * t169 + 7.0 / 6.0 * t55 * t86 * t173;
        let t178 = t53 * t177;
        let t179 = t178 * t99;
        let t181 = t92 * t169 * t98;
        let t182 = t53 * t181;
        let tvrho0 = t26 + t49 + t100 + rho[ip] * (t122 + t137 - 7.0 / 3.0 * t142 + t179 + t182);
        vrho[ip] += tvrho0;
        let t185 = t52 * t92;
        let t186 = t83 * t98;
        let t187 = t185 * t186;
        let t188 = f64::sqrt(rho[ip]);
        let t190 = 1.0 / t188 / t138;
        let t191 = t85 * t190;
        let t192 = t191 * param_ftilde;
        let t194 = t54 * t92 * t98;
        let t196 = t192 * t194 / 2.0;
        let tvsigma0 = rho[ip] * (t187 - t196);
        vsigma[ip] += tvsigma0;
        let t204 = 1.0 / t101 / t16;
        let t205 = t113 * t113;
        let t210 = 1.0 / t13 / t10 * t61;
        let t213 = t64 / t65 / t50;
        let t214 = t210 * t213;
        let t216 = t57 * t52;
        let t217 = t104 * t216;
        let t219 = t6 * t52;
        let t220 = t4 * t219;
        let t222 = -0.58494444444444444445e-1 * t214 + 0.11698888888888888889e0 * t217 + 0.37044444444444444444e-1 * t220;
        let t228 = t4 * t219 * t19;
        let t232 = piecewise3(t12, -0.2846e0 * t204 * t205 + 0.1423e0 * t102 * t222, 0.10366666666666666667e-1 * t164 + 0.22222222222222222223e-3 * t228 - 0.10111111111111111111e-2 * t220);
        let t234 = 1.0 / t123 / t29;
        let t235 = t127 * t127;
        let t241 = -0.7767222222222222222e-1 * t214 + 0.15534444444444444444e0 * t217 + 0.29011111111111111111e-1 * t220;
        let t249 = piecewise3(t12, -0.1686e0 * t234 * t235 + 0.843e-1 * t124 * t241, 0.51833333333333333333e-2 * t164 + 0.77777777777777777777e-4 * t228 - 0.43611111111111111112e-3 * t220);
        let t252 = (t249 - t232) * t43 * t48;
        let t253 = t50 * t50;
        let t255 = 1.0 / t7 / t253;
        let t256 = sigma[ip] * t255;
        let t257 = t256 * t99;
        let t259 = t141 * t177;
        let t260 = t259 * t99;
        let t262 = t141 * t181;
        let t265 = 1.0 / t144 / t83;
        let t266 = t55 * t265;
        let t267 = t169 * t169;
        let t271 = t85 * t173;
        let t279 = t56 * t216 / 9.0 + 5.0 / 18.0 * t62 * t213;
        let t281 = t155 * t158;
        let t285 = 1.0 / t157 / t80;
        let t286 = t70 * t285;
        let t287 = t167 * t167;
        let t294 = 1.0 / t138;
        let t297 = t71 * t216 / 9.0 + 5.0 / 18.0 * t74 * t213 + 0.4774648292756860073e4 * param_mbeta * t294;
        let t299 = -t159 * t297 - 2.0 * t281 * t167 + t279 * t81 + 2.0 * t286 * t287;
        let t303 = 1.0 / t87 / t138;
        let t307 = -2.0 * t266 * t147 * t267 - 7.0 / 3.0 * t146 * t271 * t169 + t146 * t147 * t299 - 91.0 / 36.0 * t55 * t86 * t303;
        let t308 = t53 * t307;
        let t309 = t308 * t99;
        let t310 = t177 * t177;
        let t311 = t53 * t310;
        let t312 = t311 * t99;
        let t313 = t178 * t181;
        let t316 = t92 * t299 * t98;
        let t317 = t53 * t316;
        let tv2rho20 = 2.0 * t122 + 2.0 * t137 - 14.0 / 3.0 * t142 + 2.0 * t179 + 2.0 * t182 + rho[ip] * (t232 + t252 + 70.0 / 9.0 * t257 - 14.0 / 3.0 * t260 - 14.0 / 3.0 * t262 + t309 + t312 + 2.0 * t313 + t317);
        v2rho2[ip] += tv2rho20;
        let t320 = t140 * t92;
        let t321 = t320 * t186;
        let t323 = t52 * t177;
        let t324 = t323 * t99;
        let t325 = t169 * t98;
        let t326 = t185 * t325;
        let t328 = 1.0 / t188 / t253;
        let t330 = t85 * t328 * param_ftilde;
        let t331 = t330 * t194;
        let t334 = t92 * t98;
        let t335 = t54 * t177 * t334;
        let t336 = t192 * t335;
        let tv2rhosigma0 = t187 - t196 + rho[ip] * (-7.0 / 3.0 * t321 + t324 + t326 + 7.0 / 4.0 * t331 - t336 / 2.0);
        v2rhosigma[ip] += tv2rhosigma0;
        let t341 = t190 * param_ftilde * t54;
        let t342 = 1.0 / t85;
        let t343 = t342 * t92;
        let t344 = t343 * t98;
        let t346 = 3.0 / 4.0 * t341 * t344;
        let t348 = 1.0 / t65 / t253;
        let t349 = param_ftilde * param_ftilde;
        let t350 = t348 * t349;
        let t351 = t54 * t54;
        let t352 = t350 * t351;
        let t354 = t84 * t92 * t98;
        let t356 = t352 * t354 / 4.0;
        let tv2sigma20 = rho[ip] * (-t346 + t356);
        v2sigma2[ip] += tv2sigma20;
        let t367 = t123 * t123;
        let t368 = 1.0 / t367;
        let t372 = t234 * t127;
        let t381 = 1.0 / t13 / t61 / t63 / t5 / t66 * t2 / 4.0;
        let t382 = 1.0 / t253;
        let t383 = t381 * t382;
        let t387 = t64 / t65 / t138;
        let t388 = t210 * t387;
        let t390 = t57 * t140;
        let t391 = t104 * t390;
        let t393 = t6 * t140;
        let t394 = t4 * t393;
        let t396 = -0.46603333333333333332e0 * t383 + 0.31068888888888888888e0 * t388 - 0.36247037037037037036e0 * t391 - 0.67692592592592592592e-1 * t394;
        let t402 = t4 * t393 * t19;
        let t406 = piecewise3(t12, 0.5058e0 * t368 * t235 * t127 - 0.5058e0 * t372 * t241 + 0.843e-1 * t124 * t396, -0.10366666666666666667e-1 * t294 - 0.18148148148148148148e-3 * t402 + 0.99166666666666666667e-3 * t394);
        let t407 = t101 * t101;
        let t408 = 1.0 / t407;
        let t412 = t204 * t113;
        let t419 = -0.35096666666666666667e0 * t383 + 0.23397777777777777778e0 * t388 - 0.27297407407407407408e0 * t391 - 0.86437037037037037036e-1 * t394;
        let t427 = piecewise3(t12, 0.8538e0 * t408 * t205 * t113 - 0.8538e0 * t412 * t222 + 0.1423e0 * t102 * t419, -0.20733333333333333334e-1 * t294 - 0.51851851851851851854e-3 * t402 + 0.22851851851851851851e-2 * t394);
        let t430 = (t406 - t427) * t43 * t48;
        let t431 = t256 * t181;
        let t433 = t141 * t316;
        let t439 = -7.0 / 27.0 * t56 * t390 - 20.0 / 27.0 * t62 * t387;
        let t441 = t279 * t158;
        let t444 = t155 * t285;
        let t449 = t157 * t157;
        let t450 = 1.0 / t449;
        let t451 = t70 * t450;
        let t452 = t287 * t167;
        let t455 = t167 * t297;
        let t464 = -7.0 / 27.0 * t71 * t390 - 20.0 / 27.0 * t74 * t387 - 0.14323944878270580219e5 * param_mbeta * t382;
        let t466 = -t159 * t464 - 3.0 * t441 * t167 - 3.0 * t281 * t297 + 6.0 * t286 * t455 + 6.0 * t444 * t287 + t439 * t81 - 6.0 * t451 * t452;
        let t468 = t92 * t466 * t98;
        let t469 = t53 * t468;
        let t470 = t253 * rho[ip];
        let t472 = 1.0 / t7 / t470;
        let t473 = sigma[ip] * t472;
        let t474 = t473 * t99;
        let t476 = t256 * t177;
        let t477 = t476 * t99;
        let t479 = t141 * t307;
        let t480 = t479 * t99;
        let t482 = t259 * t181;
        let t484 = t144 * t144;
        let t486 = t55 / t484;
        let t487 = t267 * t169;
        let t494 = t169 * t299;
        let t498 = t85 * t303;
        let t508 = 1.0 / t87 / t253;
        let t512 = 6.0 * t486 * t147 * t487 + 7.0 * t266 * t271 * t267 - 6.0 * t266 * t147 * t494 + 91.0 / 12.0 * t146 * t498 * t169 - 7.0 / 2.0 * t146 * t271 * t299 + t146 * t147 * t466 + 1729.0 / 216.0 * t55 * t86 * t508;
        let t513 = t53 * t512;
        let t514 = t513 * t99;
        let t515 = t308 * t181;
        let t517 = t311 * t181;
        let t519 = t178 * t316;
        let t521 = t141 * t310;
        let t522 = t521 * t99;
        let t524 = t177 * t92;
        let t525 = t524 * t186;
        let t526 = t308 * t525;
        let t528 = t310 * t177;
        let t529 = t53 * t528;
        let t530 = t529 * t99;
        let t531 = t430 + 70.0 / 3.0 * t431 + t427 - 7.0 * t433 + t469 - 910.0 / 27.0 * t474 + 70.0 / 3.0 * t477 - 7.0 * t480 - 14.0 * t482 + t514 + 3.0 * t515 + 3.0 * t517 + 3.0 * t519 - 7.0 * t522 + 3.0 * t526 + t530;
        let tv3rho30 = 3.0 * t232 + 3.0 * t252 + 70.0 / 3.0 * t257 - 14.0 * t260 - 14.0 * t262 + 3.0 * t309 + 3.0 * t312 + 6.0 * t313 + 3.0 * t317 + rho[ip] * t531;
        v3rho3[ip] += tv3rho30;
        let t537 = t255 * t92;
        let t538 = t537 * t186;
        let t540 = t140 * t177;
        let t541 = t540 * t99;
        let t543 = t320 * t325;
        let t545 = t52 * t307;
        let t546 = t545 * t99;
        let t547 = t52 * t310;
        let t548 = t547 * t99;
        let t549 = t323 * t181;
        let t551 = t299 * t98;
        let t552 = t185 * t551;
        let t554 = 1.0 / t188 / t470;
        let t556 = t85 * t554 * param_ftilde;
        let t557 = t556 * t194;
        let t559 = t330 * t335;
        let t562 = t54 * t307 * t334;
        let t563 = t192 * t562;
        let t566 = t54 * t310 * t334;
        let t567 = t192 * t566;
        let t569 = 70.0 / 9.0 * t538 - 14.0 / 3.0 * t541 - 14.0 / 3.0 * t543 + t546 + t548 + 2.0 * t549 + t552 - 63.0 / 8.0 * t557 + 7.0 / 2.0 * t559 - t563 / 2.0 - t567 / 2.0;
        let tv3rho2sigma0 = -14.0 / 3.0 * t321 + 2.0 * t324 + 2.0 * t326 + 7.0 / 2.0 * t331 - t336 + rho[ip] * t569;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t572 = t328 * param_ftilde * t54;
        let t573 = t572 * t344;
        let t576 = t342 * t177 * t334;
        let t577 = t341 * t576;
        let t582 = 1.0 / t65 / t470 * t349 * t351;
        let t583 = t582 * t354;
        let t585 = t145 * t92;
        let t586 = t585 * t325;
        let t587 = t352 * t586;
        let t590 = t84 * t177 * t334;
        let t591 = t352 * t590;
        let tv3rhosigma20 = -t346 + t356 + rho[ip] * (21.0 / 8.0 * t573 - 3.0 / 4.0 * t577 - 7.0 / 6.0 * t583 - t587 / 4.0 + t591 / 4.0);
        v3rhosigma2[ip] += tv3rhosigma20;
        let t596 = 1.0 / t85 / sigma[ip];
        let t598 = t596 * t92 * t98;
        let t600 = 3.0 / 8.0 * t341 * t598;
        let t601 = 1.0 / sigma[ip];
        let t603 = t601 * t84 * t334;
        let t605 = 3.0 / 8.0 * t352 * t603;
        let t606 = t87 * t87;
        let t607 = t606 * t606;
        let t608 = t607 * t87;
        let t611 = t349 * param_ftilde;
        let t612 = 1.0 / t608 / t470 * t611;
        let t613 = t351 * t54;
        let t614 = t612 * t613;
        let t616 = t145 * t342 * t334;
        let t618 = t614 * t616 / 8.0;
        let tv3sigma30 = rho[ip] * (t600 + t605 - t618);
        v3sigma3[ip] += tv3sigma30;
    }
}
