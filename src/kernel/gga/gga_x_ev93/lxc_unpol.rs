//! GGA_X_EV93 lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ev93.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ev93_lxc_unpol(
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
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_b1: f64,
    param_b2: f64,
    param_b3: f64,
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
        let t18 = t6 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = M_CBRT6;
        let t21 = param_a1 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t37 = t20 * t20;
        let t38 = param_a2 * t37;
        let t40 = 1.0 / t23 / t22;
        let t41 = t38 * t40;
        let t42 = sigma[ip] * sigma[ip];
        let t43 = t42 * t27;
        let t44 = t30 * t30;
        let t45 = t44 * rho[ip];
        let t47 = 1.0 / t19 / t45;
        let t48 = t43 * t47;
        let t51 = t22 * t22;
        let t52 = 1.0 / t51;
        let t53 = param_a3 * t52;
        let t54 = t42 * sigma[ip];
        let t55 = t44 * t44;
        let t56 = 1.0 / t55;
        let t57 = t54 * t56;
        let t60 = 1.0 + t26 * t34 / 24.0 + t41 * t48 / 288.0 + t53 * t57 / 576.0;
        let t61 = t19 * t60;
        let t62 = param_b1 * t20;
        let t63 = t62 * t25;
        let t66 = param_b2 * t37;
        let t67 = t66 * t40;
        let t70 = param_b3 * t52;
        let t73 = 1.0 + t63 * t34 / 24.0 + t67 * t48 / 288.0 + t70 * t57 / 576.0;
        let t74 = 1.0 / t73;
        let t78 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t61 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        let t79 = 1.0 / t31;
        let t80 = t79 * t60;
        let t84 = t30 * rho[ip];
        let t86 = 1.0 / t31 / t84;
        let t87 = t29 * t86;
        let t90 = t44 * t30;
        let t92 = 1.0 / t19 / t90;
        let t93 = t43 * t92;
        let t96 = t55 * rho[ip];
        let t97 = 1.0 / t96;
        let t98 = t54 * t97;
        let t101 = -t26 * t87 / 9.0 - t41 * t93 / 54.0 - t53 * t98 / 72.0;
        let t102 = t19 * t101;
        let t106 = t73 * t73;
        let t107 = 1.0 / t106;
        let t114 = -t63 * t87 / 9.0 - t67 * t93 / 54.0 - t70 * t98 / 72.0;
        let t115 = t107 * t114;
        let t120 = piecewise3(t2, 0.0, -t18 * t80 * t74 / 8.0 - 3.0 / 8.0 * t18 * t102 * t74 + 3.0 / 8.0 * t18 * t61 * t115);
        let tvrho0 = 2.0 * rho[ip] * t120 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t123 = t25 * t28;
        let t124 = t123 * t33;
        let t127 = sigma[ip] * t27;
        let t128 = t127 * t47;
        let t131 = t42 * t56;
        let t134 = t21 * t124 / 24.0 + t41 * t128 / 144.0 + t53 * t131 / 192.0;
        let t135 = t19 * t134;
        let t144 = t62 * t124 / 24.0 + t67 * t128 / 144.0 + t70 * t131 / 192.0;
        let t145 = t107 * t144;
        let t150 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t135 * t74 + 3.0 / 8.0 * t18 * t61 * t145);
        let tvsigma0 = 2.0 * rho[ip] * t150;
        vsigma[ip] += tvsigma0;
        let t154 = 1.0 / t31 / rho[ip];
        let t155 = t154 * t60;
        let t159 = t79 * t101;
        let t167 = 1.0 / t31 / t44;
        let t168 = t29 * t167;
        let t171 = t44 * t84;
        let t173 = 1.0 / t19 / t171;
        let t174 = t43 * t173;
        let t178 = 1.0 / t55 / t30;
        let t179 = t54 * t178;
        let t182 = 11.0 / 27.0 * t26 * t168 + 19.0 / 162.0 * t41 * t174 + t53 * t179 / 8.0;
        let t183 = t19 * t182;
        let t191 = 1.0 / t106 / t73;
        let t192 = t114 * t114;
        let t193 = t191 * t192;
        let t203 = 11.0 / 27.0 * t63 * t168 + 19.0 / 162.0 * t67 * t174 + t70 * t179 / 8.0;
        let t204 = t107 * t203;
        let t209 = piecewise3(t2, 0.0, t18 * t155 * t74 / 12.0 - t18 * t159 * t74 / 4.0 + t18 * t80 * t115 / 4.0 - 3.0 / 8.0 * t18 * t183 * t74 + 3.0 / 4.0 * t18 * t102 * t115 - 3.0 / 4.0 * t18 * t61 * t193 + 3.0 / 8.0 * t18 * t61 * t204);
        let tv2rho20 = 2.0 * rho[ip] * t209 + 4.0 * t120;
        v2rho2[ip] += tv2rho20;
        let t212 = t79 * t134;
        let t216 = t123 * t86;
        let t219 = t127 * t92;
        let t222 = t42 * t97;
        let t225 = -t21 * t216 / 9.0 - t41 * t219 / 27.0 - t53 * t222 / 24.0;
        let t226 = t19 * t225;
        let t240 = t6 * t17 * t19;
        let t241 = t60 * t191;
        let t242 = t144 * t114;
        let t243 = t241 * t242;
        let t252 = -t62 * t216 / 9.0 - t67 * t219 / 27.0 - t70 * t222 / 24.0;
        let t253 = t107 * t252;
        let t258 = piecewise3(t2, 0.0, -t18 * t212 * t74 / 8.0 - 3.0 / 8.0 * t18 * t226 * t74 + 3.0 / 8.0 * t18 * t135 * t115 + t18 * t80 * t145 / 8.0 + 3.0 / 8.0 * t18 * t102 * t145 - 3.0 / 4.0 * t240 * t243 + 3.0 / 8.0 * t18 * t61 * t253);
        let tv2rhosigma0 = 2.0 * rho[ip] * t258 + 2.0 * t150;
        v2rhosigma[ip] += tv2rhosigma0;
        let t261 = t40 * t27;
        let t262 = t261 * t47;
        let t265 = sigma[ip] * t56;
        let t268 = t38 * t262 / 144.0 + t53 * t265 / 96.0;
        let t269 = t19 * t268;
        let t276 = t144 * t144;
        let t277 = t191 * t276;
        let t285 = t66 * t262 / 144.0 + t70 * t265 / 96.0;
        let t286 = t107 * t285;
        let t291 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t269 * t74 + 3.0 / 4.0 * t18 * t135 * t145 - 3.0 / 4.0 * t18 * t61 * t277 + 3.0 / 8.0 * t18 * t61 * t286);
        let tv2sigma20 = 2.0 * rho[ip] * t291;
        v2sigma2[ip] += tv2sigma20;
        let t295 = 1.0 / t31 / t45;
        let t296 = t29 * t295;
        let t300 = 1.0 / t19 / t55;
        let t301 = t43 * t300;
        let t305 = 1.0 / t55 / t84;
        let t306 = t54 * t305;
        let t309 = -154.0 / 81.0 * t63 * t296 - 209.0 / 243.0 * t67 * t301 - 5.0 / 4.0 * t70 * t306;
        let t310 = t107 * t309;
        let t314 = t33 * t60;
        let t339 = t106 * t106;
        let t340 = 1.0 / t339;
        let t341 = t192 * t114;
        let t342 = t340 * t341;
        let t346 = t114 * t203;
        let t347 = t241 * t346;
        let t350 = t154 * t101;
        let t354 = t79 * t182;
        let t364 = -154.0 / 81.0 * t26 * t296 - 209.0 / 243.0 * t41 * t301 - 5.0 / 4.0 * t53 * t306;
        let t365 = t19 * t364;
        let t369 = 3.0 / 8.0 * t18 * t61 * t310 - 5.0 / 36.0 * t18 * t314 * t74 - t18 * t155 * t115 / 4.0 + 3.0 / 4.0 * t18 * t159 * t115 + 3.0 / 8.0 * t18 * t80 * t204 + 9.0 / 8.0 * t18 * t183 * t115 + 9.0 / 8.0 * t18 * t102 * t204 - 3.0 / 4.0 * t18 * t80 * t193 - 9.0 / 4.0 * t18 * t102 * t193 + 9.0 / 4.0 * t18 * t61 * t342 - 9.0 / 4.0 * t240 * t347 + t18 * t350 * t74 / 4.0 - 3.0 / 8.0 * t18 * t354 * t74 - 3.0 / 8.0 * t18 * t365 * t74;
        let t370 = piecewise3(t2, 0.0, t369);
        let tv3rho30 = 2.0 * rho[ip] * t370 + 6.0 * t209;
        v3rho3[ip] += tv3rho30;
        let t392 = t123 * t167;
        let t395 = t127 * t173;
        let t398 = t42 * t178;
        let t401 = 11.0 / 27.0 * t62 * t392 + 19.0 / 81.0 * t67 * t395 + 3.0 / 8.0 * t70 * t398;
        let t402 = t107 * t401;
        let t406 = t154 * t134;
        let t413 = t79 * t225;
        let t424 = t6 * t17 * t79;
        let t427 = t101 * t191;
        let t428 = t427 * t242;
        let t431 = t252 * t114;
        let t432 = t241 * t431;
        let t435 = t144 * t203;
        let t436 = t241 * t435;
        let t445 = 11.0 / 27.0 * t21 * t392 + 19.0 / 81.0 * t41 * t395 + 3.0 / 8.0 * t53 * t398;
        let t446 = t19 * t445;
        let t450 = t60 * t340;
        let t451 = t144 * t192;
        let t452 = t450 * t451;
        let t455 = 3.0 / 4.0 * t18 * t226 * t115 + 3.0 / 8.0 * t18 * t135 * t204 + t18 * t159 * t145 / 4.0 + t18 * t80 * t253 / 4.0 + 3.0 / 8.0 * t18 * t183 * t145 + 3.0 / 4.0 * t18 * t102 * t253 + 3.0 / 8.0 * t18 * t61 * t402 + t18 * t406 * t74 / 12.0 + t18 * t212 * t115 / 4.0 - t18 * t413 * t74 / 4.0 - 3.0 / 4.0 * t18 * t135 * t193 - t18 * t155 * t145 / 12.0 - t424 * t243 / 2.0 - 3.0 / 2.0 * t240 * t428 - 3.0 / 2.0 * t240 * t432 - 3.0 / 4.0 * t240 * t436 - 3.0 / 8.0 * t18 * t446 * t74 + 9.0 / 4.0 * t240 * t452;
        let t456 = piecewise3(t2, 0.0, t455);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t456 + 4.0 * t258;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t459 = t79 * t268;
        let t463 = t261 * t92;
        let t466 = sigma[ip] * t97;
        let t469 = -t38 * t463 / 27.0 - t53 * t466 / 12.0;
        let t470 = t19 * t469;
        let t483 = t134 * t191;
        let t484 = t483 * t242;
        let t496 = t276 * t114;
        let t497 = t450 * t496;
        let t500 = t144 * t252;
        let t501 = t241 * t500;
        let t510 = t285 * t114;
        let t511 = t241 * t510;
        let t518 = -t66 * t463 / 27.0 - t70 * t466 / 12.0;
        let t519 = t107 * t518;
        let t523 = -t18 * t459 * t74 / 8.0 - 3.0 / 8.0 * t18 * t470 * t74 + 3.0 / 8.0 * t18 * t269 * t115 + t18 * t212 * t145 / 4.0 + 3.0 / 4.0 * t18 * t226 * t145 - 3.0 / 2.0 * t240 * t484 + 3.0 / 4.0 * t18 * t135 * t253 - t18 * t80 * t277 / 4.0 - 3.0 / 4.0 * t18 * t102 * t277 + 9.0 / 4.0 * t240 * t497 - 3.0 / 2.0 * t240 * t501 + t18 * t80 * t286 / 8.0 + 3.0 / 8.0 * t18 * t102 * t286 - 3.0 / 4.0 * t240 * t511 + 3.0 / 8.0 * t18 * t61 * t519;
        let t524 = piecewise3(t2, 0.0, t523);
        let tv3rhosigma20 = 2.0 * rho[ip] * t524 + 2.0 * t291;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t529 = t3 / t4 / t51;
        let t530 = t529 * t17;
        let t532 = 1.0 / t31 / t171;
        let t533 = t532 * param_a3;
        let t546 = t276 * t144;
        let t547 = t340 * t546;
        let t551 = t144 * t285;
        let t552 = t241 * t551;
        let t556 = t107 * param_b3;
        let t561 = piecewise3(t2, 0.0, -t530 * t533 * t74 / 256.0 + 9.0 / 8.0 * t18 * t269 * t145 - 9.0 / 4.0 * t18 * t135 * t277 + 9.0 / 8.0 * t18 * t135 * t286 + 9.0 / 4.0 * t18 * t61 * t547 - 9.0 / 4.0 * t240 * t552 + t530 * t532 * t60 * t556 / 256.0);
        let tv3sigma30 = 2.0 * rho[ip] * t561;
        v3sigma3[ip] += tv3sigma30;
        let t602 = t29 / t31 / t90;
        let t607 = t43 / t19 / t96;
        let t612 = t54 / t55 / t44;
        let t634 = 3.0 / 2.0 * t18 * t354 * t115 + 3.0 / 2.0 * t18 * t159 * t204 + 3.0 / 2.0 * t18 * t365 * t115 + 9.0 / 4.0 * t18 * t183 * t204 + 27.0 / 2.0 * t240 * t450 * t192 * t203 - 3.0 * t240 * t241 * t309 * t114 + 10.0 / 27.0 * t18 * t86 * t60 * t74 - 5.0 / 9.0 * t18 * t33 * t101 * t74 + t18 * t154 * t182 * t74 / 2.0 - t18 * t79 * t364 * t74 / 2.0 - 3.0 / 8.0 * t18 * t19 * (2618.0 / 243.0 * t26 * t602 + 5225.0 / 729.0 * t41 * t607 + 55.0 / 4.0 * t53 * t612) * t74 + 3.0 / 8.0 * t18 * t61 * t107 * (2618.0 / 243.0 * t63 * t602 + 5225.0 / 729.0 * t67 * t607 + 55.0 / 4.0 * t70 * t612) + 5.0 / 9.0 * t18 * t314 * t115;
        let t658 = 1.0 / t339 / t73;
        let t659 = t192 * t192;
        let t664 = t203 * t203;
        let t677 = -t18 * t350 * t115 - t18 * t155 * t204 / 2.0 + t18 * t80 * t310 / 2.0 + t18 * t155 * t193 - 3.0 * t18 * t159 * t193 + 3.0 / 2.0 * t18 * t102 * t310 + 3.0 * t18 * t80 * t342 + 9.0 * t18 * t102 * t342 - 9.0 * t18 * t61 * t658 * t659 - 9.0 / 4.0 * t18 * t61 * t191 * t664 - 9.0 / 2.0 * t18 * t183 * t193 - 3.0 * t424 * t347 - 9.0 * t240 * t427 * t346;
        let t679 = piecewise3(t2, 0.0, t634 + t677);
        let tv4rho40 = 2.0 * rho[ip] * t679 + 8.0 * t370;
        v4rho4[ip] += tv4rho40;
        let t683 = t123 * t295;
        let t686 = t127 * t300;
        let t689 = t42 * t305;
        let t751 = 3.0 / 8.0 * t18 * t61 * t107 * (-154.0 / 81.0 * t62 * t683 - 418.0 / 243.0 * t67 * t686 - 15.0 / 4.0 * t70 * t689) - t18 * t406 * t115 / 4.0 + 9.0 / 8.0 * t18 * t446 * t115 + 9.0 / 8.0 * t18 * t226 * t204 + 3.0 / 8.0 * t18 * t135 * t310 + 3.0 / 4.0 * t18 * t159 * t253 + 3.0 / 8.0 * t18 * t80 * t402 + 3.0 / 8.0 * t18 * t365 * t145 + 9.0 / 8.0 * t18 * t183 * t253 + 9.0 / 8.0 * t18 * t102 * t402 - 3.0 / 4.0 * t18 * t212 * t193 + 3.0 / 4.0 * t18 * t413 * t115 - 9.0 / 4.0 * t18 * t226 * t193 + 3.0 / 8.0 * t18 * t354 * t145 - t18 * t350 * t145 / 4.0 - t18 * t155 * t253 / 4.0 + 3.0 / 8.0 * t18 * t212 * t204 + 9.0 / 4.0 * t18 * t135 * t342 + 5.0 / 36.0 * t18 * t314 * t145;
        let t758 = t101 * t340;
        let t766 = t60 * t658;
        let t829 = -9.0 / 4.0 * t240 * t241 * t401 * t114 + 9.0 / 4.0 * t424 * t452 + 27.0 / 4.0 * t240 * t758 * t451 + 27.0 / 4.0 * t240 * t450 * t252 * t192 - 9.0 * t240 * t766 * t144 * t341 - 3.0 / 4.0 * t240 * t241 * t144 * t309 + t6 * t17 * t154 * t243 / 2.0 - 3.0 / 4.0 * t424 * t436 - 9.0 / 4.0 * t240 * t427 * t435 - 9.0 / 4.0 * t240 * t241 * t252 * t203 - 9.0 / 4.0 * t240 * t483 * t346 - 3.0 / 2.0 * t424 * t428 - 3.0 / 2.0 * t424 * t432 - 9.0 / 4.0 * t240 * t182 * t191 * t242 - 9.0 / 2.0 * t240 * t427 * t431 - 5.0 / 36.0 * t18 * t33 * t134 * t74 - 3.0 / 8.0 * t18 * t19 * (-154.0 / 81.0 * t21 * t683 - 418.0 / 243.0 * t41 * t686 - 15.0 / 4.0 * t53 * t689) * t74 + t18 * t154 * t225 * t74 / 4.0 - 3.0 / 8.0 * t18 * t79 * t445 * t74 + 27.0 / 4.0 * t240 * t450 * t435 * t114;
        let t831 = piecewise3(t2, 0.0, t751 + t829);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t831 + 6.0 * t456;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t835 = t261 * t173;
        let t838 = sigma[ip] * t178;
        let t872 = t252 * t252;
        let t907 = -3.0 / 8.0 * t18 * t19 * (19.0 / 81.0 * t38 * t835 + 3.0 / 4.0 * t53 * t838) * t74 - t18 * t79 * t469 * t74 / 4.0 + t18 * t154 * t268 * t74 / 12.0 + t18 * t155 * t277 / 6.0 - 3.0 / 4.0 * t18 * t269 * t193 - t18 * t406 * t145 / 6.0 - t18 * t155 * t286 / 12.0 + 3.0 / 4.0 * t18 * t135 * t402 - t18 * t159 * t277 / 2.0 - 3.0 / 2.0 * t18 * t61 * t191 * t872 + t18 * t459 * t115 / 4.0 + 3.0 / 4.0 * t18 * t470 * t115 + 3.0 / 8.0 * t18 * t269 * t204 + t18 * t413 * t145 / 2.0 + t18 * t212 * t253 / 2.0 + 3.0 / 4.0 * t18 * t446 * t145 + 3.0 / 2.0 * t18 * t226 * t253 - 3.0 / 4.0 * t18 * t183 * t277 + t18 * t159 * t286 / 4.0 + t18 * t80 * t519 / 4.0;
        let t965 = t134 * t340;
        let t981 = -3.0 * t240 * t225 * t191 * t242 - 3.0 * t240 * t483 * t431 - 3.0 / 2.0 * t240 * t483 * t435 + 3.0 / 2.0 * t424 * t497 - t424 * t501 + 9.0 / 2.0 * t240 * t758 * t496 - 3.0 * t240 * t427 * t500 + 9.0 / 2.0 * t240 * t965 * t451 - 9.0 * t240 * t766 * t276 * t192 + 9.0 / 4.0 * t240 * t450 * t285 * t192 + 9.0 * t240 * t450 * t242 * t252;
        let t984 = piecewise3(t2, 0.0, t907 + 3.0 / 8.0 * t18 * t183 * t286 + 3.0 / 4.0 * t18 * t102 * t519 + 3.0 / 8.0 * t18 * t61 * t107 * (19.0 / 81.0 * t66 * t835 + 3.0 / 4.0 * t70 * t838) + 9.0 / 4.0 * t240 * t450 * t276 * t203 - 3.0 / 2.0 * t240 * t241 * t144 * t401 - t424 * t511 / 2.0 - 3.0 / 2.0 * t240 * t427 * t510 - 3.0 / 2.0 * t240 * t241 * t518 * t114 - 3.0 / 4.0 * t240 * t241 * t285 * t203 - t424 * t484 + t981);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t984 + 4.0 * t524;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t994 = 1.0 / t31 / t55;
        let t1031 = 3.0 / 8.0 * t18 * t212 * t286 + 3.0 / 4.0 * t18 * t80 * t547 - 23.0 / 768.0 * t530 * t994 * t60 * t556 + 3.0 / 8.0 * t18 * t459 * t145 - 3.0 / 4.0 * t18 * t212 * t277 + 23.0 / 768.0 * t530 * t994 * param_a3 * t74 + 9.0 / 8.0 * t18 * t226 * t286 + 9.0 / 8.0 * t18 * t135 * t519 + 9.0 / 4.0 * t18 * t102 * t547 + t530 * t532 * t101 * t556 / 256.0 + t530 * t533 * t115 / 256.0 + 9.0 / 8.0 * t18 * t470 * t145 + 9.0 / 8.0 * t18 * t269 * t253;
        let t1047 = t529 * t17 * t532;
        let t1079 = -9.0 / 4.0 * t18 * t226 * t277 - 9.0 / 4.0 * t240 * t427 * t551 - 9.0 / 4.0 * t240 * t241 * t252 * t285 - 9.0 / 4.0 * t240 * t241 * t144 * t518 - t1047 * t241 * param_b3 * t114 / 128.0 - 9.0 / 4.0 * t240 * t268 * t191 * t242 + 27.0 / 4.0 * t240 * t965 * t496 - 9.0 / 2.0 * t240 * t483 * t500 - 9.0 / 4.0 * t240 * t483 * t510 - 9.0 * t240 * t766 * t546 * t114 + 27.0 / 4.0 * t240 * t450 * t276 * t252 - 3.0 / 4.0 * t424 * t552 + 27.0 / 4.0 * t240 * t450 * t551 * t114;
        let t1081 = piecewise3(t2, 0.0, t1031 + t1079);
        let tv4rhosigma30 = 2.0 * rho[ip] * t1081 + 2.0 * t561;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t1103 = t276 * t276;
        let t1112 = t285 * t285;
        let t1122 = piecewise3(t2, 0.0, t530 * t533 * t145 / 64.0 - 9.0 / 2.0 * t18 * t269 * t277 + 9.0 / 4.0 * t18 * t269 * t286 + 9.0 * t18 * t135 * t547 - 9.0 * t240 * t483 * t551 + t530 * t532 * t134 * t556 / 64.0 - 9.0 * t18 * t61 * t658 * t1103 + 27.0 / 2.0 * t240 * t450 * t276 * t285 - 9.0 / 4.0 * t18 * t61 * t191 * t1112 - t1047 * t241 * t144 * param_b3 / 32.0);
        let tv4sigma40 = 2.0 * rho[ip] * t1122;
        v4sigma4[ip] += tv4sigma40;
    }
}
