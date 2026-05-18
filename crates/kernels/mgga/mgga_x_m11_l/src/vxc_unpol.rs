//! MGGA_X_M11_L vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_m11_l.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_m11_l_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_b_9: f64,
    param_b_10: f64,
    param_b_11: f64,
    param_c_0: f64,
    param_c_1: f64,
    param_c_2: f64,
    param_c_3: f64,
    param_c_4: f64,
    param_c_5: f64,
    param_c_6: f64,
    param_c_7: f64,
    param_c_8: f64,
    param_c_9: f64,
    param_c_10: f64,
    param_c_11: f64,
    param_d_0: f64,
    param_d_1: f64,
    param_d_2: f64,
    param_d_3: f64,
    param_d_4: f64,
    param_d_5: f64,
    param_d_6: f64,
    param_d_7: f64,
    param_d_8: f64,
    param_d_9: f64,
    param_d_10: f64,
    param_d_11: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t13 = t12 <= zeta_threshold;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t16 = pow_1_3::<f64>(t12);
        let t18 = piecewise3::<f64>(t13, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3::<f64>(rho[ip]);
        let t20 = t18 * t19;
        let t21 = pow_1_3::<f64>(9.0);
        let t22 = t21 * t21;
        let t24 = pow_1_3::<f64>(1.0 / M_PI);
        let t25 = t24 * t24;
        let t27 = t22 * t25 * param_hyb_omega_0;
        let t30 = piecewise3::<f64>(t13, t14, t16);
        let t31 = 1.0 / t30;
        let t34 = t27 * t4 / t19 * t31 / 18.0;
        let t35 = 0.135e1 <= t34;
        let t36 = 0.135e1 < t34;
        let t37 = piecewise3::<f64>(t36, t34, 0.135e1);
        let t38 = t37 * t37;
        let t41 = t38 * t38;
        let t42 = 1.0 / t41;
        let t44 = t41 * t38;
        let t45 = 1.0 / t44;
        let t47 = t41 * t41;
        let t48 = 1.0 / t47;
        let t51 = 1.0 / t47 / t38;
        let t54 = 1.0 / t47 / t41;
        let t57 = 1.0 / t47 / t44;
        let t59 = t47 * t47;
        let t60 = 1.0 / t59;
        let t63 = piecewise3::<f64>(t36, 0.135e1, t34);
        let t64 = f64::sqrt(M_PI);
        let t65 = 1.0 / t63;
        let t67 = erf_approx::<f64>(t65 / 2.0);
        let t69 = t63 * t63;
        let t70 = 1.0 / t69;
        let t72 = f64::exp(-t70 / 4.0);
        let t73 = t72 - 1.0;
        let t76 = t72 - 3.0 / 2.0 - 2.0 * t69 * t73;
        let t79 = 2.0 * t63 * t76 + t64 * t67;
        let t83 = piecewise3::<f64>(t35, 1.0 / t38 / 36.0 - t42 / 960.0 + t45 / 26880.0 - t48 / 829440.0 + t51 / 28385280.0 - t54 / 0.107347968e10 + t57 / 0.445906944e11 - t60 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t63 * t79);
        let t84 = M_CBRT6;
        let t85 = M_PI * M_PI;
        let t86 = pow_1_3::<f64>(t85);
        let t87 = t86 * t86;
        let t88 = 1.0 / t87;
        let t89 = t84 * t88;
        let t90 = M_CBRT2;
        let t91 = t90 * t90;
        let t92 = sigma[ip] * t91;
        let t93 = rho[ip] * rho[ip];
        let t94 = t19 * t19;
        let t96 = 1.0 / t94 / t93;
        let t98 = t89 * t92 * t96;
        let t100 = 0.804e0 + 0.914625e-2 * t98;
        let t103 = 0.1804e1 - 0.646416e0 / t100;
        let t105 = param_a_1;
        let t106 = t84 * t84;
        let t108 = 3.0 / 10.0 * t106 * t87;
        let t109 = tau[ip] * t91;
        let t111 = 1.0 / t94 / rho[ip];
        let t112 = t109 * t111;
        let t113 = t108 - t112;
        let t114 = t105 * t113;
        let t115 = t108 + t112;
        let t116 = 1.0 / t115;
        let t118 = param_a_2;
        let t119 = t113 * t113;
        let t120 = t118 * t119;
        let t121 = t115 * t115;
        let t122 = 1.0 / t121;
        let t124 = param_a_3;
        let t125 = t119 * t113;
        let t126 = t124 * t125;
        let t127 = t121 * t115;
        let t128 = 1.0 / t127;
        let t130 = param_a_4;
        let t131 = t119 * t119;
        let t132 = t130 * t131;
        let t133 = t121 * t121;
        let t134 = 1.0 / t133;
        let t136 = param_a_5;
        let t137 = t131 * t113;
        let t138 = t136 * t137;
        let t139 = t133 * t115;
        let t140 = 1.0 / t139;
        let t142 = param_a_6;
        let t143 = t131 * t119;
        let t144 = t142 * t143;
        let t145 = t133 * t121;
        let t146 = 1.0 / t145;
        let t148 = param_a_7;
        let t149 = t131 * t125;
        let t150 = t148 * t149;
        let t151 = t133 * t127;
        let t152 = 1.0 / t151;
        let t154 = param_a_8;
        let t155 = t131 * t131;
        let t156 = t154 * t155;
        let t157 = t133 * t133;
        let t158 = 1.0 / t157;
        let t160 = param_a_9;
        let t161 = t155 * t113;
        let t162 = t160 * t161;
        let t164 = 1.0 / t157 / t115;
        let t166 = param_a_10;
        let t167 = t155 * t119;
        let t168 = t166 * t167;
        let t170 = 1.0 / t157 / t121;
        let t172 = param_a_11;
        let t173 = t155 * t125;
        let t174 = t172 * t173;
        let t176 = 1.0 / t157 / t127;
        let t178 = t114 * t116 + t120 * t122 + t126 * t128 + t132 * t134 + t138 * t140 + t144 * t146 + t150 * t152 + t156 * t158 + t162 * t164 + t168 * t170 + t174 * t176 + param_a_0;
        let t181 = f64::exp(-0.93189002206715572255e-2 * t98);
        let t183 = 0.1552e1 - 0.552e0 * t181;
        let t185 = param_b_1;
        let t186 = t185 * t113;
        let t188 = param_b_2;
        let t189 = t188 * t119;
        let t191 = param_b_3;
        let t192 = t191 * t125;
        let t194 = param_b_4;
        let t195 = t194 * t131;
        let t197 = param_b_5;
        let t198 = t197 * t137;
        let t200 = param_b_6;
        let t201 = t200 * t143;
        let t203 = param_b_7;
        let t204 = t203 * t149;
        let t206 = param_b_8;
        let t207 = t206 * t155;
        let t209 = param_b_9;
        let t210 = t209 * t161;
        let t212 = param_b_10;
        let t213 = t212 * t167;
        let t215 = param_b_11;
        let t216 = t215 * t173;
        let t218 = t186 * t116 + t189 * t122 + t192 * t128 + t195 * t134 + t198 * t140 + t201 * t146 + t204 * t152 + t207 * t158 + t210 * t164 + t213 * t170 + t216 * t176 + param_b_0;
        let t220 = t103 * t178 + t183 * t218;
        let t222 = 1.0 - t83;
        let t224 = param_c_1;
        let t225 = t224 * t113;
        let t227 = param_c_2;
        let t228 = t227 * t119;
        let t230 = param_c_3;
        let t231 = t230 * t125;
        let t233 = param_c_4;
        let t234 = t233 * t131;
        let t236 = param_c_5;
        let t237 = t236 * t137;
        let t239 = param_c_6;
        let t240 = t239 * t143;
        let t242 = param_c_7;
        let t243 = t242 * t149;
        let t245 = param_c_8;
        let t246 = t245 * t155;
        let t248 = param_c_9;
        let t249 = t248 * t161;
        let t251 = param_c_10;
        let t252 = t251 * t167;
        let t254 = param_c_11;
        let t255 = t254 * t173;
        let t257 = t225 * t116 + t228 * t122 + t231 * t128 + t234 * t134 + t237 * t140 + t240 * t146 + t243 * t152 + t246 * t158 + t249 * t164 + t252 * t170 + t255 * t176 + param_c_0;
        let t260 = param_d_1;
        let t261 = t260 * t113;
        let t263 = param_d_2;
        let t264 = t263 * t119;
        let t266 = param_d_3;
        let t267 = t266 * t125;
        let t269 = param_d_4;
        let t270 = t269 * t131;
        let t272 = param_d_5;
        let t273 = t272 * t137;
        let t275 = param_d_6;
        let t276 = t275 * t143;
        let t278 = param_d_7;
        let t279 = t278 * t149;
        let t281 = param_d_8;
        let t282 = t281 * t155;
        let t284 = param_d_9;
        let t285 = t284 * t161;
        let t287 = param_d_10;
        let t288 = t287 * t167;
        let t290 = param_d_11;
        let t291 = t290 * t173;
        let t293 = t261 * t116 + t264 * t122 + t267 * t128 + t270 * t134 + t273 * t140 + t276 * t146 + t279 * t152 + t282 * t158 + t285 * t164 + t288 * t170 + t291 * t176 + param_d_0;
        let t295 = t103 * t257 + t183 * t293;
        let t297 = t83 * t220 + t222 * t295;
        let t301 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t297);
        let tzk0 = 2.0 * t301;
        zk[ip] += tzk0;
        let t303 = t18 / t94;
        let t307 = t38 * t37;
        let t308 = 1.0 / t307;
        let t314 = t27 * t4 / t19 / rho[ip] * t31 / 54.0;
        let t315 = piecewise3::<f64>(t36, -t314, 0.0);
        let t318 = t41 * t37;
        let t319 = 1.0 / t318;
        let t322 = t41 * t307;
        let t323 = 1.0 / t322;
        let t327 = 1.0 / t47 / t37;
        let t331 = 1.0 / t47 / t307;
        let t335 = 1.0 / t47 / t318;
        let t339 = 1.0 / t47 / t322;
        let t343 = 1.0 / t59 / t37;
        let t347 = piecewise3::<f64>(t36, 0.0, -t314);
        let t349 = t72 * t70;
        let t353 = t69 * t63;
        let t354 = 1.0 / t353;
        let t358 = t63 * t73;
        let t363 = t354 * t347 * t72 / 2.0 - 4.0 * t358 * t347 - t65 * t347 * t72;
        let t366 = -t349 * t347 + 2.0 * t347 * t76 + 2.0 * t63 * t363;
        let t370 = piecewise3::<f64>(t35, -t308 * t315 / 18.0 + t319 * t315 / 240.0 - t323 * t315 / 4480.0 + t327 * t315 / 103680.0 - t331 * t315 / 2838528.0 + t335 * t315 / 89456640.0 - t339 * t315 / 0.31850496e10 + t343 * t315 / 0.1263403008e12, -8.0 / 3.0 * t347 * t79 - 8.0 / 3.0 * t63 * t366);
        let t372 = t100 * t100;
        let t375 = 1.0 / t372 * t84 * t88;
        let t376 = t93 * rho[ip];
        let t378 = 1.0 / t94 / t376;
        let t383 = t105 * tau[ip];
        let t384 = t91 * t96;
        let t385 = t384 * t116;
        let t388 = t114 * t122;
        let t389 = t109 * t96;
        let t392 = t118 * t113;
        let t393 = t392 * t122;
        let t396 = t120 * t128;
        let t399 = t124 * t119;
        let t400 = t399 * t128;
        let t403 = t126 * t134;
        let t406 = t130 * t125;
        let t407 = t406 * t134;
        let t410 = t132 * t140;
        let t413 = t136 * t131;
        let t414 = t413 * t140;
        let t417 = t138 * t146;
        let t420 = t142 * t137;
        let t421 = t420 * t146;
        let t424 = 5.0 / 3.0 * t383 * t385 + 5.0 / 3.0 * t388 * t389 + 10.0 / 3.0 * t393 * t389 + 10.0 / 3.0 * t396 * t389 + 5.0 * t400 * t389 + 5.0 * t403 * t389 + 20.0 / 3.0 * t407 * t389 + 20.0 / 3.0 * t410 * t389 + 25.0 / 3.0 * t414 * t389 + 25.0 / 3.0 * t417 * t389 + 10.0 * t421 * t389;
        let t425 = t144 * t152;
        let t428 = t148 * t143;
        let t429 = t428 * t152;
        let t432 = t150 * t158;
        let t435 = t154 * t149;
        let t436 = t435 * t158;
        let t439 = t156 * t164;
        let t442 = t160 * t155;
        let t443 = t442 * t164;
        let t446 = t162 * t170;
        let t449 = t166 * t161;
        let t450 = t449 * t170;
        let t453 = t168 * t176;
        let t456 = t172 * t167;
        let t457 = t456 * t176;
        let t461 = 1.0 / t157 / t133;
        let t462 = t174 * t461;
        let t465 = 10.0 * t425 * t389 + 35.0 / 3.0 * t429 * t389 + 35.0 / 3.0 * t432 * t389 + 40.0 / 3.0 * t436 * t389 + 40.0 / 3.0 * t439 * t389 + 15.0 * t443 * t389 + 15.0 * t446 * t389 + 50.0 / 3.0 * t450 * t389 + 50.0 / 3.0 * t453 * t389 + 55.0 / 3.0 * t457 * t389 + 55.0 / 3.0 * t462 * t389;
        let t466 = t424 + t465;
        let t468 = t89 * sigma[ip];
        let t469 = t91 * t378;
        let t470 = t181 * t218;
        let t474 = t185 * tau[ip];
        let t477 = t186 * t122;
        let t480 = t188 * t113;
        let t481 = t480 * t122;
        let t484 = t189 * t128;
        let t487 = t191 * t119;
        let t488 = t487 * t128;
        let t491 = t192 * t134;
        let t494 = t194 * t125;
        let t495 = t494 * t134;
        let t498 = t195 * t140;
        let t501 = t197 * t131;
        let t502 = t501 * t140;
        let t505 = t198 * t146;
        let t508 = t200 * t137;
        let t509 = t508 * t146;
        let t512 = 5.0 / 3.0 * t474 * t385 + 5.0 / 3.0 * t477 * t389 + 10.0 / 3.0 * t481 * t389 + 10.0 / 3.0 * t484 * t389 + 5.0 * t488 * t389 + 5.0 * t491 * t389 + 20.0 / 3.0 * t495 * t389 + 20.0 / 3.0 * t498 * t389 + 25.0 / 3.0 * t502 * t389 + 25.0 / 3.0 * t505 * t389 + 10.0 * t509 * t389;
        let t513 = t201 * t152;
        let t516 = t203 * t143;
        let t517 = t516 * t152;
        let t520 = t204 * t158;
        let t523 = t206 * t149;
        let t524 = t523 * t158;
        let t527 = t207 * t164;
        let t530 = t209 * t155;
        let t531 = t530 * t164;
        let t534 = t210 * t170;
        let t537 = t212 * t161;
        let t538 = t537 * t170;
        let t541 = t213 * t176;
        let t544 = t215 * t167;
        let t545 = t544 * t176;
        let t548 = t216 * t461;
        let t551 = 10.0 * t513 * t389 + 35.0 / 3.0 * t517 * t389 + 35.0 / 3.0 * t520 * t389 + 40.0 / 3.0 * t524 * t389 + 40.0 / 3.0 * t527 * t389 + 15.0 * t531 * t389 + 15.0 * t534 * t389 + 50.0 / 3.0 * t538 * t389 + 50.0 / 3.0 * t541 * t389 + 55.0 / 3.0 * t545 * t389 + 55.0 / 3.0 * t548 * t389;
        let t552 = t512 + t551;
        let t554 = -0.1576608624e-1 * t375 * t92 * t378 * t178 + t103 * t466 - 0.13717421124828532236e-1 * t468 * t469 * t470 + t183 * t552;
        let t561 = t224 * tau[ip];
        let t564 = t225 * t122;
        let t567 = t227 * t113;
        let t568 = t567 * t122;
        let t571 = t228 * t128;
        let t574 = t230 * t119;
        let t575 = t574 * t128;
        let t578 = t231 * t134;
        let t581 = t233 * t125;
        let t582 = t581 * t134;
        let t585 = t234 * t140;
        let t588 = t236 * t131;
        let t589 = t588 * t140;
        let t592 = t237 * t146;
        let t595 = t239 * t137;
        let t596 = t595 * t146;
        let t599 = 5.0 / 3.0 * t561 * t385 + 5.0 / 3.0 * t564 * t389 + 10.0 / 3.0 * t568 * t389 + 10.0 / 3.0 * t571 * t389 + 5.0 * t575 * t389 + 5.0 * t578 * t389 + 20.0 / 3.0 * t582 * t389 + 20.0 / 3.0 * t585 * t389 + 25.0 / 3.0 * t589 * t389 + 25.0 / 3.0 * t592 * t389 + 10.0 * t596 * t389;
        let t600 = t240 * t152;
        let t603 = t242 * t143;
        let t604 = t603 * t152;
        let t607 = t243 * t158;
        let t610 = t245 * t149;
        let t611 = t610 * t158;
        let t614 = t246 * t164;
        let t617 = t248 * t155;
        let t618 = t617 * t164;
        let t621 = t249 * t170;
        let t624 = t251 * t161;
        let t625 = t624 * t170;
        let t628 = t252 * t176;
        let t631 = t254 * t167;
        let t632 = t631 * t176;
        let t635 = t255 * t461;
        let t638 = 10.0 * t600 * t389 + 35.0 / 3.0 * t604 * t389 + 35.0 / 3.0 * t607 * t389 + 40.0 / 3.0 * t611 * t389 + 40.0 / 3.0 * t614 * t389 + 15.0 * t618 * t389 + 15.0 * t621 * t389 + 50.0 / 3.0 * t625 * t389 + 50.0 / 3.0 * t628 * t389 + 55.0 / 3.0 * t632 * t389 + 55.0 / 3.0 * t635 * t389;
        let t639 = t599 + t638;
        let t641 = t181 * t293;
        let t645 = t260 * tau[ip];
        let t648 = t261 * t122;
        let t651 = t263 * t113;
        let t652 = t651 * t122;
        let t655 = t264 * t128;
        let t658 = t266 * t119;
        let t659 = t658 * t128;
        let t662 = t267 * t134;
        let t665 = t269 * t125;
        let t666 = t665 * t134;
        let t669 = t270 * t140;
        let t672 = t272 * t131;
        let t673 = t672 * t140;
        let t676 = t273 * t146;
        let t679 = t275 * t137;
        let t680 = t679 * t146;
        let t683 = 5.0 / 3.0 * t645 * t385 + 5.0 / 3.0 * t648 * t389 + 10.0 / 3.0 * t652 * t389 + 10.0 / 3.0 * t655 * t389 + 5.0 * t659 * t389 + 5.0 * t662 * t389 + 20.0 / 3.0 * t666 * t389 + 20.0 / 3.0 * t669 * t389 + 25.0 / 3.0 * t673 * t389 + 25.0 / 3.0 * t676 * t389 + 10.0 * t680 * t389;
        let t684 = t276 * t152;
        let t687 = t278 * t143;
        let t688 = t687 * t152;
        let t691 = t279 * t158;
        let t694 = t281 * t149;
        let t695 = t694 * t158;
        let t698 = t282 * t164;
        let t701 = t284 * t155;
        let t702 = t701 * t164;
        let t705 = t285 * t170;
        let t708 = t287 * t161;
        let t709 = t708 * t170;
        let t712 = t288 * t176;
        let t715 = t290 * t167;
        let t716 = t715 * t176;
        let t719 = t291 * t461;
        let t722 = 10.0 * t684 * t389 + 35.0 / 3.0 * t688 * t389 + 35.0 / 3.0 * t691 * t389 + 40.0 / 3.0 * t695 * t389 + 40.0 / 3.0 * t698 * t389 + 15.0 * t702 * t389 + 15.0 * t705 * t389 + 50.0 / 3.0 * t709 * t389 + 50.0 / 3.0 * t712 * t389 + 55.0 / 3.0 * t716 * t389 + 55.0 / 3.0 * t719 * t389;
        let t723 = t683 + t722;
        let t725 = -0.1576608624e-1 * t375 * t92 * t378 * t257 + t103 * t639 - 0.13717421124828532236e-1 * t468 * t469 * t641 + t183 * t723;
        let t727 = t370 * t220 + t222 * t725 - t370 * t295 + t83 * t554;
        let t732 = piecewise3::<f64>(t3, 0.0, -t7 * t303 * t297 / 8.0 - 3.0 / 8.0 * t7 * t20 * t727);
        let tvrho0 = 2.0 * rho[ip] * t732 + 2.0 * t301;
        vrho[ip] += tvrho0;
        let t738 = t89 * t91;
        let t739 = t96 * t181;
        let t743 = 0.591228234e-2 * t375 * t384 * t178 + 0.51440329218106995885e-2 * t738 * t739 * t218;
        let t751 = 0.591228234e-2 * t375 * t384 * t257 + 0.51440329218106995885e-2 * t738 * t739 * t293;
        let t753 = t222 * t751 + t83 * t743;
        let t757 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t753);
        let tvsigma0 = 2.0 * rho[ip] * t757;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t759 = t105 * t91;
        let t760 = t111 * t116;
        let t762 = t122 * t91;
        let t763 = t762 * t111;
        let t767 = t128 * t91;
        let t768 = t767 * t111;
        let t773 = t134 * t91;
        let t774 = t773 * t111;
        let t779 = t140 * t91;
        let t780 = t779 * t111;
        let t785 = t146 * t91;
        let t786 = t785 * t111;
        let t791 = -t114 * t763 - 2.0 * t120 * t768 - 3.0 * t126 * t774 - 4.0 * t132 * t780 - 5.0 * t138 * t786 - 2.0 * t392 * t763 - 3.0 * t399 * t768 - 4.0 * t406 * t774 - 5.0 * t413 * t780 - 6.0 * t420 * t786 - t759 * t760;
        let t792 = t152 * t91;
        let t793 = t792 * t111;
        let t798 = t158 * t91;
        let t799 = t798 * t111;
        let t804 = t164 * t91;
        let t805 = t804 * t111;
        let t810 = t170 * t91;
        let t811 = t810 * t111;
        let t816 = t176 * t91;
        let t817 = t816 * t111;
        let t822 = t461 * t91;
        let t823 = t822 * t111;
        let t826 = -6.0 * t144 * t793 - 7.0 * t150 * t799 - 8.0 * t156 * t805 - 9.0 * t162 * t811 - 10.0 * t168 * t817 - 11.0 * t174 * t823 - 7.0 * t428 * t793 - 8.0 * t435 * t799 - 9.0 * t442 * t805 - 10.0 * t449 * t811 - 11.0 * t456 * t817;
        let t827 = t791 + t826;
        let t829 = t185 * t91;
        let t850 = -t186 * t763 - 2.0 * t189 * t768 - 3.0 * t192 * t774 - 4.0 * t195 * t780 - 5.0 * t198 * t786 - 2.0 * t480 * t763 - 3.0 * t487 * t768 - 4.0 * t494 * t774 - 5.0 * t501 * t780 - 6.0 * t508 * t786 - t829 * t760;
        let t873 = -6.0 * t201 * t793 - 7.0 * t204 * t799 - 8.0 * t207 * t805 - 9.0 * t210 * t811 - 10.0 * t213 * t817 - 11.0 * t216 * t823 - 7.0 * t516 * t793 - 8.0 * t523 * t799 - 9.0 * t530 * t805 - 10.0 * t537 * t811 - 11.0 * t544 * t817;
        let t874 = t850 + t873;
        let t876 = t103 * t827 + t183 * t874;
        let t878 = t224 * t91;
        let t899 = -t225 * t763 - 2.0 * t228 * t768 - 3.0 * t231 * t774 - 4.0 * t234 * t780 - 5.0 * t237 * t786 - 2.0 * t567 * t763 - 3.0 * t574 * t768 - 4.0 * t581 * t774 - 5.0 * t588 * t780 - 6.0 * t595 * t786 - t878 * t760;
        let t922 = -6.0 * t240 * t793 - 7.0 * t243 * t799 - 8.0 * t246 * t805 - 9.0 * t249 * t811 - 10.0 * t252 * t817 - 11.0 * t255 * t823 - 7.0 * t603 * t793 - 8.0 * t610 * t799 - 9.0 * t617 * t805 - 10.0 * t624 * t811 - 11.0 * t631 * t817;
        let t923 = t899 + t922;
        let t925 = t260 * t91;
        let t946 = -t261 * t763 - 2.0 * t264 * t768 - 3.0 * t267 * t774 - 4.0 * t270 * t780 - 5.0 * t273 * t786 - 2.0 * t651 * t763 - 3.0 * t658 * t768 - 4.0 * t665 * t774 - 5.0 * t672 * t780 - 6.0 * t679 * t786 - t925 * t760;
        let t969 = -6.0 * t276 * t793 - 7.0 * t279 * t799 - 8.0 * t282 * t805 - 9.0 * t285 * t811 - 10.0 * t288 * t817 - 11.0 * t291 * t823 - 7.0 * t687 * t793 - 8.0 * t694 * t799 - 9.0 * t701 * t805 - 10.0 * t708 * t811 - 11.0 * t715 * t817;
        let t970 = t946 + t969;
        let t972 = t103 * t923 + t183 * t970;
        let t974 = t222 * t972 + t83 * t876;
        let t978 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t974);
        let tvtau0 = 2.0 * rho[ip] * t978;
        vtau[ip] += tvtau0;
    }
}
