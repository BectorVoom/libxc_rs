//! GGA_C_OP_G96 fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_g96.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_op_g96_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = 1.0 <= zeta_threshold;
        let t4 = t1 || rho[ip] / 2.0 <= dens_threshold;
        let t5 = zeta_threshold - 1.0;
        let t6 = -t5;
        let t7 = piecewise5(t1, t5, t1, t6, 0.0);
        let t8 = t7 * t7;
        let t9 = 1.0 - t8;
        let t10 = t9 * rho[ip];
        let t11 = 1.0 + t7;
        let t14 = t11 * rho[ip] / 2.0 <= dens_threshold;
        let t15 = M_CBRT3;
        let t16 = t15 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t19 = 1.0 / t18;
        let t20 = t16 * t19;
        let t21 = M_CBRT4;
        let t22 = t20 * t21;
        let t23 = M_CBRT2;
        let t24 = t11 <= zeta_threshold;
        let t25 = 1.0 - t7;
        let t26 = t25 <= zeta_threshold;
        let t27 = piecewise5(t24, t5, t26, t6, t7);
        let t28 = 1.0 + t27;
        let t29 = t28 * rho[ip];
        let t30 = pow_1_3(t29);
        let t31 = 1.0 / t30;
        let t33 = f64::sqrt(sigma[ip]);
        let t34 = t33 * t23;
        let t35 = pow_1_3(rho[ip]);
        let t37 = 1.0 / t35 / rho[ip];
        let t38 = t34 * t37;
        let t39 = f64::sqrt(t38);
        let t40 = t39 * t38;
        let t44 = 1.0 + 2.0 / 1233.0 * t20 * t21 * t40;
        let t45 = 1.0 / t44;
        let t49 = piecewise3(t14, 0.0, t22 * t23 * t31 * t45 / 9.0);
        let t53 = t25 * rho[ip] / 2.0 <= dens_threshold;
        let t54 = piecewise5(t26, t5, t24, t6, -t7);
        let t55 = 1.0 + t54;
        let t56 = t55 * rho[ip];
        let t57 = pow_1_3(t56);
        let t58 = 1.0 / t57;
        let t63 = piecewise3(t53, 0.0, t22 * t23 * t58 * t45 / 9.0);
        let t64 = t49 + t63;
        let t65 = t64 == 0.0;
        let t66 = piecewise3(t65, f64::EPSILON, t64);
        let t69 = 0.359628532e1 / t66 + 0.5764e0;
        let t70 = t66 * t66;
        let t71 = t70 * t70;
        let t72 = 1.0 / t71;
        let t74 = t70 * t66;
        let t75 = 1.0 / t74;
        let t77 = 1.0 / t70;
        let t79 = 0.312207199195441936e2 * t72 + 0.149037398922132448e2 * t75 + 0.1778517305052e1 * t77;
        let t80 = 1.0 / t79;
        let tzk0 = piecewise3(t4, 0.0, -0.25e0 * t10 * t69 * t80);
        zk[ip] += tzk0;
        let t84 = t9 * t69;
        let t88 = 1.0 / t30 / t29;
        let t94 = t18 * t18;
        let t95 = 1.0 / t94;
        let t96 = t15 * t95;
        let t97 = t21 * t21;
        let t98 = t23 * t23;
        let t99 = t97 * t98;
        let t100 = t96 * t99;
        let t101 = t44 * t44;
        let t102 = 1.0 / t101;
        let t103 = t31 * t102;
        let t104 = t39 * t33;
        let t105 = rho[ip] * rho[ip];
        let t107 = 1.0 / t35 / t105;
        let t108 = t104 * t107;
        let t113 = piecewise3(t14, 0.0, -t22 * t23 * t88 * t45 * t28 / 27.0 + 4.0 / 3699.0 * t100 * t103 * t108);
        let t115 = 1.0 / t57 / t56;
        let t121 = t58 * t102;
        let t126 = piecewise3(t53, 0.0, -t22 * t23 * t115 * t45 * t55 / 27.0 + 4.0 / 3699.0 * t100 * t121 * t108);
        let t128 = piecewise3(t65, 0.0, t113 + t126);
        let t133 = t79 * t79;
        let t134 = 1.0 / t133;
        let t135 = t69 * t134;
        let t137 = 1.0 / t71 / t66;
        let t138 = t137 * t128;
        let t140 = t72 * t128;
        let t144 = -0.1248828796781767744e3 * t138 - 0.447112196766397344e2 * t140 - 0.3557034610104e1 * t75 * t128;
        let t149 = piecewise3(t4, 0.0, -0.25e0 * t84 * t80 + 0.89907133e0 * t10 * t77 * t128 * t80 + 0.25e0 * t10 * t135 * t144);
        let tvrho0 = rho[ip] * t149 + tzk0;
        vrho[ip] += tvrho0;
        let t151 = 1.0 / t33;
        let t152 = t39 * t151;
        let t153 = t152 * t37;
        let t157 = piecewise3(t14, 0.0, -t100 * t103 * t153 / 2466.0);
        let t161 = piecewise3(t53, 0.0, -t100 * t121 * t153 / 2466.0);
        let t163 = piecewise3(t65, 0.0, t157 + t161);
        let t168 = t137 * t163;
        let t170 = t72 * t163;
        let t172 = t75 * t163;
        let t174 = -0.1248828796781767744e3 * t168 - 0.447112196766397344e2 * t170 - 0.3557034610104e1 * t172;
        let t179 = piecewise3(t4, 0.0, 0.89907133e0 * t10 * t77 * t163 * t80 + 0.25e0 * t10 * t135 * t174);
        let tvsigma0 = rho[ip] * t179;
        vsigma[ip] += tvsigma0;
        let t181 = t9 * t77;
        let t182 = t128 * t80;
        let t188 = t128 * t128;
        let t193 = t28 * t28;
        let t196 = 1.0 / t30 / t193 / t105;
        let t203 = t96 * t99 * t88;
        let t204 = t102 * t28;
        let t208 = M_PI * t31;
        let t210 = 1.0 / t101 / t44;
        let t211 = t208 * t210;
        let t212 = t33 * sigma[ip];
        let t213 = t212 * t23;
        let t214 = t105 * t105;
        let t215 = t214 * t105;
        let t216 = 1.0 / t215;
        let t217 = t213 * t216;
        let t221 = t96 * t97 * t31;
        let t222 = 1.0 / t39;
        let t223 = t102 * t222;
        let t224 = t35 * t35;
        let t226 = 1.0 / t224 / t214;
        let t228 = t223 * sigma[ip] * t226;
        let t231 = t105 * rho[ip];
        let t233 = 1.0 / t35 / t231;
        let t234 = t104 * t233;
        let t239 = piecewise3(t14, 0.0, 4.0 / 81.0 * t22 * t23 * t196 * t45 * t193 - 8.0 / 11097.0 * t203 * t204 * t108 + 256.0 / 1520289.0 * t211 * t217 - 16.0 / 11097.0 * t221 * t228 - 28.0 / 11097.0 * t100 * t103 * t234);
        let t240 = t55 * t55;
        let t243 = 1.0 / t57 / t240 / t105;
        let t250 = t96 * t99 * t115;
        let t251 = t102 * t55;
        let t255 = M_PI * t58;
        let t256 = t255 * t210;
        let t260 = t96 * t97 * t58;
        let t267 = piecewise3(t53, 0.0, 4.0 / 81.0 * t22 * t23 * t243 * t45 * t240 - 8.0 / 11097.0 * t250 * t251 * t108 + 256.0 / 1520289.0 * t256 * t217 - 16.0 / 11097.0 * t260 * t228 - 28.0 / 11097.0 * t100 * t121 * t234);
        let t269 = piecewise3(t65, 0.0, t239 + t267);
        let t274 = t10 * t77;
        let t275 = t128 * t134;
        let t276 = t275 * t144;
        let t280 = 1.0 / t133 / t79;
        let t281 = t69 * t280;
        let t282 = t144 * t144;
        let t287 = 1.0 / t71 / t70;
        let t288 = t287 * t188;
        let t292 = t137 * t188;
        let t300 = 0.624414398390883872e3 * t288 - 0.1248828796781767744e3 * t137 * t269 + 0.1788448787065589376e3 * t292 - 0.447112196766397344e2 * t72 * t269 + 0.10671103830312e2 * t72 * t188 - 0.3557034610104e1 * t75 * t269;
        let t305 = piecewise3(t4, 0.0, 0.179814266e1 * t181 * t182 + 0.5e0 * t84 * t134 * t144 - 0.179814266e1 * t10 * t75 * t188 * t80 + 0.89907133e0 * t10 * t77 * t269 * t80 - 0.179814266e1 * t274 * t276 - 0.5e0 * t10 * t281 * t282 + 0.25e0 * t10 * t135 * t300);
        let tv2rho20 = rho[ip] * t305 + 2.0 * t149;
        v2rho2[ip] += tv2rho20;
        let t307 = t163 * t80;
        let t310 = t10 * t75;
        let t311 = t307 * t128;
        let t314 = t102 * t39;
        let t315 = t151 * t37;
        let t320 = t214 * rho[ip];
        let t321 = 1.0 / t320;
        let t322 = t34 * t321;
        let t325 = t96 * t97;
        let t327 = 1.0 / t224 / t231;
        let t328 = t222 * t327;
        let t332 = t152 * t107;
        let t337 = piecewise3(t14, 0.0, t203 * t314 * t315 * t28 / 7398.0 - 32.0 / 506763.0 * t211 * t322 + 2.0 / 3699.0 * t325 * t103 * t328 + 2.0 / 3699.0 * t100 * t103 * t332);
        let t351 = piecewise3(t53, 0.0, t250 * t314 * t315 * t55 / 7398.0 - 32.0 / 506763.0 * t256 * t322 + 2.0 / 3699.0 * t325 * t121 * t328 + 2.0 / 3699.0 * t100 * t121 * t332);
        let t353 = piecewise3(t65, 0.0, t337 + t351);
        let t358 = t163 * t134;
        let t359 = t358 * t144;
        let t365 = t275 * t174;
        let t368 = t10 * t69;
        let t369 = t280 * t174;
        let t370 = t369 * t144;
        let t373 = t287 * t163;
        let t376 = t137 * t353;
        let t380 = t72 * t353;
        let t386 = 0.624414398390883872e3 * t373 * t128 - 0.1248828796781767744e3 * t376 + 0.1788448787065589376e3 * t168 * t128 - 0.447112196766397344e2 * t380 + 0.10671103830312e2 * t170 * t128 - 0.3557034610104e1 * t75 * t353;
        let t391 = piecewise3(t4, 0.0, 0.89907133e0 * t181 * t307 - 0.179814266e1 * t310 * t311 + 0.89907133e0 * t10 * t77 * t353 * t80 - 0.89907133e0 * t274 * t359 + 0.25e0 * t84 * t134 * t174 - 0.89907133e0 * t274 * t365 - 0.5e0 * t368 * t370 + 0.25e0 * t10 * t135 * t386);
        let tv2rhosigma0 = rho[ip] * t391 + t179;
        v2rhosigma[ip] += tv2rhosigma0;
        let t393 = t163 * t163;
        let t398 = t151 * t23;
        let t399 = 1.0 / t214;
        let t400 = t398 * t399;
        let t403 = 1.0 / sigma[ip];
        let t405 = 1.0 / t224 / t105;
        let t406 = t403 * t405;
        let t407 = t223 * t406;
        let t410 = 1.0 / t212;
        let t411 = t39 * t410;
        let t412 = t411 * t37;
        let t417 = piecewise3(t14, 0.0, 4.0 / 168921.0 * t211 * t400 - t221 * t407 / 4932.0 + t100 * t103 * t412 / 4932.0);
        let t426 = piecewise3(t53, 0.0, 4.0 / 168921.0 * t256 * t400 - t260 * t407 / 4932.0 + t100 * t121 * t412 / 4932.0);
        let t428 = piecewise3(t65, 0.0, t417 + t426);
        let t433 = t358 * t174;
        let t436 = t174 * t174;
        let t440 = t287 * t393;
        let t442 = t137 * t428;
        let t444 = t137 * t393;
        let t446 = t72 * t428;
        let t452 = 0.624414398390883872e3 * t440 - 0.1248828796781767744e3 * t442 + 0.1788448787065589376e3 * t444 - 0.447112196766397344e2 * t446 + 0.10671103830312e2 * t72 * t393 - 0.3557034610104e1 * t75 * t428;
        let t457 = piecewise3(t4, 0.0, -0.179814266e1 * t10 * t75 * t393 * t80 + 0.89907133e0 * t10 * t77 * t428 * t80 - 0.179814266e1 * t274 * t433 - 0.5e0 * t10 * t281 * t436 + 0.25e0 * t10 * t135 * t452);
        let tv2sigma20 = rho[ip] * t457;
        v2sigma2[ip] += tv2sigma20;
    }
}
