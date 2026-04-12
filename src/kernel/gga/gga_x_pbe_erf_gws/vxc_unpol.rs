//! GGA_X_PBE_ERF_GWS vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbe_erf_gws.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use crate::math::erf::{erf_approx};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pbe_erf_gws_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_ax: f64,
    param_b_PBE: f64,
    param_hyb_omega_0: f64,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = param_hyb_omega_0 * param_hyb_omega_0;
        let t4 = param_ax * t3;
        let t5 = M_CBRT3;
        let t7 = M_CBRTPI;
        let t8 = t7 * M_PI;
        let t9 = 1.0 / t8;
        let t10 = 2.0 <= zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t12 = M_CBRT2;
        let t13 = piecewise3(t10, t11, t12);
        let t14 = t13 * t13;
        let t15 = 1.0 / t14;
        let t16 = t9 * t15;
        let t17 = t12 * t12;
        let t18 = pow_1_3(rho[ip]);
        let t19 = t18 * t18;
        let t20 = 1.0 / t19;
        let t25 = f64::exp(-t4 * t5 * t16 * t17 * t20 / 12.0);
        let t26 = param_b_PBE * t25;
        let t27 = t26 * sigma[ip];
        let t28 = param_kappa + 1.0;
        let t29 = t5 * t28;
        let t30 = t5 * t5;
        let t31 = t12 * t30;
        let t32 = t7 * t7;
        let t34 = t31 / t32;
        let t35 = 1.0 / t18;
        let t37 = 1.0 / t13;
        let t40 = t34 * param_hyb_omega_0 * t35 * t37 / 6.0;
        let t41 = t40 < 0.5e-1;
        let t42 = t14 * t14;
        let t43 = M_PI * M_PI;
        let t44 = t32 * t43;
        let t45 = t42 * t44;
        let t46 = t18 * rho[ip];
        let t47 = t45 * t46;
        let t49 = t14 * t8;
        let t50 = t49 * t17;
        let t52 = t5 * t19 * t3;
        let t53 = t50 * t52;
        let t55 = 7.0 * t47 - 6.0 * t53;
        let t56 = t14 * t13;
        let t57 = 1.0 / param_hyb_omega_0;
        let t64 = erf_approx(t57 * t5 * t32 * t13 * t17 * t18 / 2.0);
        let t66 = f64::sqrt(M_PI);
        let t67 = t66 * t43;
        let t68 = t56 * t64 * t67;
        let t69 = rho[ip] * param_hyb_omega_0;
        let t75 = t3 * t3;
        let t76 = t75 * t30;
        let t78 = 12.0 * t76 * t12;
        let t79 = -36.0 * t68 * t31 * t69 + 81.0 * t47 + 54.0 * t53 - t78;
        let t80 = 1.0 / t79;
        let t82 = 10000000000.0 < t40;
        let t83 = t43 * t43;
        let t84 = rho[ip] * rho[ip];
        let t86 = t42 * t14;
        let t90 = t44 * t17 * t5;
        let t96 = t8 * t12 * t30;
        let t102 = t75 * t3;
        let t103 = 1.0 / t102;
        let t107 = 1.0 / t3;
        let t108 = t107 * t30;
        let t113 = t108 * t8 * t14 * t12 * t19 / 2.0;
        let t114 = f64::exp(t113);
        let t115 = t114 * t8;
        let t118 = t5 * t3;
        let t119 = t14 * t17 * t118;
        let t123 = t114 * t12;
        let t127 = (7.0 * t115 * t19 * t119 - 12.0 * t123 * t76 + 6.0 * t47 + 11.0 * t53 + t78) * t8;
        let t128 = t19 * t14;
        let t129 = t127 * t128;
        let t130 = t17 * t30;
        let t131 = t42 * t114;
        let t132 = t44 * t12;
        let t136 = t56 * t114;
        let t143 = t14 * t114 * t8;
        let t148 = t114 * t17;
        let t153 = 12.0 * t136 * t64 * t67 * t130 * t69 - 27.0 * t131 * t132 * t46 - 4.0 * t130 * t75 - 36.0 * t143 * t52 + 4.0 * t148 * t76 + 24.0 * t49 * t52;
        let t156 = t130 * t107 / t153;
        let t159 = piecewise5(t41, t55 * t80, t82, (5600.0 * t96 * t19 * t75 * t14 - 140.0 * t90 * t46 * t3 * t42 - 1863.0 * t83 * t84 * t86) * t103 / 201600.0, -t129 * t156 / 18.0);
        let t163 = t19 * t84;
        let t165 = param_kappa * t163 * t8;
        let t166 = 27.0 / 28.0 * t27 * t29 * t159 + t165;
        let t167 = t166 * t46;
        let t170 = piecewise3(t10, t11 * zeta_threshold, 2.0 * t12);
        let t171 = t170 * t17;
        let t172 = t167 * t171;
        let t173 = 0.135e1 <= t40;
        let t174 = 0.135e1 < t40;
        let t175 = piecewise3(t174, t40, 0.135e1);
        let t176 = t175 * t175;
        let t177 = t176 * t176;
        let t178 = t177 * t176;
        let t179 = t177 * t177;
        let t182 = t179 * t177;
        let t184 = t179 * t176;
        let t190 = 0.240888840192e14 * t179 * t178 + 19448.0 * t176 - 807840.0 * t177 + 30551040.0 * t178 - 0.104552448e10 * t179 - 0.90333315072e12 * t182 + 0.3226189824e11 * t184 - 429.0;
        let t191 = t179 * t179;
        let t192 = 1.0 / t191;
        let t195 = piecewise3(t174, 0.135e1, t40);
        let t196 = t195 * t195;
        let t197 = t196 * t196;
        let t200 = 32.0 * t197 - 16.0 * t196;
        let t203 = f64::exp(-1.0 / t196 / 4.0);
        let t207 = 1.0 / t195;
        let t209 = erf_approx(t207 / 2.0);
        let t210 = t66 * t209;
        let t215 = piecewise3(t173, t190 * t192 / 0.8671998246912e15, t200 * t203 / 3.0 - 32.0 / 3.0 * t197 - 8.0 / 3.0 * t210 * t195 + 8.0 * t196 + 1.0);
        let t216 = 1.0 / t7;
        let t217 = t215 * t216;
        let t218 = param_b_PBE * t159;
        let t220 = t25 * sigma[ip] * t5;
        let t224 = 864.0 * t218 * t220 + 896.0 * t165;
        let t225 = 1.0 / t224;
        let t226 = t5 * t225;
        let t227 = t217 * t226;
        let t230 = piecewise3(t2, 0.0, -84.0 * t172 * t227);
        let t231 = 1.0 / rho[ip];
        let tzk0 = 2.0 * t230 * t231;
        zk[ip] += tzk0;
        let t234 = param_b_PBE * param_ax * t3;
        let t237 = t234 * t30 * t9 * t15;
        let t238 = t19 * rho[ip];
        let t239 = 1.0 / t238;
        let t241 = t17 * t239 * t25;
        let t242 = sigma[ip] * t28;
        let t243 = t242 * t159;
        let t247 = t45 * t18;
        let t249 = t5 * t35;
        let t250 = t249 * t3;
        let t251 = t50 * t250;
        let t253 = 28.0 / 3.0 * t247 - 4.0 * t251;
        let t255 = t79 * t79;
        let t256 = 1.0 / t255;
        let t257 = t55 * t256;
        let t258 = f64::exp(-t113);
        let t267 = -72.0 * t45 * t258 * t18 - 36.0 * t68 * t31 * param_hyb_omega_0 + 108.0 * t247 + 36.0 * t251;
        let t293 = (8.0 * t247 + 14.0 * t45 * t18 * t114 - 22.0 / 3.0 * t115 * t35 * t119 + 22.0 / 3.0 * t251) * t8;
        let t294 = t293 * t128;
        let t297 = t35 * t14;
        let t298 = t127 * t297;
        let t301 = t153 * t153;
        let t303 = t107 / t301;
        let t304 = t86 * t107;
        let t305 = t304 * t30;
        let t306 = t83 * t17;
        let t316 = t42 * t13 * t57 * t5;
        let t317 = t43 * M_PI;
        let t318 = f64::powf(M_PI, 1.0 / 6.0);
        let t319 = t318 * t318;
        let t320 = t319 * t319;
        let t321 = t320 * t318;
        let t322 = t321 * t317;
        let t324 = t114 * t64;
        let t328 = t131 * t44;
        let t329 = t258 * t12;
        let t343 = 12.0 * t136 * t64 * t67 * t17 * t30 * param_hyb_omega_0 - 9.0 * t305 * t306 * rho[ip] * t114 + 24.0 * t316 * t322 * t19 * t324 - 72.0 * t131 * t132 * t18 + 24.0 * t328 * t329 * t18 - 16.0 * t143 * t250 + 16.0 * t49 * t250;
        let t345 = t130 * t303 * t343;
        let t349 = piecewise5(t41, t253 * t80 - t257 * t267, t82, (-3726.0 * t83 * rho[ip] * t86 - 560.0 / 3.0 * t90 * t18 * t3 * t42 + 11200.0 / 3.0 * t96 * t35 * t75 * t14) * t103 / 201600.0, -t294 * t156 / 18.0 - t298 * t156 / 27.0 + t129 * t345 / 18.0);
        let t354 = param_kappa * t238 * t8;
        let t356 = 3.0 / 56.0 * t237 * t241 * t243 + 27.0 / 28.0 * t27 * t29 * t349 + 8.0 / 3.0 * t354;
        let t357 = t356 * t46;
        let t358 = t357 * t171;
        let t361 = t166 * t18;
        let t362 = t361 * t171;
        let t365 = t177 * t175;
        let t366 = t179 * t365;
        let t367 = 1.0 / t46;
        let t371 = t34 * param_hyb_omega_0 * t367 * t37 / 18.0;
        let t372 = piecewise3(t174, -t371, 0.0);
        let t375 = t176 * t175;
        let t376 = t179 * t375;
        let t379 = t179 * t175;
        let t382 = t177 * t375;
        let t391 = 38896.0 * t175 * t372 + 0.18330624e9 * t365 * t372 + 0.3372443762688e15 * t366 * t372 - 3231360.0 * t375 * t372 - 0.1083999780864e14 * t376 * t372 + 0.3226189824e12 * t379 * t372 - 0.836419584e10 * t382 * t372;
        let t395 = 1.0 / t191 / t175;
        let t396 = t190 * t395;
        let t400 = t196 * t195;
        let t401 = piecewise3(t174, 0.0, -t371);
        let t402 = t400 * t401;
        let t404 = t195 * t401;
        let t406 = 128.0 * t402 - 32.0 * t404;
        let t409 = 1.0 / t400;
        let t410 = t200 * t409;
        let t411 = t401 * t203;
        let t415 = t203 * t207;
        let t422 = piecewise3(t173, t391 * t192 / 0.8671998246912e15 - t396 * t372 / 0.541999890432e14, t406 * t203 / 3.0 + t410 * t411 / 6.0 - 128.0 / 3.0 * t402 + 8.0 / 3.0 * t415 * t401 - 8.0 / 3.0 * t210 * t401 + 16.0 * t404);
        let t423 = t422 * t216;
        let t424 = t423 * t226;
        let t427 = t224 * t224;
        let t428 = 1.0 / t427;
        let t429 = t5 * t428;
        let t430 = param_b_PBE * t349;
        let t433 = t4 * t30;
        let t434 = t218 * t433;
        let t435 = t16 * t17;
        let t438 = t435 * t239 * t25 * sigma[ip];
        let t442 = 864.0 * t430 * t220 + 48.0 * t434 * t438 + 7168.0 / 3.0 * t354;
        let t443 = t429 * t442;
        let t444 = t217 * t443;
        let t448 = piecewise3(t2, 0.0, -84.0 * t172 * t424 + 84.0 * t172 * t444 - 84.0 * t358 * t227 - 112.0 * t362 * t227);
        let tvrho0 = 2.0 * t448;
        vrho[ip] += tvrho0;
        let t449 = t30 * t28;
        let t451 = t26 * t449 * t159;
        let t453 = t46 * t170 * t17;
        let t454 = t217 * t225;
        let t455 = t453 * t454;
        let t458 = t171 * t215;
        let t459 = t167 * t458;
        let t461 = t216 * t30 * t428;
        let t463 = t461 * t218 * t25;
        let t467 = piecewise3(t2, 0.0, -81.0 * t451 * t455 + 72576.0 * t459 * t463);
        let tvsigma0 = 2.0 * t467;
        vsigma[ip] += tvsigma0;
    }
}
